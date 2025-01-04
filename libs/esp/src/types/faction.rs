// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Faction {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub rank_names: Vec<String>,
    pub reactions: Vec<FactionReaction>,
    pub data: FactionData,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FactionData {
    pub favored_attributes: [AttributeId; 2],
    pub requirements: [FactionRequirement; 10],
    pub favored_skills: [SkillId; 7],
    pub flags: FactionFlags,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct FactionRequirement {
    pub attributes: [i32; 2],
    pub primary_skill: i32,
    pub favored_skill: i32,
    pub reputation: i32,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FactionReaction {
    pub faction: String,
    pub reaction: i32,
}

impl Load for Faction {
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
                b"RNAM" => {
                    this.rank_names.push(stream.load()?);
                }
                b"FADT" => {
                    stream.expect(240u32)?;
                    this.data = stream.load()?;
                }
                b"ANAM" => {
                    this.reactions.push(stream.load()?);
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

impl Save for Faction {
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
        // RNAM
        for value in &self.rank_names {
            stream.save(b"RNAM")?;
            stream.save(&32u32)?;
            stream.save::<FixedString<32>>(value.as_ref())?;
        }
        // FADT
        stream.save(b"FADT")?;
        stream.save(&240u32)?;
        stream.save(&self.data)?;
        // ANAM / INTV
        for value in &self.reactions {
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

impl Load for FactionData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let favored_attributes = stream.load()?;
        let requirements = [
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
        ];
        let favored_skills = stream.load()?;
        let flags = stream.load()?;
        Ok(Self {
            favored_attributes,
            requirements,
            favored_skills,
            flags,
        })
    }
}

impl Save for FactionData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.favored_attributes)?;
        for requirement in &self.requirements {
            stream.save(requirement)?;
        }
        stream.save(&self.favored_skills)?;
        stream.save(&self.flags)?;
        Ok(())
    }
}

impl Load for FactionReaction {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        // ANAM
        let faction = stream.load()?;
        // INTV
        stream.expect(*b"INTV")?;
        stream.expect(4u32)?;
        let reaction = stream.load()?;
        Ok(Self { faction, reaction })
    }
}

impl Save for FactionReaction {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        // ANAM
        stream.save(b"ANAM")?;
        stream.save(&self.faction)?;
        // INTV
        stream.save(b"INTV")?;
        stream.save(&4u32)?;
        stream.save(&self.reaction)?;
        Ok(())
    }
}

impl SqlInfo for Faction {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("rank_names", "TEXT"),         //array
            ("reactions", "TEXT"),          //array
            ("favored_attributes", "TEXT"), //enum
            ("requirements", "TEXT"),       //array
            ("favored_skills", "TEXT"),     //array
            ("flags", "TEXT"),              //flags
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
                self.name,
                as_json!(self.rank_names),
                as_json!(self.reactions),
                as_json!(self.data.favored_attributes),
                as_json!(self.data.requirements),
                as_json!(self.data.favored_skills),
                as_json!(self.data.flags)
            ],
        )
    }
}
impl Faction {}
