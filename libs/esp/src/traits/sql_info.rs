use crate::prelude::*;

#[macro_export]
macro_rules! as_json {
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
    fn table_insert_text(&self, mod_name: &str) -> String;
    fn table_schema(&self) -> TableSchema;
}

pub trait SqlInfo {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)>;
    fn table_constraints(&self) -> Vec<&'static str> {
        vec![]
    }
    fn table_insert(&self, db: &Connection, name: &str) -> rusqlite::Result<usize>;
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

    fn table_insert(&self, db: &Connection, name: &str) -> rusqlite::Result<usize> {
        delegate! {
            match self {
                inner => inner.table_insert(db, name)
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

    fn table_insert_text(&self, mod_name: &str) -> String {
        let variable_names = self
            .table_columns()
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let params = self
            .table_columns()
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 4))
            .collect::<Vec<_>>()
            .join(", ");

        let str = format!(
            "INSERT INTO {}
            (
            id, mod, flags {}
            ) 
            VALUES
            (
            {}, {}, {}, {}
            )",
            self.table_name(),
            variable_names,
            self.editor_id().to_lowercase(),
            mod_name,
            as_flags!(self.object_flags()),
            params
        );
        str.to_string()
    }
}
