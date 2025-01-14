// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Door {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub open_sound: String,
    pub close_sound: String,
}

impl Load for Door {
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
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"SNAM" => {
                    this.open_sound = stream.load()?;
                }
                b"ANAM" => {
                    this.close_sound = stream.load()?;
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

impl Save for Door {
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
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // SNAM
        if !self.open_sound.is_empty() {
            stream.save(b"SNAM")?;
            stream.save(&self.open_sound)?;
        }
        // ANAM
        if !self.close_sound.is_empty() {
            stream.save(b"ANAM")?;
            stream.save(&self.close_sound)?;
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

impl SqlInfo for Door {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT"), //FK
            ("mesh", "TEXT"),
            ("open_sound", "TEXT"),  //FK
            ("close_sound", "TEXT"), //FK
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(open_sound) REFERENCES SOUN(id)",
            "FOREIGN KEY(close_sound) REFERENCES SOUN(id)",
        ]
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
                as_option!(self.open_sound),
                as_option!(self.close_sound),
            ],
        )
    }
}
