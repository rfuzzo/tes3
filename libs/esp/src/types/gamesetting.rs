// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GameSetting {
    pub flags: ObjectFlags,
    pub id: String,
    pub value: GameSettingValue,
}

#[esp_meta]
#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub enum GameSettingValue {
    #[default]
    Float(f32),
    Integer(i32),
    String(String),
}

impl Load for GameSetting {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"STRV" => {
                    this.value = GameSettingValue::String(stream.load()?);
                }
                b"FLTV" => {
                    stream.expect(4u32)?;
                    this.value = GameSettingValue::Float(stream.load()?);
                }
                b"INTV" => {
                    stream.expect(4u32)?;
                    this.value = GameSettingValue::Integer(stream.load()?);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for GameSetting {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        //
        match &self.value {
            GameSettingValue::String(value) => {
                // STRV
                stream.save(b"STRV")?;
                stream.save(value)?;
            }
            GameSettingValue::Float(value) => {
                // FLTV
                stream.save(b"FLTV")?;
                stream.save(&4u32)?;
                stream.save(value)?;
            }
            GameSettingValue::Integer(value) => {
                // INTV
                stream.save(b"INTV")?;
                stream.save(&4u32)?;
                stream.save(value)?;
            }
        }
        Ok(())
    }
}

impl SqlInfo for GameSetting {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![("type", "TEXT"), ("val", "TEXT")]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let type_name = match &self.value {
            GameSettingValue::Float(_) => "Float".to_string(),
            GameSettingValue::Integer(_) => "Integer".to_string(),
            GameSettingValue::String(_) => "String".to_string(),
        };
        let value = match &self.value {
            GameSettingValue::Float(f) => f.to_string(),
            GameSettingValue::Integer(i) => i.to_string(),
            GameSettingValue::String(s) => s.to_string(),
        };

        let as_tes3: TES3Object = self.clone().into();
        as_tes3.table_insert2(db, mod_name, params![type_name, value])
    }
}
