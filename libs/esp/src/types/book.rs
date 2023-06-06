// internal imports
use crate::prelude::*;

#[esp_meta(true)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Book {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub enchanting: String,
    pub text: String,
    pub data: BookData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct BookData {
    pub weight: f32,
    pub value: u32,
    pub book_type: BookType,
    pub skill: SkillId,
    pub enchantment: u32,
}

impl Load for Book {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"MODL" => {
                    this.mesh = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"BKDT" => {
                    stream.expect(20u32)?;
                    this.data = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"ITEX" => {
                    this.icon = stream.load()?;
                }
                b"TEXT" => {
                    this.text = stream.load()?;
                }
                b"ENAM" => {
                    this.enchanting = stream.load()?;
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

impl Save for Book {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // MODL
        if !self.mesh.is_empty() {
            stream.save(b"MODL")?;
            stream.save(&self.mesh)?;
        }
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // BKDT
        stream.save(b"BKDT")?;
        stream.save(&20u32)?;
        stream.save(&self.data)?;
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // ITEX
        if !self.icon.is_empty() {
            stream.save(b"ITEX")?;
            stream.save(&self.icon)?;
        }
        // TEXT
        if !self.text.is_empty() {
            stream.save(b"TEXT")?;
            stream.save(&self.text)?;
        }
        // ENAM
        if !self.enchanting.is_empty() {
            stream.save(b"ENAM")?;
            stream.save(&self.enchanting)?;
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
impl crate::editor::Editor for Book {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        egui::Grid::new(format!("{}.Editor", name))
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("flags").color(egui::Color32::LIGHT_BLUE));
                self.flags.add_editor(ui, format!("{}.flags", name));
                ui.end_row();

                ui.label(egui::RichText::new("id").color(egui::Color32::LIGHT_BLUE));
                self.id.add_editor(ui, format!("{}.id", name));
                ui.end_row();

                ui.label(egui::RichText::new("name").color(egui::Color32::LIGHT_BLUE));
                self.name.add_editor(ui, format!("{}.name", name));
                ui.end_row();

                ui.label(egui::RichText::new("script").color(egui::Color32::LIGHT_BLUE));
                self.script.add_editor(ui, format!("{}.script", name));
                ui.end_row();

                ui.label(egui::RichText::new("mesh").color(egui::Color32::LIGHT_BLUE));
                self.mesh.add_editor(ui, format!("{}.mesh", name));
                ui.end_row();

                ui.label(egui::RichText::new("icon").color(egui::Color32::LIGHT_BLUE));
                self.icon.add_editor(ui, format!("{}.icon", name));
                ui.end_row();

                ui.label(egui::RichText::new("enchanting").color(egui::Color32::LIGHT_BLUE));
                self.enchanting.add_editor(ui, format!("{}.enchanting", name));
                ui.end_row();

                ui.label(egui::RichText::new("data").color(egui::Color32::LIGHT_BLUE));
                self.data.add_editor(ui, format!("{}.data", name));
                ui.end_row();

                // custom editor here
                ui.label(egui::RichText::new("text").color(egui::Color32::LIGHT_BLUE));
                ui.push_id(format!("{}.text", name), |ui| {
                    egui::ScrollArea::vertical().min_scrolled_height(600.0).show(ui, |ui| {
                        ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text));
                    });
                });
                ui.end_row();
            });
    }
}
