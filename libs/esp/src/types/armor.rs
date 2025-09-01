// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Armor {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub enchanting: String,
    pub biped_objects: Vec<BipedObject>,
    pub data: ArmorData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct ArmorData {
    pub armor_type: ArmorType,
    pub weight: f32,
    pub value: u32,
    pub health: u32,
    pub enchantment: u32,
    pub armor_rating: u32,
}

impl Load for Armor {
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
                b"AODT" => {
                    stream.expect(24u32)?;
                    this.data = stream.load()?;
                }
                b"ITEX" => {
                    this.icon = stream.load()?;
                }
                b"INDX" => {
                    this.biped_objects.push(stream.load()?);
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

impl Save for Armor {
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
        // AODT
        stream.save(b"AODT")?;
        stream.save(&24u32)?;
        stream.save(&self.data)?;
        // ITEX
        if !self.icon.is_empty() {
            stream.save(b"ITEX")?;
            stream.save(&self.icon)?;
        }
        // INDX / BNAM / CNAM
        for value in &self.biped_objects {
            stream.save(value)?;
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

impl SqlInfo for Armor {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT COLLATE NOCASE"), //FK
            ("mesh", "TEXT"),
            ("icon", "TEXT"),
            ("enchanting", "TEXT COLLATE NOCASE"), //FK
            ("armor_type", "TEXT"),                //enum
            ("weight", "REAL"),
            ("value", "INTEGER"),
            ("health", "INTEGER"),
            ("enchantment", "INTEGER"),
            ("armor_rating", "INTEGER"),
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(script) REFERENCES SCPT(id)",
            "FOREIGN KEY(enchanting) REFERENCES ENCH(id)",
        ]
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
            self.icon,
            as_option!(self.enchanting),
            as_enum!(self.data.armor_type),
            self.data.weight,
            self.data.value,
            self.data.health,
            self.data.enchantment,
            self.data.armor_rating,
        ];

        s.execute(params)
    }

    fn insert_join_sql_record(&self, mod_name: &str, tx: &mut Transaction<'_>) -> rusqlite::Result<usize> {
        // prepare cached schema
        let schema = BipedObject::default().get_join_insert_schema();
        let mut s = tx.prepare_cached(&schema).unwrap();

        for biped_object in &self.biped_objects {
            biped_object.table_insert(&mut s, mod_name, &[&self.editor_id(), &Null])?;
        }
        Ok(0)
    }
}
