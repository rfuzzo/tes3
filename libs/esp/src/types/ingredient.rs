// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ingredient {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub data: IngredientData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct IngredientData {
    pub weight: f32,
    pub value: u32,
    pub effects: [EffectId; 4],
    pub skills: [SkillId; 4],
    pub attributes: [AttributeId; 4],
}

impl Load for Ingredient {
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
                b"IRDT" => {
                    stream.expect(56u32)?;
                    this.data = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"ITEX" => {
                    this.icon = stream.load()?;
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

impl Save for Ingredient {
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
        // IRDT
        stream.save(b"IRDT")?;
        stream.save(&56u32)?;
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
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}

impl SqlInfo for Ingredient {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT COLLATE NOCASE"), //FK
            ("mesh", "TEXT"),
            ("icon", "TEXT"),
            ("weight", "REAL"),
            ("value", "INTEGER"),
            ("effects_1", "TEXT"),
            ("effects_2", "TEXT"),
            ("effects_3", "TEXT"),
            ("effects_4", "TEXT"),
            ("skills_1", "TEXT"),
            ("skills_2", "TEXT"),
            ("skills_3", "TEXT"),
            ("skills_4", "TEXT"),
            ("attributes_1", "TEXT"),
            ("attributes_2", "TEXT"),
            ("attributes_3", "TEXT"),
            ("attributes_4", "TEXT"),
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(script) REFERENCES SCPT(id)"]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        as_tes3.table_insert2(
            db,
            mod_name,
            params![
                self.name,
                as_option!(self.script),
                self.mesh,
                self.icon,
                self.data.weight,
                self.data.value,
                as_enum!(self.data.effects[0]),
                as_enum!(self.data.effects[1]),
                as_enum!(self.data.effects[2]),
                as_enum!(self.data.effects[3]),
                as_enum!(self.data.skills[0]),
                as_enum!(self.data.skills[1]),
                as_enum!(self.data.skills[2]),
                as_enum!(self.data.skills[3]),
                as_enum!(self.data.attributes[0]),
                as_enum!(self.data.attributes[1]),
                as_enum!(self.data.attributes[2]),
                as_enum!(self.data.attributes[3])
            ],
        )
    }
}
