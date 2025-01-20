// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Alchemy {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub effects: Vec<Effect>,
    pub data: AlchemyData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct AlchemyData {
    pub weight: f32,
    pub value: u32,
    pub flags: AlchemyFlags,
}

impl Load for Alchemy {
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
                b"TEXT" => {
                    this.icon = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"ALDT" => {
                    stream.expect(12u32)?;
                    this.data = stream.load()?;
                }
                b"ENAM" => {
                    stream.expect(24u32)?;
                    this.effects.push(stream.load()?);
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

impl Save for Alchemy {
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
        // TEXT
        if !self.icon.is_empty() {
            stream.save(b"TEXT")?;
            stream.save(&self.icon)?;
        }
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // ALDT
        stream.save(b"ALDT")?;
        stream.save(&12u32)?;
        stream.save(&self.data)?;
        // ENAM
        for value in &self.effects {
            stream.save(b"ENAM")?;
            stream.save(&24u32)?;
            stream.save(value)?;
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

impl SqlInfo for Alchemy {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT COLLATE NOCASE"), //FK
            ("mesh", "TEXT"),
            ("icon", "TEXT"),
            ("weight", "REAL"),
            ("value", "INTEGER"),
            ("data_flags", "TEXT"), // flags
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
                as_flags!(self.data.flags)
            ],
        )
    }

    fn join_table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        for effect in &self.effects {
            effect.table_insert(db, mod_name, &[&Null, &Null, &self.editor_id()])?;
        }
        Ok(1)
    }
}
