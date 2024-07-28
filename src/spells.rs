use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
use vizia::prelude::*;

use crate::{DieType, Unit};

#[derive(Lens, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub typ: SpellType,
    pub range: i32,
    pub duration: SpellDuration,
    pub school: SpellSchool,
    pub damage: Option<DieType>,
    pub casting_time: CastingTime,
}

impl Spell {
    pub fn new(
        id: i32,
        name: String,
        level: i32,
        typ: SpellType,
        range: i32,
        duration: SpellDuration,
        school: SpellSchool,
        damage: Option<DieType>,
        casting_time: CastingTime,
    ) -> Self {
        Self {
            id,
            name,
            level,
            typ,
            range,
            duration,
            school,
            damage,
            casting_time,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SpellList {
    pub spells: Vec<Spell>,
}

impl SpellList {
    /// Creates a new [`SpellList`] from a json file.
    ///
    /// # Panics
    ///
    /// Panics if it can't open a file or could not correctly generate the species list.
    pub fn new(file_path: &str) -> Self {
        if std::path::Path::new(file_path).exists() {
            let file = File::open(file_path).expect("Unable to open file");
            let reader = BufReader::new(file);
            let spell_list: Vec<Spell> = serde_json::from_reader(reader).expect("could not read");
            Self { spells: spell_list }
        } else {
            Self { spells: vec![] }
        }
    }
    /// Adds a new species to the species list
    pub fn add(&mut self, spell: Spell) {
        self.spells.push(spell);
    }
    pub fn get_spell(&self, spell_id: i32) -> Option<Spell> {
        for spell in self.spells.clone() {
            if spell.id == spell_id {
                return Some(spell);
            }
        }
        return None;
    }

    /// Writes the species list to a json.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn write(&self, file_path: &str) -> std::io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.spells)?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum SpellType {
    Ritual,
    Cantrip,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum SpellDuration {
    Instantaneous,
    Concentration,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum CastingTime {
    Action,
    Reaction,
}
