// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Light {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub sound: String,
    pub data: LightData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct LightData {
    pub weight: f32,
    pub value: u32,
    pub time: i32,
    pub radius: u32,
    pub color: [u8; 4],
    pub flags: LightFlags,
}

impl Load for Light {
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
                b"ITEX" => {
                    this.icon = stream.load()?;
                }
                b"LHDT" => {
                    stream.expect(24u32)?;
                    this.data = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"SNAM" => {
                    this.sound = stream.load()?;
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

impl Save for Light {
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
        // ITEX
        if !self.icon.is_empty() {
            stream.save(b"ITEX")?;
            stream.save(&self.icon)?;
        }
        // LHDT
        stream.save(b"LHDT")?;
        stream.save(&24u32)?;
        stream.save(&self.data)?;
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // SNAM
        if !self.sound.is_empty() {
            stream.save(b"SNAM")?;
            stream.save(&self.sound)?;
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

impl SqlInfo for Light {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT"), //FK
            ("mesh", "TEXT"),
            ("icon", "TEXT"),
            ("sound", "TEXT"), //FK
            ("weight", "REAL"),
            ("value", "INTEGER"),
            ("time", "INTEGER"),
            ("radius", "INTEGER"),
            ("color", "BLOB"), // todo color
            ("flags", "TEXT"), //flags
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(script) REFERENCES SCPT(id)",
            "FOREIGN KEY(sound) REFERENCES SOUN(id)",
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        let sql = as_tes3.table_insert_text(mod_name);
        db.execute(
            sql.as_str(),
            params![
                self.name,
                as_option!(self.script),
                self.mesh,
                self.icon,
                as_option!(self.sound),
                self.data.weight,
                self.data.value,
                self.data.time,
                self.data.radius,
                self.data.color,
                as_json!(self.flags),
            ],
        )
    }
}
