// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Birthsign {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub texture: String,
    pub description: String,
    pub spells: Vec<String>,
}

impl Load for Birthsign {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"TNAM" => {
                    this.texture = stream.load()?;
                }
                b"DESC" => {
                    this.description = stream.load()?;
                }
                b"NPCS" => {
                    this.spells.push(stream.load()?);
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

impl Save for Birthsign {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // TNAM
        if !self.texture.is_empty() {
            stream.save(b"TNAM")?;
            stream.save(&self.texture)?;
        }
        // DESC
        if !self.description.is_empty() {
            stream.save(b"DESC")?;
            stream.save(&self.description)?;
        }
        // NPCS
        for value in &self.spells {
            stream.save(b"NPCS")?;
            stream.save(&32u32)?;
            stream.save::<FixedString<32>>(value.as_ref())?;
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

impl SqlInfo for Birthsign {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![("name", "TEXT"), ("texture", "TEXT"), ("description", "TEXT")]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        as_tes3.table_insert2(db, mod_name, params![self.name, self.texture, self.description])
    }

    fn join_table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        for spell_id in &self.spells {
            let join = SpellJoin {
                spell_id: spell_id.clone().to_lowercase(),
            };
            join.table_insert(db, mod_name, params![&Null, &self.editor_id().to_lowercase(), &Null, &Null])?;
        }
        Ok(0)
    }
}
