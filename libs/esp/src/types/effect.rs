// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct Effect {
    pub magic_effect: EffectId2,
    pub skill: SkillId2,
    pub attribute: AttributeId2,
    pub range: EffectRange,
    pub area: u32,
    pub duration: u32,
    pub min_magnitude: u32,
    pub max_magnitude: u32,
}

// impl SqlInfo for Effect {
//     fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
//         vec![
//             ("magic_effect", "TEXT"), //enum
//             ("skill", "TEXT"),        //enum
//             ("attribute", "TEXT"),    //enum
//             ("range", "TEXT"),        //enum
//             ("area", "INTEGER"),
//             ("duration", "INTEGER"),
//             ("min_magnitude", "INTEGER"),
//             ("max_magnitude", "INTEGER"),
//         ]
//     }

//     fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
//         let as_tes3: TES3Object = self.clone().into();
//         let sql = as_tes3.table_insert_text(mod_name);
//         db.execute(
//             sql.as_str(),
//             params![
//                 as_enum!(self.magic_effect),
//                 as_enum!(self.skill),
//                 as_enum!(self.attribute),
//                 as_enum!(self.range),
//                 self.area,
//                 self.duration,
//                 self.min_magnitude,
//                 self.max_magnitude,
//             ],
//         )
//     }
// }
