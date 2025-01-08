// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Class {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub description: String,
    pub data: ClassData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassData {
    pub attribute1: AttributeId,
    pub attribute2: AttributeId,
    pub specialization: Specialization,
    pub minor1: SkillId,
    pub major1: SkillId,
    pub minor2: SkillId,
    pub major2: SkillId,
    pub minor3: SkillId,
    pub major3: SkillId,
    pub minor4: SkillId,
    pub major4: SkillId,
    pub minor5: SkillId,
    pub major5: SkillId,
    pub flags: ClassFlags,
    pub services: ServiceFlags,
}

impl Load for Class {
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
                b"CLDT" => {
                    stream.expect(60u32)?;
                    this.data = stream.load()?;
                }
                b"DESC" => {
                    this.description = stream.load()?;
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

impl Save for Class {
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
        // CLDT
        stream.save(b"CLDT")?;
        stream.save(&60u32)?;
        stream.save(&self.data)?;
        // DESC
        if !self.description.is_empty() {
            stream.save(b"DESC")?;
            stream.save(&self.description)?;
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

impl SqlInfo for Class {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("description", "TEXT"),
            ("attribute1", "TEXT"),     //enum
            ("attribute2", "TEXT"),     //enum
            ("specialization", "TEXT"), //enum
            ("minor1", "TEXT"),         //enum
            ("major1", "TEXT"),         //enum
            ("minor2", "TEXT"),         //enum
            ("major2", "TEXT"),         //enum
            ("minor3", "TEXT"),         //enum
            ("major3", "TEXT"),         //enum
            ("minor4", "TEXT"),         //enum
            ("major4", "TEXT"),         //enum
            ("minor5", "TEXT"),         //enum
            ("major5", "TEXT"),         //enum
            ("data_flags", "TEXT"),     //flags
            ("services", "TEXT"),       //flags
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        let sql = as_tes3.table_insert_text(mod_name);
        db.execute(
            sql.as_str(),
            params![
                self.name,
                self.description,
                as_enum!(self.data.attribute1),
                as_enum!(self.data.attribute2),
                as_enum!(self.data.specialization),
                as_enum!(self.data.minor1),
                as_enum!(self.data.major1),
                as_enum!(self.data.minor2),
                as_enum!(self.data.major2),
                as_enum!(self.data.minor3),
                as_enum!(self.data.major3),
                as_enum!(self.data.minor4),
                as_enum!(self.data.major4),
                as_enum!(self.data.minor5),
                as_enum!(self.data.major5),
                as_flags!(self.data.flags),
                as_flags!(self.data.services),
            ],
        )
    }
}
