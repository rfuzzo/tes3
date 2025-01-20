use crate::prelude::*;

#[macro_export]
macro_rules! as_color {
    ( $x:expr ) => {
        serde_json::to_string_pretty(&$x).unwrap()
    };
}

#[macro_export]
macro_rules! as_option {
    ( $x:expr ) => {
        if $x.is_empty() {
            None
        } else {
            Some($x.to_owned())
        }
    };
}

#[macro_export]
macro_rules! as_json {
    ( $x:expr ) => {
        serde_json::to_string_pretty(&$x).unwrap()
    };
}

#[macro_export]
macro_rules! as_enum {
    ( $x:expr ) => {
        format!("{:?}", $x)
    };
}

#[macro_export]
macro_rules! as_flags {
    ( $x:expr ) => {
        format!("{:?}", $x)
    };
}

#[macro_export]
macro_rules! as_sql {
    ( $x:expr ) => {
        format!("{:?}", $x)
    };
}

#[derive(Debug)]
pub struct TableSchema {
    pub name: &'static str,
    pub columns: Vec<String>,
    pub constraints: Vec<&'static str>,
}
pub trait SqlInfoMeta {
    fn table_name(&self) -> &'static str;
    fn table_schema(&self) -> TableSchema;

    fn table_insert2(&self, db: &Connection, mod_name: &str, params: &[&dyn ToSql]) -> rusqlite::Result<usize>;
}

pub trait SqlInfo {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)>;
    fn table_constraints(&self) -> Vec<&'static str> {
        vec![]
    }
    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize>;
    fn join_table_insert(&self, _db: &Connection, _mod_name: &str) -> rusqlite::Result<usize> {
        Ok(0)
    }
}

pub trait SqlJoinInfo {
    fn table_name(&self) -> &'static str;
    fn table_columns(&self) -> Vec<(&'static str, &'static str)>;
    fn table_constraints(&self) -> Vec<&'static str> {
        vec![]
    }
    fn table_insert(&self, db: &Connection, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize>;

    fn table_insert2(&self, db: &Connection, mod_name: &str, params: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        let variable_names = self
            .table_columns()
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let param_names = self
            .table_columns()
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 2))
            .collect::<Vec<_>>()
            .join(", ");

        let str = format!(
            "INSERT INTO {}
            (
            mod, {}
            ) 
            VALUES
            (
            ?1, {}
            )",
            self.table_name(),
            variable_names,
            param_names,
        );

        let mut final_params = params![mod_name].to_vec();
        final_params.extend_from_slice(params);

        db.execute(str.as_str(), final_params.as_slice())
    }

    fn table_schema(&self) -> TableSchema {
        TableSchema {
            name: self.table_name(),
            columns: self
                .table_columns()
                .iter()
                .map(|(name, ty)| format!("{} {}", name, ty))
                .collect::<Vec<_>>(),
            constraints: self.table_constraints(),
        }
    }
}

impl SqlInfo for TES3Object {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        delegate! {
            match self {
                inner => inner.table_columns()
            }
        }
    }
    fn table_constraints(&self) -> Vec<&'static str> {
        delegate! {
            match self {
                inner => inner.table_constraints()
            }
        }
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        delegate! {
            match self {
                inner => inner.table_insert(db, mod_name)
            }
        }
    }

    fn join_table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        delegate! {
            match self {
                inner => inner.join_table_insert(db, mod_name)
            }
        }
    }
}

impl SqlInfoMeta for TES3Object {
    fn table_name(&self) -> &'static str {
        self.tag_str()
    }

    fn table_schema(&self) -> TableSchema {
        TableSchema {
            name: self.table_name(),
            columns: self
                .table_columns()
                .iter()
                .map(|(name, ty)| format!("{} {}", name, ty))
                .collect::<Vec<_>>(),
            constraints: self.table_constraints(),
        }
    }

    fn table_insert2(&self, db: &Connection, mod_name: &str, params: &[&dyn ToSql]) -> rusqlite::Result<usize> {
        let variable_names = self
            .table_columns()
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let param_names = self
            .table_columns()
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 4))
            .collect::<Vec<_>>()
            .join(", ");

        let as_tes3: TES3Object = self.clone().into();
        let str = format!(
            "INSERT INTO {}
            (
            id, mod, flags, {}
            ) 
            VALUES
            (
            ?1, ?2, ?3, {}
            )",
            as_tes3.table_name(),
            variable_names,
            param_names
        );

        let id = self.editor_id();
        let flags = as_flags!(self.object_flags());
        let mut final_params = params![id, mod_name, flags].to_vec();
        final_params.extend_from_slice(params);

        db.execute(str.as_str(), final_params.as_slice())
    }
}
