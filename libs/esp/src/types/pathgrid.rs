// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PathGrid {
    pub flags: ObjectFlags,
    pub cell: String,
    pub data: PathGridData,
    pub points: Vec<PathGridPoint>,
    pub connections: Vec<u32>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct PathGridData {
    pub grid: (i32, i32),
    pub granularity: u16,
    pub point_count: u16,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PathGridPoint {
    pub location: [i32; 3],
    pub auto_generated: u8,
    pub connection_count: u8,
}

impl Load for PathGrid {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.cell = stream.load()?;
                }
                b"DATA" => {
                    stream.expect(12u32)?;
                    this.data = stream.load()?;
                }
                b"PGRP" => {
                    let len: u32 = stream.load()?;
                    this.points = (0..len / 16).load(|_| stream.load())?;
                }
                b"PGRC" => {
                    let len: u32 = stream.load()?;
                    this.connections = (0..len / 4).load(|_| stream.load())?;
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

impl Save for PathGrid {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&12u32)?;
        stream.save(&self.data)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.cell)?;
        // PGRP
        if !self.points.is_empty() {
            stream.save(b"PGRP")?;
            stream.save_as::<u32>(self.points.len() * 16)?;
            for value in &self.points {
                stream.save(value)?;
            }
        }
        // PGRC
        if !self.connections.is_empty() {
            stream.save(b"PGRC")?;
            stream.save_as::<u32>(self.connections.len() * 4)?;
            for value in &self.connections {
                stream.save(value)?;
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

impl Load for PathGridPoint {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let location = stream.load()?;
        let auto_generated = stream.load()?;
        let connection_count = stream.load()?;
        stream.skip(2)?; // padding
        Ok(Self {
            location,
            auto_generated,
            connection_count,
        })
    }
}

impl Save for PathGridPoint {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.location)?;
        stream.save(&self.auto_generated)?;
        stream.save(&self.connection_count)?;
        stream.save(&[0u8; 2])?; // padding
        Ok(())
    }
}

impl SqlInfo for PathGrid {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("cell", "TEXT"), //FK
            ("grid", "TEXT"), //format
            ("granularity", "INTEGER"),
            ("point_count", "INTEGER"),
            ("points", "TEXT"),      //array
            ("connections", "TEXT"), //array
        ]
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        vec![
        //"FOREIGN KEY(cell) REFERENCES CELL(id)"
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        let sql = as_tes3.table_insert_text(mod_name);
        db.execute(
            sql.as_str(),
            params![
                as_option!(self.cell),
                as_sql!(self.data.grid),
                self.data.granularity,
                self.data.point_count,
                as_json!(self.points),
                as_json!(self.connections),
            ],
        )
    }
}
