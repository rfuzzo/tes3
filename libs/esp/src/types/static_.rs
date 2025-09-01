// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Static {
    pub flags: ObjectFlags,
    pub id: String,
    pub mesh: String,
}

impl Load for Static {
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

impl Save for Static {
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
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}

impl SqlInfo for Static {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![("mesh", "TEXT")]
    }

    fn insert_sql_record(&self, mod_name: &str, s: &mut CachedStatement<'_>) -> rusqlite::Result<usize> {
        let id = self.editor_id();
        let flags = as_flags!(self.object_flags());

        let params = params![id, mod_name, flags, self.mesh];

        s.execute(params)
    }
}
