// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StartScript {
    pub flags: ObjectFlags,
    pub id: String,
    pub script: String,
}

impl Load for StartScript {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"DATA" => {
                    this.id = stream.load()?;
                }
                b"NAME" => {
                    this.script = stream.load()?;
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

impl Save for StartScript {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&self.id)?;
        // NAME
        if !self.script.is_empty() {
            stream.save(b"NAME")?;
            stream.save(&self.script)?;
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

impl SqlInfo for StartScript {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("script", "TEXT"), //FK
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(script) REFERENCES SCPT(id)"]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        as_tes3.table_insert2(db, mod_name, params![as_option!(self.script)])
    }
}
