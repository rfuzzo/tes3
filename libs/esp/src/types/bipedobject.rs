// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BipedObject {
    pub biped_object_type: BipedObjectType,
    pub male_bodypart: String,
    pub female_bodypart: String,
}

impl Load for BipedObject {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();
        // INDX
        stream.expect(1u32)?;
        this.biped_object_type = stream.load()?;
        //
        for _ in 0..2 {
            // BNAM
            if stream.expect(*b"BNAM").is_ok() {
                this.male_bodypart = stream.load()?;
                continue;
            }
            // CNAM
            if stream.expect(*b"CNAM").is_ok() {
                this.female_bodypart = stream.load()?;
                continue;
            }
        }
        Ok(this)
    }
}

impl Save for BipedObject {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        // INDX
        stream.save(b"INDX")?;
        stream.save(&1u32)?;
        stream.save(&self.biped_object_type)?;
        // BNAM
        if !self.male_bodypart.is_empty() {
            stream.save(b"BNAM")?;
            stream.save(&self.male_bodypart)?;
        }
        // CNAM
        if !self.female_bodypart.is_empty() {
            stream.save(b"CNAM")?;
            stream.save(&self.female_bodypart)?;
        }
        Ok(())
    }
}

impl SqlJoinInfo for BipedObject {
    fn table_name(&self) -> &'static str {
        "JOIN_BipedObject"
    }

    fn table_constraints(&self) -> Vec<&'static str> {
        // TODO add unique constraint
        vec![]
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
