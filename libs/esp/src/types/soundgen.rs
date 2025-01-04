// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SoundGen {
    pub flags: ObjectFlags,
    pub id: String,
    pub sound_gen_type: SoundGenType,
    pub creature: String,
    pub sound: String,
}

impl Load for SoundGen {
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
                    this.sound_gen_type = stream.load()?;
                }
                b"CNAM" => {
                    this.creature = stream.load()?;
                }
                b"SNAM" => {
                    this.sound = stream.load()?;
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

impl Save for SoundGen {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&4u32)?;
        stream.save(&self.sound_gen_type)?;
        // CNAM
        if !self.creature.is_empty() {
            stream.save(b"CNAM")?;
            stream.save(&self.creature)?;
        }
        // SNAM
        if !self.sound.is_empty() {
            stream.save(b"SNAM")?;
            stream.save(&self.sound)?;
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

impl SqlInfo for SoundGen {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("sound_gen_type", "TEXT"), //enum
            ("creature", "TEXT"),       //FK
            ("sound", "TEXT"),          //FK
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(creature) REFERENCES CREA(id)",
            "FOREIGN KEY(sound) REFERENCES SOUN(id)",
        ]
    }

    fn table_name(&self) -> &'static str {
        self.tag_str()
    }

    fn table_insert(&self, db: &Connection, name: &str) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text().as_str(),
            params![
                self.editor_id(),
                name,
                as_enum!(self.sound_gen_type),
                as_option!(self.creature),
                as_option!(self.sound),
            ],
        )
    }
}
