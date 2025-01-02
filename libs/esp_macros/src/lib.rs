use proc_macro::TokenStream;
use quote::{quote, ToTokens};

mod features;

#[doc(hidden)]
#[proc_macro_attribute]
pub fn esp_meta(args: TokenStream, input: TokenStream) -> TokenStream {
    #[allow(unused_mut)]
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);

    // todo parse this properly
    if args.to_string().is_empty() {
        impl_meta(&mut input);
    }

    #[cfg(feature = "serde")]
    {
        features::serde::impl_serialize_deserialize(&mut input);
    }

    let output = quote! {
        #input
    };

    output.into()
}

#[doc(hidden)]
#[proc_macro_derive(TES3Object, attributes(tag))]
pub fn derive_tes3object(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let variants = match &input.data {
        syn::Data::Enum(e) if input.ident == "TES3Object" => &e.variants,
        _ => panic!("derive(TES3Object) must be on the TES3Object enum"),
    };

    let idents = parse_variant_idents(variants);
    let tags = parse_variant_tags(variants);

    let impl_variants = tes3object_variant_impls(&idents, &tags);
    let impl_object = tes3object_inherent_impls(&idents);

    let output = quote! {
        const _: () = {
            #impl_variants
            #impl_object
        };
    };

    output.into()
}

fn tes3object_variant_impls(idents: &[syn::Ident], tags: &[syn::LitStr]) -> impl ToTokens {
    let tags_bytes = tags //
        .iter()
        .map(|tag| syn::LitByteStr::new(tag.value().as_bytes(), tag.span()));

    let idents_str = idents //
        .iter()
        .map(|ident| syn::LitStr::new(&ident.to_string(), ident.span()));

    quote! {
        #(
            #[doc(hidden)]
            impl #idents {
                pub const TAG: &'static [u8; 4] = #tags_bytes;
                pub const TAG_STR: &'static str = #tags;
                pub const TYPE_NAME: &'static str = #idents_str;
            }

            impl TryFrom<TES3Object> for #idents {
                type Error = ();
                fn try_from(value: TES3Object) -> Result<Self, Self::Error> {
                    match value {
                        TES3Object::#idents(inner) => Ok(inner),
                        _ => Err(())
                    }
                }
            }

            impl<'a> TryFrom<&'a TES3Object> for &'a #idents {
                type Error = ();
                fn try_from(value: &'a TES3Object) -> Result<Self, Self::Error> {
                    match value {
                        TES3Object::#idents(inner) => Ok(inner),
                        _ => Err(())
                    }
                }
            }

            impl<'a> TryFrom<&'a mut TES3Object> for &'a mut #idents {
                type Error = ();
                fn try_from(value: &'a mut TES3Object) -> Result<Self, Self::Error> {
                    match value {
                        TES3Object::#idents(inner) => Ok(inner),
                        _ => Err(())
                    }
                }
            }
        )*
    }
}

fn tes3object_inherent_impls(idents: &[syn::Ident]) -> impl ToTokens {
    quote! {
        use bytes_io::*;
        #[cfg(feature = "egui")]
        use crate::editor::Editor;

        impl Load for TES3Object {
            fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                let tag = stream.load()?;
                stream.skip(8)?; // skip size/padding

                match &tag {
                    #(
                        #idents::TAG => Ok(Self::#idents(stream.load()?)),
                    )*
                    _ => Reader::error(format!("Unexpected Tag: {}", tag.to_str_lossy()))?,
                }
            }
        }

        impl Save for TES3Object {
            fn save(&self, stream: &mut Writer) -> io::Result<()> {
                let start_pos = stream.cursor.position();

                // buffer for tag/size/padding
                stream.save(&[0u32; 3])?;

                // save object & get tag
                let tag = match self {
                    #(
                        TES3Object::#idents(obj) => { stream.save(obj)?; obj.tag() }
                    )*
                };

                // calculate object size
                let final_pos = stream.cursor.position();
                let size = (final_pos - start_pos - 16) as u32;

                // update the tag & size
                stream.cursor.set_position(start_pos);
                stream.save(tag)?;
                stream.save(&size)?;
                stream.cursor.set_position(final_pos);

                Ok(())
            }
        }

        #[cfg(feature = "egui")]
        impl Editor for TES3Object {
            fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
                match self {
                    #(
                        TES3Object::#idents(obj) => obj.add_editor(ui, name),
                    )*
                }
            }
            fn to_json(&self) -> String {
                match self {
                    #(
                        TES3Object::#idents(obj) => obj.to_json(),
                    )*
                }
            }
            fn get_editor_list(&mut self) -> Option<Vec<(&str, &mut dyn editor::Editor)>> {
                match self {
                    #(
                        TES3Object::#idents(obj) => obj.get_editor_list(),
                    )*
                }
            }
        }
    }
}

fn parse_variant_tags<'a, I>(variants: I) -> Vec<syn::LitStr>
where
    I: IntoIterator<Item = &'a syn::Variant>,
{
    variants.into_iter().map(|v| v.attrs[0].parse_args().unwrap()).collect()
}

fn parse_variant_idents<'a, I>(variants: I) -> Vec<syn::Ident>
where
    I: IntoIterator<Item = &'a syn::Variant>,
{
    variants.into_iter().map(|v| v.ident.clone()).collect()
}

fn impl_meta(input: &mut syn::DeriveInput) {
    input.attrs.push(syn::parse_quote! {
        #[cfg_attr(feature = "egui", derive(Editor))]
    });
}
