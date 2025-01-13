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

impl SqlJoinInfo for Effect {
    fn table_name(&self) -> &'static str {
        "JOIN_Effect"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("spell_id", "TEXT"), //FK
            ("ench_id", "TEXT"),  //FK
            // this
            ("magic_effect", "TEXT"), //enum
            ("skill", "TEXT"),        //enum
            ("attribute", "TEXT"),    //enum
            ("range", "TEXT"),        //enum
            ("area", "INTEGER"),
            ("duration", "INTEGER"),
            ("min_magnitude", "INTEGER"),
            ("max_magnitude", "INTEGER"),
        ]
    }

    // used in SPELL, ENCH
    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![
                links[0],
                links[1],
                as_enum!(self.magic_effect),
                as_enum!(self.skill),
                as_enum!(self.attribute),
                as_enum!(self.range),
                self.area,
                self.duration,
                self.min_magnitude,
                self.max_magnitude,
            ],
        )
    }
}
