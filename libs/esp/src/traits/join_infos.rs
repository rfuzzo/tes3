use crate::prelude::*;

// used in RACE, BSGN, CREA, NPC_
// depends on SPEL
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SpellJoin {
    pub spell_id: String,
}
impl SqlJoinInfo for SpellJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_Spells"
    }
    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(race_id) REFERENCES RACE(id)",
            "FOREIGN KEY(bsgn_id) REFERENCES BSGN(id)",
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NPC_(id)",
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(spell_id) REFERENCES SPEL(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("race_id", "TEXT"), //FK
            ("bsgn_id", "TEXT"), //FK
            ("crea_id", "TEXT"), //FK
            ("npc_id", "TEXT"),  //FK
            // this
            ("spell_id", "TEXT"), //FK
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(db, mod_name, params![links[0], links[1], links[2], links[3], self.spell_id])
    }
}

// used in REGN
// depends on SOUN
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SoundJoin {
    pub sound_id: String,
}
impl SqlJoinInfo for SoundJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_Sounds"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(regn_id) REFERENCES REGN(id)"]
    }
    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(sound_id) REFERENCES SOUN(id)"]
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
        self.table_insert2(db, mod_name, params![links[0], self.sound_id])
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
        "JOIN_Inventory"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(cont_id) REFERENCES CONT(id)",
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NPC_(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("cont_id", "TEXT"), //FK
            ("crea_id", "TEXT"), //FK
            ("npc_id", "TEXT"),  //FK
            // this
            ("idx", "INTEGER"),
            ("item_id", "TEXT"), //FK TODO
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(db, mod_name, params![links[0], links[1], links[2], self.index, self.item_id])
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

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(levi_id) REFERENCES LEVI(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("levi_id", "TEXT"), //FK
            // this
            ("item_id", "TEXT"), //FK TODO
            ("probability", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(db, mod_name, params![links[0], self.item_id, self.probability])
    }
}

// used in LEVC
// depends on CREA or NPC_ or another list //TODO
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CreatureJoin {
    pub creature_id: String,
    pub probability: u16,
}
impl SqlJoinInfo for CreatureJoin {
    fn table_name(&self) -> &'static str {
        "JOIN_Creature"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(levc_id) REFERENCES LEVC(id)"]
    }
    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            //"FOREIGN KEY(creature_id) REFERENCES CREA(id)", // can also be NPC_
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("levc_id", "TEXT"), //FK
            // this
            ("creature_id", "TEXT"), //FK
            ("probability", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(db, mod_name, params![links[0], self.creature_id, self.probability])
    }
}

// used in CREA, NPC_
impl SqlJoinInfo for TravelDestination {
    fn table_name(&self) -> &'static str {
        "JOIN_TravelDestination"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NPC_(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("crea_id", "TEXT"), //FK
            ("npc_id", "TEXT"),  //FK
            // this
            ("translation", "TEXT"),
            ("rotation", "TEXT"),
            ("cell", "TEXT"), //FK TODO
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(
            db,
            mod_name,
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

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(crea_id) REFERENCES CREA(id)",
            "FOREIGN KEY(npc_id) REFERENCES NPC_(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("crea_id", "TEXT"), //FK
            ("npc_id", "TEXT"),  //FK
            // this
            ("package", "TEXT"), //json TODO
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        let value = serde_json::to_string(&self).unwrap();
        self.table_insert2(db, mod_name, params![links[0], links[1], value])
    }
}

// used in INFO
impl SqlJoinInfo for Filter {
    fn table_name(&self) -> &'static str {
        "JOIN_Filter"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(info_id) REFERENCES INFO(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("info_id", "TEXT"), //FK
            // this
            ("idx", "INTEGER"),
            ("filter_type", "TEXT"), //enum
            ("function", "TEXT"),    //enum
            ("comparison", "TEXT"),  //enum
            ("id", "TEXT"),
            ("value", "TEXT"), // union
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(
            db,
            mod_name,
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
// depends on FACT
impl SqlJoinInfo for FactionReaction {
    fn table_name(&self) -> &'static str {
        "JOIN_FactionReaction"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(fact_id) REFERENCES FACT(id)"]
    }
    fn table_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(faction) REFERENCES FACT(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("fact_id", "TEXT"), //FK
            // this
            ("faction", "TEXT COLLATE NOCASE"), //FK
            ("reaction", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(db, mod_name, params![links[0], self.faction, self.reaction])
    }
}

// used in FACT
impl SqlJoinInfo for FactionRequirement {
    fn table_name(&self) -> &'static str {
        "JOIN_FactionRequirement"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(fact_id) REFERENCES FACT(id)"]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("fact_id", "TEXT"), //FK
            // this
            ("attributes", "TEXT"), //format
            ("primary_skill", "INTEGER"),
            ("favored_skill", "INTEGER"),
            ("reputation", "INTEGER"),
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(
            db,
            mod_name,
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
// depends on BODY
impl SqlJoinInfo for BipedObject {
    fn table_name(&self) -> &'static str {
        "JOIN_BipedObject"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![
            "FOREIGN KEY(armo_id) REFERENCES ARMO(id)",
            "FOREIGN KEY(clot_id) REFERENCES CLOT(id)",
        ]
    }
    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![
            "FOREIGN KEY(male_bodypart) REFERENCES BODY(id)",
            "FOREIGN KEY(female_bodypart) REFERENCES BODY(id)",
        ]
    }

    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // parents
            ("armo_id", "TEXT"), //FK
            ("clot_id", "TEXT"), //FK
            // this
            ("biped_object_type", "TEXT"),              //enum
            ("male_bodypart", "TEXT COLLATE NOCASE"),   //fk
            ("female_bodypart", "TEXT COLLATE NOCASE"), //fk
        ]
    }

    // used in ARMO, CLOT
    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        self.table_insert2(
            db,
            mod_name,
            params![
                links[0],
                links[1],
                as_enum!(self.biped_object_type),
                as_option!(self.male_bodypart),
                as_option!(self.female_bodypart)
            ],
        )
    }
}

// used in SPEL, ENCH, ALCH
impl SqlJoinInfo for Effect {
    fn table_name(&self) -> &'static str {
        "JOIN_Effect"
    }

    fn table_parent_constraints(&self) -> Vec<&'static str> {
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
        self.table_insert2(
            db,
            mod_name,
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

    fn table_parent_constraints(&self) -> Vec<&'static str> {
        vec!["FOREIGN KEY(cell_id) REFERENCES CELL(id)"]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
            "FOREIGN KEY(owner) REFERENCES NPC_(id)",
            "FOREIGN KEY(owner_global) REFERENCES GLOB(id)",
            "FOREIGN KEY(owner_faction) REFERENCES FACT(id)",
        ]
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
        let owner = if let Some(owner) = &self.owner { owner } else { "" };
        let owner_global = if let Some(owner_global) = &self.owner_global {
            owner_global
        } else {
            ""
        };
        let owner_faction = if let Some(owner_faction) = &self.owner_faction {
            owner_faction
        } else {
            ""
        };

        self.table_insert2(
            db,
            mod_name,
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
                as_option!(owner),
                as_option!(owner_global),
                as_option!(owner_faction),
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
