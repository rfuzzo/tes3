// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Enchanting {
    pub flags: ObjectFlags,
    pub id: String,
    pub effects: Vec<Effect>,
    pub data: EnchantingData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct EnchantingData {
    pub enchant_type: EnchantType,
    pub cost: u32,
    pub max_charge: u32,
    pub flags: EnchantingFlags,
}

impl Load for Enchanting {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"ENDT" => {
                    stream.expect(16u32)?;
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

impl Save for Enchanting {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // ENDT
        stream.save(b"ENDT")?;
        stream.save(&16u32)?;
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

impl SqlInfo for Enchanting {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("effects", "TEXT"),      //json
            ("enchant_type", "TEXT"), //enum
            ("cost", "INTEGER"),
            ("max_charge", "INTEGER"),
            ("flags", "TEXT"), //flags
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        let sql = as_tes3.table_insert_text(mod_name);
        db.execute(
            sql.as_str(),
            params![
                as_json!(self.effects),
                as_enum!(self.data.enchant_type),
                self.data.cost,
                self.data.max_charge,
                as_json!(self.data.flags),
            ],
        )
    }
}
