// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LeveledItem {
    pub flags: ObjectFlags,
    pub id: String,
    pub leveled_item_flags: LeveledItemFlags,
    pub chance_none: u8,
    pub items: Vec<(String, u16)>,
}

impl Load for LeveledItem {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"DATA" => {
                    stream.expect(4u32)?;
                    this.leveled_item_flags = stream.load()?;
                }
                b"NNAM" => {
                    stream.expect(1u32)?;
                    this.chance_none = stream.load()?;
                }
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.items.reserve(stream.load_as::<u32, usize>()?);
                }
                b"INAM" => {
                    this.items.push(default());
                    this.items.last_mut().ok_or_else(err)?.0 = stream.load()?;
                }
                b"INTV" => {
                    stream.expect(2u32)?;
                    this.items.last_mut().ok_or_else(err)?.1 = stream.load()?;
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

impl Save for LeveledItem {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&4u32)?;
        stream.save(&self.leveled_item_flags)?;
        // NNAM
        stream.save(b"NNAM")?;
        stream.save(&1u32)?;
        stream.save(&self.chance_none)?;
        // INDX
        if !self.items.is_empty() {
            stream.save(b"INDX")?;
            stream.save(&4u32)?;
            stream.save_as::<u32>(self.items.len())?;
            //
            for (item, level) in &self.items {
                // INAM
                stream.save(b"INAM")?;
                stream.save(item)?;
                // INTV
                stream.save(b"INTV")?;
                stream.save(&2u32)?;
                stream.save(level)?;
            }
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

fn err() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "PC Level provided without a corresponding item id",
    )
}

impl SqlInfo for LeveledItem {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("leveled_item_flags", "TEXT"), //flags
            ("chance_none", "INTEGER"),
        ]
    }

    fn insert_sql_record(&self, mod_name: &str, s: &mut CachedStatement<'_>) -> rusqlite::Result<usize> {
        let id = self.editor_id();
        let flags = as_flags!(self.object_flags());

        let params = params![id, mod_name, flags, as_flags!(self.leveled_item_flags), self.chance_none];

        s.execute(params)
    }

    fn insert_join_sql_record(&self, mod_name: &str, tx: &mut Transaction<'_>) -> rusqlite::Result<usize> {
        // prepare cached schema
        let schema = ItemJoin::default().get_join_insert_schema();
        let mut s = tx.prepare_cached(&schema).unwrap();

        for (item_id, probability) in &self.items {
            let join = ItemJoin {
                item_id: item_id.to_string(),
                probability: probability.to_owned(),
            };
            join.table_insert(&mut s, mod_name, &[&self.editor_id()])?;
        }
        Ok(0)
    }
}
