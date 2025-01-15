// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Creature {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub inventory: Vec<(i32, FixedString<32>)>,
    pub spells: Vec<String>,
    pub ai_data: AiData,
    pub ai_packages: Vec<AiPackage>,
    pub travel_destinations: Vec<TravelDestination>,
    pub sound: String,
    pub scale: Option<f32>,
    pub creature_flags: CreatureFlags,
    pub blood_type: u8,
    pub data: CreatureData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct CreatureData {
    pub creature_type: CreatureType,
    pub level: u32,
    pub strength: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub agility: u32,
    pub speed: u32,
    pub endurance: u32,
    pub personality: u32,
    pub luck: u32,
    pub health: u32,
    pub magicka: u32,
    pub fatigue: u32,
    pub soul: u32,
    pub combat: u32,
    pub magic: u32,
    pub stealth: u32,
    pub attack1: (u32, u32),
    pub attack2: (u32, u32),
    pub attack3: (u32, u32),
    pub gold: u32,
}

impl Load for Creature {
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
                b"CNAM" => {
                    this.sound = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"NPDT" => {
                    stream.expect(96u32)?;
                    this.data = stream.load()?;
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    let flags = stream.load()?;
                    (this.creature_flags, this.blood_type) = unpack_flags(flags);
                }
                b"XSCL" => {
                    stream.expect(4u32)?;
                    this.scale = Some(stream.load()?);
                }
                b"NPCO" => {
                    stream.expect(36u32)?;
                    this.inventory.push(stream.load()?);
                }
                b"NPCS" => {
                    this.spells.push(stream.load()?);
                }
                b"AIDT" => {
                    stream.expect(12u32)?;
                    this.ai_data = stream.load()?;
                }
                b"DODT" => {
                    stream.expect(24u32)?;
                    this.travel_destinations.push(stream.load()?);
                }
                b"AI_T" => {
                    stream.expect(16u32)?;
                    this.ai_packages.push(AiPackage::Travel(stream.load()?));
                }
                b"AI_W" => {
                    stream.expect(14u32)?;
                    this.ai_packages.push(AiPackage::Wander(stream.load()?));
                }
                b"AI_E" => {
                    stream.expect(48u32)?;
                    this.ai_packages.push(AiPackage::Escort(stream.load()?));
                }
                b"AI_F" => {
                    stream.expect(48u32)?;
                    this.ai_packages.push(AiPackage::Follow(stream.load()?));
                }
                b"AI_A" => {
                    stream.expect(33u32)?;
                    this.ai_packages.push(AiPackage::Activate(stream.load()?));
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

impl Save for Creature {
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
        // CNAM
        if !self.sound.is_empty() {
            stream.save(b"CNAM")?;
            stream.save(&self.sound)?;
        }
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // NPDT
        stream.save(b"NPDT")?;
        stream.save(&96u32)?;
        stream.save(&self.data)?;
        // FLAG
        stream.save(b"FLAG")?;
        stream.save(&4u32)?;
        stream.save(&pack_flags(self.creature_flags, self.blood_type))?;
        // XSCL
        if let Some(value) = &self.scale {
            let scale = value.clamp(0.5, 2.0);
            let scale_is_default = (scale - 1.0).abs() < 1e-6;
            if !scale_is_default {
                stream.save(b"XSCL")?;
                stream.save(&4u32)?;
                stream.save(&scale)?;
            }
        }
        // NPCO
        for value in &self.inventory {
            stream.save(b"NPCO")?;
            stream.save(&36u32)?;
            stream.save(value)?;
        }
        // NPCS
        for value in &self.spells {
            stream.save(b"NPCS")?;
            stream.save(&32u32)?;
            stream.save::<FixedString<32>>(value.as_ref())?;
        }
        // AIDT
        stream.save(b"AIDT")?;
        stream.save(&12u32)?;
        stream.save(&self.ai_data)?;
        // DODT
        for value in &self.travel_destinations {
            stream.save(b"DODT")?;
            stream.save(&24u32)?;
            stream.save(value)?;
        }
        //
        for value in &self.ai_packages {
            match value {
                AiPackage::Travel(package) => {
                    // AI_T
                    stream.save(b"AI_T")?;
                    stream.save(&16u32)?;
                    stream.save(package)?;
                }
                AiPackage::Wander(package) => {
                    // AI_W
                    stream.save(b"AI_W")?;
                    stream.save(&14u32)?;
                    stream.save(package)?;
                }
                AiPackage::Escort(package) => {
                    // AI_E
                    stream.save(b"AI_E")?;
                    stream.save(&48u32)?;
                    stream.save(package)?;
                }
                AiPackage::Follow(package) => {
                    // AI_F
                    stream.save(b"AI_F")?;
                    stream.save(&48u32)?;
                    stream.save(package)?;
                }
                AiPackage::Activate(package) => {
                    // AI_A
                    stream.save(b"AI_A")?;
                    stream.save(&33u32)?;
                    stream.save(package)?;
                }
            }
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

#[allow(clippy::cast_possible_truncation)]
const fn unpack_flags(flags: u32) -> (CreatureFlags, u8) {
    // Note: drops unknown flags, may be `.ess` incompatible.
    let creature_flags = CreatureFlags::from_bits_truncate(flags as u8);
    // Blood types start at the 10th bit and are 3 bits long.
    let blood_type = ((flags >> 10) & 0b111) as u8;
    (creature_flags, blood_type)
}

#[allow(clippy::cast_lossless)]
const fn pack_flags(npc_flags: CreatureFlags, blood_type: u8) -> u32 {
    let flags = npc_flags.bits() as u32;
    let blood_type = blood_type as u32;
    flags | ((blood_type & 0b111) << 10)
}

impl SqlInfo for Creature {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("script", "TEXT"), //FK
            ("mesh", "TEXT"),
            // ai data
            ("hello", "INTEGER"),
            ("fight", "INTEGER"),
            ("flee", "INTEGER"),
            ("alarm", "INTEGER"),
            ("services", "INTEGER"), // flags
            //
            ("sound", "TEXT"), //FK
            ("scale", "REAL"),
            ("creature_flags", "TEXT"), //flags
            ("blood_type", "INTEGER"),
            // data
            ("creature_type", "TEXT"), //enun
            ("level", "INTEGER"),
            ("strength", "INTEGER"),
            ("intelligence", "INTEGER"),
            ("willpower", "INTEGER"),
            ("agility", "INTEGER"),
            ("speed", "INTEGER"),
            ("endurance", "INTEGER"),
            ("personality", "INTEGER"),
            ("luck", "INTEGER"),
            ("health", "INTEGER"),
            ("magicka", "INTEGER"),
            ("fatigue", "INTEGER"),
            ("soul", "INTEGER"),
            ("combat", "INTEGER"),
            ("magic", "INTEGER"),
            ("stealth", "INTEGER"),
            ("attack1_min", "INTEGER"),
            ("attack1_max", "INTEGER"),
            ("attack2_min", "INTEGER"),
            ("attack2_max", "INTEGER"),
            ("attack3_min", "INTEGER"),
            ("attack3_max", "INTEGER"),
            ("gold", "INTEGER"),
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(script) REFERENCES SCPT(id)",
            //"FOREIGN KEY(sound) REFERENCES SNDG(id)", //todo cycle
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
                self.ai_data.hello,
                self.ai_data.fight,
                self.ai_data.flee,
                self.ai_data.alarm,
                as_flags!(self.ai_data.services),
                as_option!(self.sound),
                self.scale,
                as_flags!(self.creature_flags),
                self.blood_type,
                as_enum!(self.data.creature_type),
                self.data.level,
                self.data.strength,
                self.data.intelligence,
                self.data.willpower,
                self.data.agility,
                self.data.speed,
                self.data.endurance,
                self.data.personality,
                self.data.luck,
                self.data.health,
                self.data.magicka,
                self.data.fatigue,
                self.data.soul,
                self.data.combat,
                self.data.magic,
                self.data.stealth,
                self.data.attack1.0,
                self.data.attack1.1,
                self.data.attack2.0,
                self.data.attack2.1,
                self.data.attack3.0,
                self.data.attack3.1,
                self.data.gold,
            ],
        )
    }

    fn join_table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        for (idx, item_id) in &self.inventory {
            let join = InventoryJoin {
                index: *idx,
                item_id: item_id.to_string(),
            };
            join.table_insert(db, mod_name, &[&Null, &self.editor_id().to_lowercase(), &Null])?;
        }

        for spell_id in &self.spells {
            let join = SpellJoin {
                spell_id: spell_id.clone(),
            };
            join.table_insert(db, mod_name, &[&Null, &Null, &self.editor_id().to_lowercase(), &Null])?;
        }

        for dest in &self.travel_destinations {
            dest.table_insert(db, mod_name, &[&self.editor_id().to_lowercase(), &Null])?;
        }

        for package in &self.ai_packages {
            package.table_insert(db, mod_name, &[&self.editor_id().to_lowercase(), &Null])?;
        }
        Ok(0)
    }
}
