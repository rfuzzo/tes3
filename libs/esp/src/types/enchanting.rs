// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Enchanting {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub data: Option<EnchantingData>,
    pub effects: Option<Vec<Effect>>,
    pub deleted: Option<u32>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct EnchantingData {
    pub kind: EnchantType,
    pub cost: u32,
    pub max_charge: u32,
    pub flags: u32,
}

impl Load for Enchanting {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self {
            flags1: stream.load()?,
            flags2: stream.load()?,
            ..default()
        };

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"ENDT" => {
                    stream.expect(16u32)?;
                    this.data = Some(stream.load()?);
                }
                b"ENAM" => {
                    stream.expect(24u32)?;
                    this.effects.get_or_insert_with(default).push(stream.load()?);
                }
                b"DELE" => {
                    stream.expect(4u32)?;
                    this.deleted = Some(stream.load()?);
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
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // ENDT
        if let Some(value) = &self.data {
            stream.save(b"ENDT")?;
            stream.save(&16u32)?;
            stream.save(value)?;
        }
        // ENAM
        for effect in self.effects.iter().flatten() {
            stream.save(b"ENAM")?;
            stream.save(&24u32)?;
            stream.save(effect)?;
        }
        // DELE
        if let Some(value) = &self.deleted {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        Ok(())
    }
}