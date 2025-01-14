use crate::prelude::*;

// used in RACE, BSGN, CREA, NPC_
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SpellJoin {
    pub spell_id: String,
}
impl SqlJoinInfo for SpellJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_SPELLS"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(race_id) REFERENCES RACE(id)",
            "FOREIGN KEY(bsgn_id) REFERENCES BSGN(id)",
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NPC_(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("race_id", "TEXT NOT NULL"), //FK
            ("bsgn_id", "TEXT NOT NULL"), //FK
            ("crea_id", "TEXT NOT NULL"), //FK
            ("npc_id", "TEXT NOT NULL"),  //FK
            // this
            ("spell_id", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![links[0], links[1], links[2], links[3], self.spell_id],
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
        vec!["FOREIGN KEY(regn_id) REFERENCES REGN(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("regn_id", "TEXT NOT NULL"), //FK
            // this
            ("sound_id", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(self.table_insert_text(mod_name).as_str(), params![links[0], self.sound_id])
    }
}

// used in CONT, CREA, NPC_
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
        vec![
            "FOREIGN KEY(cont_id) REFERENCES CONT(id)",
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NOC_(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("cont_id", "TEXT NOT NULL"), //FK
            ("crea_id", "TEXT NOT NULL"), //FK
            ("npc_id", "TEXT NOT NULL"),  //FK
            // this
            ("index", "INTEGER"),
            ("item_id", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![links[0], links[1], links[2], self.index, self.item_id],
        )
    }
}

// used in LEVI
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ItemJoin {
    pub item_id: String,
    pub probability: u16,
}
impl SqlJoinInfo for ItemJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_Item"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(levi_id) REFERENCES LEVI(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("levi_id", "TEXT NOT NULL"), //FK
            // this
            ("item_id", "TEXT"), //FK
            ("probability", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![links[0], self.item_id, self.probability],
        )
    }
}

// used in LEVC
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CreatureJoin {
    pub creature_id: String,
    pub probability: u16,
}
impl SqlJoinInfo for CreatureJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_Creature"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(levc_id) REFERENCES LEVC(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("levc_id", "TEXT NOT NULL"), //FK
            // this
            ("creature_id", "TEXT"), //FK
            ("probability", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![links[0], self.creature_id, self.probability],
        )
    }
}

// used in CREA, NPC_
impl SqlJoinInfo for TravelDestination {
    fn table_name(&self) -> &'static str {
        "JOIN_TravelDestination"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NPC_(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("crea_id", "TEXT NOT NULL"), //FK
            ("npc_id", "TEXT NOT NULL"),  //FK
            // this
            ("translation", "TEXT"),
            ("rotation", "TEXT"),
            ("cell", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![
                links[0],
                links[1],
                as_sql!(self.translation),
                as_sql!(self.rotation),
                self.cell
            ],
        )
    }
}

// used in CREA, NPC_
impl SqlJoinInfo for AiPackage {
    fn table_name(&self) -> &'static str {
        "JOIN_AiPackage"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NPC_(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("crea_id", "TEXT NOT NULL"), //FK
            ("npc_id", "TEXT NOT NULL"),  //FK
            // this
            ("package", "TEXT"), //json TODO
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        let value = serde_json::to_string(&self).unwrap();
        db.execute(self.table_insert_text(mod_name).as_str(), params![links[0], links[1], value])
    }
}

// used in INFO
impl SqlJoinInfo for Filter {
    fn table_name(&self) -> &'static str {
        "JOIN_Filter"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(info_id) REFERENCES INFO(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("info_id", "TEXT NOT NULL"), //FK
            // this
            ("index", "INTEGER"),
            ("filter_type", "TEXT"), //enum
            ("function", "TEXT"),    //enum
            ("comparison", "TEXT"),  //enum
            ("id", "TEXT"),
            ("value", "TEXT"), // union
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![
                links[0],
                self.index,
                as_enum!(self.filter_type),
                as_enum!(self.function),
                as_enum!(self.comparison),
                self.id,
                as_json!(self.value)
            ],
        )
    }
}

// used in FACT
impl SqlJoinInfo for FactionReaction {
    fn table_name(&self) -> &'static str {
        "JOIN_FactionReaction"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(fact_id) REFERENCES FACT(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("fact_id", "TEXT NOT NULL"), //FK
            // this
            ("faction", "TEXT"), //FK
            ("reaction", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![links[0], self.faction, self.reaction],
        )
    }
}

// used in FACT
impl SqlJoinInfo for FactionRequirement {
    fn table_name(&self) -> &'static str {
        "JOIN_FactionRequirement"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(fact_id) REFERENCES FACT(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("fact_id", "TEXT NOT NULL"), //FK
            // this
            ("attributes", "TEXT"), //format
            ("primary_skill", "INTEGER"),
            ("favored_skill", "INTEGER"),
            ("reputation", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![
                links[0],
                as_sql!(self.attributes),
                self.primary_skill,
                self.favored_skill,
                self.reputation
            ],
        )
    }
}

// used in ARMO, CLOT
impl SqlJoinInfo for BipedObject {
    fn table_name(&self) -> &'static str {
        "JOIN_BipedObject"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![
            "FOREIGN KEY(armo_id) REFERENCES ARMO(id)",
            "FOREIGN KEY(clot_id) REFERENCES CLOT(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("armo_id", "TEXT"), //FK
            ("clot_id", "TEXT"), //FK
            // this
            ("biped_object_type", "TEXT"), //enum
            ("male_bodypart", "TEXT"),     //fk
            ("female_bodypart", "TEXT"),   //fk
        ]
    }

    // used in ARMO, CLOT
    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![
                links[0],
                links[1],
                as_enum!(self.biped_object_type),
                self.male_bodypart,
                self.female_bodypart
            ],
        )
    }
}

impl SqlJoinInfo for Effect {
    fn table_name(&self) -> &'static str {
        "JOIN_Effect"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![
            "FOREIGN KEY(spell_id) REFERENCES SPEL(id)",
            "FOREIGN KEY(ench_id) REFERENCES ENCH(id)",
            "FOREIGN KEY(alch_id) REFERENCES ALCH(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("spell_id", "TEXT"), //FK
            ("ench_id", "TEXT"),  //FK
            ("alch_id", "TEXT"),  //FK
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

    // used in SPELL, ENCH, ALCH
    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![
                links[0],
                links[1],
                links[2],
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

// used in CELL
impl SqlJoinInfo for Reference {
    fn table_name(&self) -> &'static str {
        "JOIN_Reference"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(cell_id) REFERENCES CELL(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("cell_id", "TEXT"), //FK
            ("grid", "TEXT"),
            // this
            ("mast_index", "INREGER"),
            ("refr_index", "INTEGER"),
            ("temporary", "INTEGER"), //bool
            ("translation", "TEXT"),  //format
            ("rotation", "TEXT"),     //format
            ("scale", "REAL"),
            ("moved_cell", "TEXT"),    //format
            ("owner", "TEXT"),         //FK
            ("owner_global", "TEXT"),  //FK
            ("owner_faction", "TEXT"), //FK
            ("owner_faction_rank", "INTEGER"),
            ("charge_left", "INTEGER"),
            ("health_left", "INTEGER"),
            ("object_count", "INTEGER"),
            ("destination", "TEXT"), //enum
            ("lock_level", "INTEGER"),
            ("key", "TEXT"),
            ("trap", "TEXT"),
            ("soul", "TEXT"),
            ("blocked", "INTEGER"),
            ("deleted", "INTEGER"), //bool
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        db.execute(
            self.table_insert_text(mod_name).as_str(),
            params![
                links[0],
                links[1],
                self.mast_index,
                self.refr_index,
                self.temporary,
                as_sql!(self.translation),
                as_sql!(self.rotation),
                self.scale,
                as_sql!(self.moved_cell),
                self.owner,
                self.owner_global,
                self.owner_faction,
                self.owner_faction_rank,
                self.charge_left,
                self.health_left,
                self.object_count,
                as_enum!(self.destination),
                self.lock_level,
                self.key,
                self.trap,
                self.soul,
                self.blocked,
                self.deleted,
            ],
        )
    }
}
