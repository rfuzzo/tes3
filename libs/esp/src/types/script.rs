// internal imports
use crate::prelude::*;

#[esp_meta(true)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Script {
    pub flags: ObjectFlags,
    pub id: String,
    pub header: ScriptHeader,
    pub variables: Vec<u8>,
    pub bytecode: Vec<u8>,
    pub text: String,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct ScriptHeader {
    pub num_shorts: u32,
    pub num_longs: u32,
    pub num_floats: u32,
    pub bytecode_length: u32,
    pub variables_length: u32,
}

impl Load for Script {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"SCHD" => {
                    stream.expect(52u32)?;
                    this.id = stream.load::<FixedString<32>>()?.into();
                    this.header = stream.load()?;
                }
                b"SCVR" => {
                    this.variables = stream.load()?;
                }
                b"SCDT" => {
                    this.bytecode = stream.load()?;
                }
                b"SCTX" => {
                    this.text = stream.load()?;
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.flags.insert(ObjectFlags::DELETED);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Script {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // SCHD
        stream.save(b"SCHD")?;
        stream.save(&52u32)?;
        stream.save::<FixedString<32>>(self.id.as_ref())?;
        stream.save(&self.header)?;
        // SCVR
        if !self.variables.is_empty() {
            stream.save(b"SCVR")?;
            stream.save(&self.variables)?;
        }
        // SCDT
        if !self.bytecode.is_empty() {
            stream.save(b"SCDT")?;
            stream.save(&self.bytecode)?;
        }
        // SCTX
        if !self.text.is_empty() {
            stream.save(b"SCTX")?;
            stream.save(&self.text)?;
        }
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}

#[cfg(feature = "egui")]
impl crate::editor::Editor for Script {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        egui::Grid::new(name.clone()).num_columns(2).striped(true).show(ui, |ui| {
            ui.label(egui::RichText::new("flags").color(egui::Color32::LIGHT_BLUE));
            self.flags.add_editor(ui, format!("{}.flags", name));
            ui.end_row();

            ui.label(egui::RichText::new("id").color(egui::Color32::LIGHT_BLUE));
            self.id.add_editor(ui, format!("{}.id", name));
            ui.end_row();

            ui.label(egui::RichText::new("header").color(egui::Color32::LIGHT_BLUE));
            self.header.add_editor(ui, format!("{}.header", name));
            ui.end_row();

            ui.label(egui::RichText::new("variables").color(egui::Color32::LIGHT_BLUE));
            self.variables.add_editor(ui, format!("{}.variables", name));
            ui.end_row();

            ui.label(egui::RichText::new("bytecode").color(egui::Color32::LIGHT_BLUE));
            self.bytecode.add_editor(ui, format!("{}.bytecode", name));
            ui.end_row();

            // custom editor here
            ui.label(egui::RichText::new("text").color(egui::Color32::LIGHT_BLUE));
            ui.push_id(format!("{}.text", name), |ui| {
                egui::ScrollArea::vertical().min_scrolled_height(600.0).show(ui, |ui| {
                    // ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text));
                    ui.add(egui::TextEdit::multiline(&mut self.text));
                });
            });
            ui.end_row();
        });
    }
}
