// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Race {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub spells: Vec<String>,
    pub description: String,
    pub data: RaceData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct RaceData {
    pub skill_bonuses: SkillBonuses,
    pub strength: [i32; 2],
    pub intelligence: [i32; 2],
    pub willpower: [i32; 2],
    pub agility: [i32; 2],
    pub speed: [i32; 2],
    pub endurance: [i32; 2],
    pub personality: [i32; 2],
    pub luck: [i32; 2],
    pub height: [f32; 2],
    pub weight: [f32; 2],
    pub flags: RaceFlags,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct SkillBonuses {
    pub skill_0: SkillId,
    pub bonus_0: i32,
    pub skill_1: SkillId,
    pub bonus_1: i32,
    pub skill_2: SkillId,
    pub bonus_2: i32,
    pub skill_3: SkillId,
    pub bonus_3: i32,
    pub skill_4: SkillId,
    pub bonus_4: i32,
    pub skill_5: SkillId,
    pub bonus_5: i32,
    pub skill_6: SkillId,
    pub bonus_6: i32,
}

impl Load for Race {
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
                b"RADT" => {
                    stream.expect(140u32)?;
                    this.data = stream.load()?;
                }
                b"NPCS" => {
                    this.spells.push(stream.load()?);
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

impl Save for Race {
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
        // RADT
        stream.save(b"RADT")?;
        stream.save(&140u32)?;
        stream.save(&self.data)?;
        // NPCS
        for value in &self.spells {
            stream.save(b"NPCS")?;
            stream.save(&32u32)?;
            stream.save::<FixedString<32>>(value.as_ref())?;
        }
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

impl SqlInfo for Race {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("spells", "TEXT"), // array
            ("description", "TEXT"),
            ("skill_0", "TEXT"), //enum
            ("bonus_0", "INTEGER"),
            ("skill_1", "TEXT"), //enum
            ("bonus_1", "INTEGER"),
            ("skill_2", "TEXT"), //enum
            ("bonus_2", "INTEGER"),
            ("skill_3", "TEXT"), //enum
            ("bonus_3", "INTEGER"),
            ("skill_4", "TEXT"), //enum
            ("bonus_4", "INTEGER"),
            ("skill_5", "TEXT"), //enum
            ("bonus_5", "INTEGER"),
            ("skill_6", "TEXT"), //enum
            ("bonus_6", "INTEGER"),
            ("strength", "TEXT"),     //format
            ("intelligence", "TEXT"), //format
            ("willpower", "TEXT"),    //format
            ("agility", "TEXT"),      //format
            ("speed", "TEXT"),        //format
            ("endurance", "TEXT"),    //format
            ("personality", "TEXT"),  //format
            ("luck", "TEXT"),         //format
            ("height", "TEXT"),       //format
            ("weight", "TEXT"),       //format
            ("data_flags", "TEXT"),   //flags
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
                as_enum!(self.data.skill_bonuses.skill_0),
                self.data.skill_bonuses.bonus_0,
                as_enum!(self.data.skill_bonuses.skill_1),
                self.data.skill_bonuses.bonus_1,
                as_enum!(self.data.skill_bonuses.skill_2),
                self.data.skill_bonuses.bonus_2,
                as_enum!(self.data.skill_bonuses.skill_3),
                self.data.skill_bonuses.bonus_3,
                as_enum!(self.data.skill_bonuses.skill_4),
                self.data.skill_bonuses.bonus_4,
                as_enum!(self.data.skill_bonuses.skill_5),
                self.data.skill_bonuses.bonus_5,
                as_enum!(self.data.skill_bonuses.skill_6),
                self.data.skill_bonuses.bonus_6,
                as_sql!(self.data.strength),
                as_sql!(self.data.intelligence),
                as_sql!(self.data.willpower),
                as_sql!(self.data.agility),
                as_sql!(self.data.speed),
                as_sql!(self.data.endurance),
                as_sql!(self.data.personality),
                as_sql!(self.data.luck),
                as_sql!(self.data.height),
                as_sql!(self.data.weight),
                as_flags!(self.data.flags),
            ],
        )
        // join tables
        .and_then(|_| {
            for spell_id in &self.spells {
                let join = SpellJoin {
                    spell_id: spell_id.clone(),
                };
                join.table_insert(db, mod_name, &[&self.editor_id(), &Null, &Null, &Null])?;
            }
            Ok(1)
        })
    }
}
