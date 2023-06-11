use proc_macro::TokenStream;
use quote::quote;

#[doc(hidden)]
#[proc_macro_derive(LoadSave)]
pub fn derive_load_save(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(_) => impl_load_save_for_struct(&input),
        syn::Data::Enum(_) => impl_load_save_for_enum(&input),
        syn::Data::Union(_) => unimplemented!(),
    }
}

fn impl_load_save_for_struct(input: &syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let fields: Vec<_> = get_struct_fields(&input.data).collect();
    if fields.is_empty() {
        impl_load_save_for_bitflags(ident).into()
    } else {
        impl_load_save_for_struct_with_named_fields(ident, &fields).into()
    }
}

fn impl_load_save_for_bitflags(ident: &syn::Ident) -> impl Into<TokenStream> {
    quote! {
        const _: () = {
            use crate::prelude::*;

            impl Load for #ident {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    Ok(Self::from_bits_retain(stream.load()?))
                }
            }

            impl Save for #ident {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.save(&self.bits())
                }
            }
        };
    }
}

fn impl_load_save_for_struct_with_named_fields(ident: &syn::Ident, fields: &[&syn::Ident]) -> impl Into<TokenStream> {
    quote! {
        const _: () = {
            use crate::prelude::*;

            impl Load for #ident {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    Ok(Self {
                        #(
                            #fields: stream.load()?,
                        )*
                    })
                }
            }

            impl Save for #ident {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    #(
                        stream.save(&self.#fields)?;
                    )*
                    Ok(())
                }
            }
        };
    }
}

fn impl_load_save_for_enum(input: &syn::DeriveInput) -> TokenStream {
    let variants = match &input.data {
        syn::Data::Enum(e) => &e.variants,
        _ => panic!("invalid input"),
    };

    let repr_ident = get_repr(&input.attrs);
    let self_ident = &input.ident;

    let variant_idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let variant_values: Vec<_> = variants.iter().map(|v| &v.discriminant.as_ref().unwrap().1).collect();
    let variant_strings: Vec<_> = variant_idents.iter().map(|id| get_literal_str(id)).collect();

    let output = quote! {
        const _: () = {
            use bytes_io::*;

            unsafe impl AsRepr for #self_ident {
                type Repr = #repr_ident;
            }

            impl TryFrom<#repr_ident> for #self_ident {
                type Error = ();
                fn try_from(value: #repr_ident) -> Result<Self, Self::Error> {
                    match value {
                        #(
                            #variant_values => Ok(Self::#variant_idents),
                        )*
                        _ => Err(()),
                    }
                }
            }

            impl Load for #self_ident {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let value: #repr_ident = stream.load()?;
                    TryFrom::try_from(value).map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid enum variant")
                    })
                }
            }

            impl Save for #self_ident {
                #[allow(trivial_numeric_casts, clippy::unnecessary_cast)]
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    match self {
                        #(
                            Self::#variant_idents => stream.save(&(#variant_values as #repr_ident)),
                        )*
                    }
                }
            }

            impl #self_ident {
                pub const fn display(&self) -> &'static str {
                    match self {
                        #(
                            Self::#variant_idents => #variant_strings,
                        )*
                    }
                }
            }
        };
    };

    output.into()
}

fn get_literal_str(id: &syn::Ident) -> syn::LitStr {
    syn::LitStr::new(&id.to_string(), id.span())
}

fn get_struct_fields(data: &syn::Data) -> impl Iterator<Item = &syn::Ident> {
    let fields = match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(f),
            ..
        }) => Some(f.named.iter().filter_map(|f| f.ident.as_ref())),
        _ => None,
    };
    fields.into_iter().flatten()
}

fn get_repr(attributes: &[syn::Attribute]) -> syn::Ident {
    attributes
        .iter()
        .find_map(|attr| {
            let outer_ident = attr.path().get_ident()?;
            let inner_ident: syn::Ident = attr.parse_args().ok()?;
            (*outer_ident == "repr").then_some(inner_ident)
        })
        .expect("Could not find repr.")
}

///////////////////////////////////////////////////////////
/// egui macros

// todo: write a proper editor override
#[doc(hidden)]
#[proc_macro_derive(Editor, attributes(editor))]
pub fn derive_editor(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(_) => impl_editor_for_struct(&input),
        syn::Data::Enum(_) => impl_editor_for_enum(&input),
        syn::Data::Union(_) => unimplemented!(),
    }
}

fn impl_editor_for_struct(input: &syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let fields: Vec<_> = get_struct_fields(&input.data).collect();

    if fields.is_empty() {
        //todo impl_editor_for_bitflags(ident).into()
        let output = quote! {
            #input
        };
        output.into()
    } else {
        let field_names: Vec<_> = fields.iter().map(|id| get_literal_str(id)).collect();
        let ident_name = get_literal_str(ident);

        impl_editor_for_struct_with_named_fields(ident, ident_name, &fields, &field_names).into()
    }
}

fn impl_editor_for_struct_with_named_fields(
    ident: &syn::Ident,
    _ident_name: syn::LitStr,
    fields: &[&syn::Ident],
    field_names: &[syn::LitStr],
) -> impl Into<TokenStream> {
    quote! {
        const _: () = {
            use crate::prelude::*;

            impl crate::editor::Editor for #ident {
                fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
                    egui::Grid::new(name.clone()).num_columns(2).striped(true).show(ui, |ui| {
                        #(
                            ui.label(egui::RichText::new(#field_names).color(egui::Color32::LIGHT_BLUE));

                            self.#fields.add_editor(ui, format!("{}.{}", name, #field_names));
                            ui.end_row();
                        )*
                    });
                }
            }
        };
    }
}

fn impl_editor_for_enum(input: &syn::DeriveInput) -> TokenStream {
    let variants = match &input.data {
        syn::Data::Enum(e) => &e.variants,
        _ => panic!("invalid input"),
    };

    let ident = &input.ident;
    let variant_idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let variant_strings: Vec<_> = variant_idents.iter().map(|id| get_literal_str(id)).collect();

    let output = quote! {
        const _: () = {
            use crate::prelude::*;
            impl crate::editor::Editor for #ident {
                fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
                    let mut selected = self.to_owned();
                    ui.push_id(name.clone(), |ui| {
                        egui::ComboBox::from_id_source(name)
                            .selected_text(format!("{:?}", selected))
                            .show_ui(ui, |ui| {
                                #(
                                    ui.selectable_value(&mut selected, #ident::#variant_idents, #variant_strings);
                                )*
                            });
                    });
                    *self = selected;
                }
            }
        };
    };

    output.into()
}
