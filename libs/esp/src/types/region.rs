// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Region {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub weather_chances: WeatherChances,
    pub sleep_creature: String,
    pub map_color: [u8; 4],
    pub sounds: Vec<(FixedString<32>, u8)>,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WeatherChances {
    pub clear: u8,
    pub cloudy: u8,
    pub foggy: u8,
    pub overcast: u8,
    pub rain: u8,
    pub thunder: u8,
    pub ash: u8,
    pub blight: u8,
    pub snow: u8,
    pub blizzard: u8,
}

impl Load for Region {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"WEAT" => {
                    this.weather_chances = stream.load()?;
                }
                b"BNAM" => {
                    this.sleep_creature = stream.load()?;
                }
                b"CNAM" => {
                    stream.expect(4u32)?;
                    this.map_color = stream.load()?;
                }
                b"SNAM" => {
                    stream.expect(33u32)?;
                    this.sounds.push(stream.load()?);
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

impl Save for Region {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // WEAT
        stream.save(b"WEAT")?;
        stream.save(&self.weather_chances)?;
        // BNAM
        if !self.sleep_creature.is_empty() {
            stream.save(b"BNAM")?;
            stream.save(&self.sleep_creature)?;
        }
        // CNAM
        stream.save(b"CNAM")?;
        stream.save(&4u32)?;
        stream.save(&self.map_color)?;
        // SNAM
        for (sound, chance) in &self.sounds {
            stream.save(b"SNAM")?;
            stream.save(&33u32)?;
            stream.save(sound)?;
            stream.save(chance)?;
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

impl Load for WeatherChances {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len: u32 = stream.load()?;
        assert!((len == 8 || len == 10), "Unexpected size ({len}) for REGN::WEAT");
        let clear = stream.load()?;
        let cloudy = stream.load()?;
        let foggy = stream.load()?;
        let overcast = stream.load()?;
        let rain = stream.load()?;
        let thunder = stream.load()?;
        let ash = stream.load()?;
        let blight = stream.load()?;
        let snow = if len == 10 { stream.load()? } else { 0 };
        let blizzard = if len == 10 { stream.load()? } else { 0 };
        Ok(Self {
            clear,
            cloudy,
            foggy,
            overcast,
            rain,
            thunder,
            ash,
            blight,
            snow,
            blizzard,
        })
    }
}

impl Save for WeatherChances {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&10u32)?;
        stream.save(&self.clear)?;
        stream.save(&self.cloudy)?;
        stream.save(&self.foggy)?;
        stream.save(&self.overcast)?;
        stream.save(&self.rain)?;
        stream.save(&self.thunder)?;
        stream.save(&self.ash)?;
        stream.save(&self.blight)?;
        stream.save(&self.snow)?;
        stream.save(&self.blizzard)?;
        Ok(())
    }
}

impl SqlInfo for Region {
    fn table_columns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "TEXT"),
            ("clear", "INTEGER"),
            ("cloudy", "INTEGER"),
            ("foggy", "INTEGER"),
            ("overcast", "INTEGER"),
            ("rain", "INTEGER"),
            ("thunder", "INTEGER"),
            ("ash", "INTEGER"),
            ("blight", "INTEGER"),
            ("snow", "INTEGER"),
            ("blizzard", "INTEGER"),
            ("sleep_creature", "TEXT"),
            ("map_color", "TEXT"), //color
            ("sounds", "TEXT"),    //aray
        ]
    }

    fn table_insert(&self, db: &Connection, mod_name: &str) -> rusqlite::Result<usize> {
        let as_tes3: TES3Object = self.clone().into();
        let sql = as_tes3.table_insert_text(mod_name);
        db.execute(
            sql.as_str(),
            params![
                self.name,
                self.weather_chances.clear,
                self.weather_chances.cloudy,
                self.weather_chances.foggy,
                self.weather_chances.overcast,
                self.weather_chances.rain,
                self.weather_chances.thunder,
                self.weather_chances.ash,
                self.weather_chances.blight,
                self.weather_chances.snow,
                self.weather_chances.blizzard,
                self.sleep_creature,
                as_json!(self.map_color),
                as_json!(self.sounds)
            ],
        )
    }
}
