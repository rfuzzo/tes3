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
            ("rank_names", "TEXT"),           //format
            ("favored_attributes_1", "TEXT"), //enum
            ("favored_attributes_2", "TEXT"), //enum
            ("favored_skills_1", "TEXT"),     //enum
            ("favored_skills_2", "TEXT"),     //enum
            ("favored_skills_3", "TEXT"),     //enum
            ("favored_skills_4", "TEXT"),     //enum
            ("favored_skills_5", "TEXT"),     //enum
            ("favored_skills_6", "TEXT"),     //enum
            ("favored_skills_7", "TEXT"),     //enum
            ("dataflags", "TEXT"),            //flags
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        as_tes3
            .table_insert2(
                db,
                mod_name,
                params![
                    self.name,
                    as_sql!(self.rank_names),
                    as_enum!(self.data.favored_attributes[0]),
                    as_enum!(self.data.favored_attributes[1]),
                    as_enum!(self.data.favored_skills[0]),
                    as_enum!(self.data.favored_skills[1]),
                    as_enum!(self.data.favored_skills[2]),
                    as_enum!(self.data.favored_skills[3]),
                    as_enum!(self.data.favored_skills[4]),
                    as_enum!(self.data.favored_skills[5]),
                    as_enum!(self.data.favored_skills[6]),
                    as_flags!(self.data.flags)
                ],
            )
            // join tables
            .and_then(|_| {
                for reaction in &self.reactions {
                    reaction.table_insert(db, mod_name, &[&self.editor_id()])?;
                }
                Ok(0)
            })
            .and_then(|_| {
                for requirement in &self.data.requirements {
                    requirement.table_insert(db, mod_name, &[&self.editor_id()])?;
                }
                Ok(0)
            })
    }
}
impl Faction {}
