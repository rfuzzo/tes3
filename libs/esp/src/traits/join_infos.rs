use crate::prelude::*;

// used in RACE, BGSN
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SpellJoin {
    pub spell_id: String,
}
impl SqlJoinInfo for SpellJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_SPELLS"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("race_id", "TEXT"), //FK
            ("bgsn_id", "TEXT"), //FK
            // this
            ("spell_id", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![links[0], links[1], self.spell_id],
        )
    }
}

// used in REGN
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SoundJoin {
    pub sound_id: String,
}
impl SqlJoinInfo for SoundJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_SOUNDS"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("regn_id", "TEXT"), //FK
            // this
            ("sound_id", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(self.table_insert_text(mod_name).as_str(), params![links[0], self.sound_id])
    }
}

// used in CONT
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct InventoryJoin {
    pub index: i32,
    pub item_id: String,
}
impl SqlJoinInfo for InventoryJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_INVENTORY"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("cont_id", "TEXT"), //FK
            // this
            ("index", "INTEGER"),
            ("item_id", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![links[0], self.index, self.item_id],
        )
    }
}
