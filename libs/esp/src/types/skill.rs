// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Skill {
    pub flags: ObjectFlags,
    pub skill_id: SkillId,
    pub data: SkillData,
    pub description: String,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct SkillData {
    pub governing_attribute: i32,
    pub specialization: i32,
    pub actions: [f32; 4],
}

impl Load for Skill {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.skill_id = stream.load()?;
                }
                b"SKDT" => {
                    stream.expect(24u32)?;
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

impl Save for Skill {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // INDX
        stream.save(b"INDX")?;
        stream.save(&4u32)?;
        stream.save(&self.skill_id)?;
        // SKDT
        stream.save(b"SKDT")?;
        stream.save(&24u32)?;
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

impl SqlInfo for Skill {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("skill_id", "TEXT"), //enum
            ("governing_attribute", "INTEGER"),
            ("specialization", "INTEGER"),
            ("actions", "TEXT"), //array
            ("description", "TEXT"),
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![]
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
                as_enum!(self.skill_id),
                self.data.governing_attribute,
                self.data.specialization,
                as_json!(self.data.actions),
                self.description,
            ],
        )
    }
}
