use crate::prelude::*;

mod activator;
mod aidata;
mod aipackage;
mod alchemy;
mod apparatus;
mod armor;
mod bipedobject;
mod birthsign;
mod bodypart;
mod book;
mod cell;
mod class;
mod clothing;
mod container;
mod creature;
mod dialogue;
mod dialogueinfo;
mod door;
mod effect;
mod enchanting;
mod enums;
mod faction;
mod flags;
mod gamesetting;
mod globalvariable;
mod header;
mod ingredient;
mod landscape;
mod landscapetexture;
mod leveledcreature;
mod leveleditem;
mod light;
mod lockpick;
mod magiceffect;
mod miscitem;
mod npc;
mod pathgrid;
mod plugin;
mod probe;
mod race;
mod reference;
mod region;
mod repairitem;
mod script;
mod skill;
mod sound;
mod soundgen;
mod spell;
mod startscript;
mod static_;
mod string;
mod weapon;

pub use activator::*;
pub use aidata::*;
pub use aipackage::*;
pub use alchemy::*;
pub use apparatus::*;
pub use armor::*;
pub use bipedobject::*;
pub use birthsign::*;
pub use bodypart::*;
pub use book::*;
pub use cell::*;
pub use class::*;
pub use clothing::*;
pub use container::*;
pub use creature::*;
pub use dialogue::*;
pub use dialogueinfo::*;
pub use door::*;
pub use effect::*;
pub use enchanting::*;
pub use enums::*;
pub use faction::*;
pub use flags::*;
pub use gamesetting::*;
pub use globalvariable::*;
pub use header::*;
pub use ingredient::*;
pub use landscape::*;
pub use landscapetexture::*;
pub use leveledcreature::*;
pub use leveleditem::*;
pub use light::*;
pub use lockpick::*;
pub use magiceffect::*;
pub use miscitem::*;
pub use npc::*;
pub use pathgrid::*;
pub use plugin::*;
pub use probe::*;
pub use race::*;
pub use reference::*;
pub use region::*;
pub use repairitem::*;
pub use script::*;
pub use skill::*;
pub use sound::*;
pub use soundgen::*;
pub use spell::*;
pub use startscript::*;
pub use static_::*;
pub use string::*;
pub use weapon::*;

#[rustfmt::skip]
#[esp_meta]
#[derive(TES3Object, Clone, Debug, From, PartialEq)]
pub enum TES3Object {
    #[tag("TES3")] Header(Header),                      // incomplete, values
    #[tag("GMST")] GameSetting(GameSetting),            // done
    #[tag("GLOB")] GlobalVariable(GlobalVariable),      // done
    #[tag("CLAS")] Class(Class),                        // done
    #[tag("FACT")] Faction(Faction),                    // done
    #[tag("RACE")] Race(Race),                          // done
    #[tag("SOUN")] Sound(Sound),                        // done
    #[tag("SNDG")] SoundGen(SoundGen),                  // done
    #[tag("SKIL")] Skill(Skill),                        // done
    #[tag("MGEF")] MagicEffect(MagicEffect),            // done
    #[tag("SCPT")] Script(Script),                      // incomplete, values
    #[tag("REGN")] Region(Region),                      // done
    #[tag("BSGN")] Birthsign(Birthsign),                // done
    #[tag("SSCR")] StartScript(StartScript),            // done
    #[tag("LTEX")] LandscapeTexture(LandscapeTexture),  // done
    #[tag("SPEL")] Spell(Spell),                        // done
    #[tag("STAT")] Static(Static),                      // done
    #[tag("DOOR")] Door(Door),                          // done
    #[tag("MISC")] MiscItem(MiscItem),                  // done
    #[tag("WEAP")] Weapon(Weapon),                      // done
    #[tag("CONT")] Container(Container),                // done
    #[tag("CREA")] Creature(Creature),                  // done
    #[tag("BODY")] Bodypart(Bodypart),                  // done
    #[tag("LIGH")] Light(Light),                        // done
    #[tag("ENCH")] Enchanting(Enchanting),              // done
    #[tag("NPC_")] Npc(Npc),                            // done
    #[tag("ARMO")] Armor(Armor),                        // done
    #[tag("CLOT")] Clothing(Clothing),                  // done
    #[tag("REPA")] RepairItem(RepairItem),              // done
    #[tag("ACTI")] Activator(Activator),                // done
    #[tag("APPA")] Apparatus(Apparatus),                // done
    #[tag("LOCK")] Lockpick(Lockpick),                  // done
    #[tag("PROB")] Probe(Probe),                        // done
    #[tag("INGR")] Ingredient(Ingredient),              // done
    #[tag("BOOK")] Book(Book),                          // done
    #[tag("ALCH")] Alchemy(Alchemy),                    // done
    #[tag("LEVI")] LeveledItem(LeveledItem),            // done
    #[tag("LEVC")] LeveledCreature(LeveledCreature),    // done
    #[tag("CELL")] Cell(Cell),                          // done
    #[tag("LAND")] Landscape(Landscape),                // incomplete, blobs
    #[tag("PGRD")] PathGrid(PathGrid),                  // incomplete, blobs
    #[tag("DIAL")] Dialogue(Dialogue),                  // done
    #[tag("INFO")] DialogueInfo(DialogueInfo),          // done
}
