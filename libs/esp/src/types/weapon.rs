// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Weapon {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub enchanting: String,
    pub data: WeaponData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct WeaponData {
    pub weight: f32,
    pub value: u32,
    pub weapon_type: WeaponType,
    pub health: u16,
    pub speed: f32,
    pub reach: f32,
    pub enchantment: u16,
    pub chop_min: u8,
    pub chop_max: u8,
    pub slash_min: u8,
    pub slash_max: u8,
    pub thrust_min: u8,
    pub thrust_max: u8,
    pub flags: WeaponFlags,
}

impl Load for Weapon {
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
                b"WPDT" => {
                    stream.expect(32u32)?;
                    this.data = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"ITEX" => {
                    this.icon = stream.load()?;
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

impl Save for Weapon {
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
        // WPDT
        stream.save(b"WPDT")?;
        stream.save(&32u32)?;
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

impl SqlInfo for Weapon {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT"), //FK
            ("mesh", "TEXT"),
            ("icon", "TEXT"),
            ("enchanting", "TEXT"), //FK
            //
            ("weight", "REAL"),
            ("value", "INTEGER"),
            ("weapon_type", "TEXT"), //enum
            ("health", "INTEGER"),
            ("speed", "REAL"),
            ("reach", "REAL"),
            ("enchantment", "INTEGER"),
            ("chop_min", "INTEGER"),
            ("chop_max", "INTEGER"),
            ("slash_min", "INTEGER"),
            ("slash_max", "INTEGER"),
            ("thrust_min", "INTEGER"),
            ("thrust_max", "INTEGER"),
            ("dataflags", "TEXT"), //flags
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(script) REFERENCES SCPT(id)",
            "FOREIGN KEY(enchanting) REFERENCES ENCH(id)",
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
                self.icon,
                as_option!(self.enchanting),
                self.data.weight,
                self.data.value,
                as_enum!(self.data.weapon_type),
                self.data.health,
                self.data.speed,
                self.data.reach,
                self.data.enchantment,
                self.data.chop_min,
                self.data.chop_max,
                self.data.slash_min,
                self.data.slash_max,
                self.data.thrust_min,
                self.data.thrust_max,
                as_flags!(self.data.flags)
            ],
        )
    }
}
