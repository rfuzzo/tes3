// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Container {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub encumbrance: f32,
    pub container_flags: ContainerFlags,
    pub inventory: Vec<(i32, FixedString<32>)>,
}

impl Load for Container {
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
                b"CNDT" => {
                    stream.expect(4u32)?;
                    this.encumbrance = stream.load()?;
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    this.container_flags = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"NPCO" => {
                    stream.expect(36u32)?;
                    this.inventory.push(stream.load()?);
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

impl Save for Container {
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
        // CNDT
        stream.save(b"CNDT")?;
        stream.save(&4u32)?;
        stream.save(&self.encumbrance)?;
        // FLAG
        stream.save(b"FLAG")?;
        stream.save(&4u32)?;
        stream.save(&self.container_flags)?;
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // NPCO
        for (count, id) in &self.inventory {
            stream.save(b"NPCO")?;
            stream.save(&36u32)?;
            stream.save(count)?;
            stream.save(id)?;
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

impl SqlInfo for Container {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT COLLATE NOCASE"), //FK
            ("mesh", "TEXT"),
            ("encumbrance", "REAL"),
            ("container_flags", "TEXT"), //flags
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(script) REFERENCES SCPT(id)"]
    }

    fn insert_sql_record(&self, mod_name: &str, s: &mut CachedStatement<'_>) -> rusqlite::Result<usize> {
        let id = self.editor_id();
        let flags = as_flags!(self.object_flags());

        let params = params![
            id,
            mod_name,
            flags,
            self.name,
            as_option!(self.script),
            self.mesh,
            self.encumbrance,
            as_flags!(self.container_flags),
        ];

        s.execute(params)
    }

    fn insert_join_sql_record(&self, mod_name: &str, s: &mut CachedStatement<'_>) -> rusqlite::Result<usize> {
        for (idx, item_id) in &self.inventory {
            let join = InventoryJoin {
                index: *idx,
                item_id: item_id.to_string(),
            };
            join.table_insert(s, mod_name, &[&self.editor_id(), &Null, &Null])?;
        }
        Ok(0)
    }
}
