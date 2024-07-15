use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt;
use std::fmt::Display;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
use vizia::prelude::*;

use crate::unit_derived_lenses::species;
use crate::StatBlock;

#[derive(Data, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Species {
    pub species_id: i32,
    pub species_name: String,
    pub stats_increase: StatBlock,
}

impl Species {
    /// Creates a new [`Species`].
    pub fn new(species_id: i32, species_name: String, stats_increase: StatBlock) -> Self {
        Self {
            species_id,
            species_name,
            stats_increase,
        }
    }
}

impl Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.species_name)
    }
}

#[derive(Data, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SpeciesList {
    pub species: Vec<Species>,
}

impl SpeciesList {
    /// Creates a new [`SpeciesList`] from a json file.
    ///
    /// # Panics
    ///
    /// Panics if it can't open a file or could not correctly generate the species list.
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let species_list: Vec<Species> = serde_json::from_reader(reader).expect("could not read");
        Self {
            species: species_list,
        }
    }
    /// Adds a new species to the species list
    pub fn add(&mut self, speciesa: Species) {
        self.species.push(speciesa);
    }
    pub fn get_species(&self, species_id: i32) -> Option<Species> {
        for speciesa in self.species.clone() {
            if speciesa.species_id == species_id {
                return Some(speciesa);
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
        serde_json::to_writer(&mut writer, &self.species)?;
        writer.flush()?;
        Ok(())
    }
}
