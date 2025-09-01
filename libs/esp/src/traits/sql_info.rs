use rusqlite::CachedStatement;

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

#[derive(Debug)]
pub struct JoinTableSchema {
    pub name: &'static str,
    pub columns: Vec<String>,
    pub parent_constraints: Vec<&'static str>,
    pub constraints: Vec<&'static str>,
}

pub trait SqlInfoMeta {
    fn table_name(&self) -> &'static str;
    fn table_schema(&self) -> TableSchema;
    fn get_insert_schema(&self) -> String;
    fn get_join_insert_schema(&self) -> String;
}

pub trait SqlInfo {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)>;
    fn table_constraints(&self) -> Vec<&'static str> {
        vec![]
    }
    fn insert_sql_record(&self, mod_name: &str, s: &mut CachedStatement<'_>) -> rusqlite::Result<usize>;
    fn insert_join_sql_record(&self, _mod_name: &str, _s: &mut CachedStatement<'_>) -> rusqlite::Result<usize> {
        Ok(0)
    }
}

pub trait SqlJoinInfo {
    fn table_name(&self) -> &'static str;
    fn table_columns(&self) -> Vec<(&'static str, &'static str)>;
    fn table_constraints(&self) -> Vec<&'static str> {
        vec![]
    }
    fn table_parent_constraints(&self) -> Vec<&'static str>;
    fn table_insert(&self, s: &mut CachedStatement<'_>, mod_name: &str, links: &[&dyn ToSql]) -> rusqlite::Result<usize>;

    fn table_schema(&self) -> JoinTableSchema {
        JoinTableSchema {
            name: self.table_name(),
            columns: self
                .table_columns()
                .iter()
                .map(|(name, ty)| format!("{} {}", name, ty))
                .collect::<Vec<_>>(),
            constraints: self.table_constraints(),
            parent_constraints: self.table_parent_constraints(),
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
    fn insert_sql_record(&self, mod_name: &str, s: &mut CachedStatement<'_>) -> rusqlite::Result<usize> {
        delegate! {
            match self {
                inner => inner.insert_sql_record(mod_name, s)
            }
        }
    }
    fn insert_join_sql_record(&self, mod_name: &str, s: &mut CachedStatement<'_>) -> rusqlite::Result<usize> {
        delegate! {
            match self {
                inner => inner.insert_join_sql_record(mod_name, s)
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

    fn get_insert_schema(&self) -> String {
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

        let str = format!(
            "REPLACE INTO {}
            (
            id, mod, flags, {}
            ) 
            VALUES
            (
            ?1, ?2, ?3, {}
            )",
            self.table_name(),
            variable_names,
            param_names
        );

        str
    }

    fn get_join_insert_schema(&self) -> String {
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

        let str = format!(
            "REPLACE INTO {}
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

        str
    }
}
