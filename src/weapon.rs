use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs::File, io::BufReader, io::BufWriter, io::Write};

use crate::{unit::DieType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub item_id: i32,
    pub item_name: String,
    pub damage_die: DieType,
    pub damage_die_count: i32,
    pub damage_type: DamageType,
    pub properties: Vec<Properties>,
}

impl Weapon {
    /// Creates a new [`Weapon`].
    pub fn new(item_id: i32, item_name: String, damage_die: DieType, damage_die_count: i32, damage_type: DamageType, properties: Vec<Properties>) -> Self {
        Self { item_id, item_name, damage_die, damage_die_count, damage_type, properties }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WeaponList {
    pub weapons: Vec<Weapon>,
}

impl WeaponList {
    /// Creates a new [`WeaponList`] from a json file.
    ///
    /// # Panics
    ///
    /// Panics if it can't open a file or could not correctly generate the weapon list.
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let weapon_list: Vec<Weapon> = serde_json::from_reader(reader).expect("could not read");
        Self{weapons: weapon_list}
    }
    /// Adds a new weapon to the weapon list
    pub fn add(&mut self, weapon: Weapon) {
        self.weapons.push(weapon);
    }
    pub fn get_weapon(&self, item_id: i32) -> Option<Weapon> {
        for weapon in self.weapons.clone() {
            if weapon.item_id == item_id {
                return Some(weapon);
            }
        }
        return None;
    }

    /// Writes the weapon list to a json.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn write(&self, file_path: &str) -> std::io::Result<()>{
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.weapons)?;
        writer.flush()?;
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Properties {
    Ammunition(Ammunition),
    Finesse,
    Heavy,
    Light,
    Loading,
    Range((i32, i32)),
    Reach,
    Special,
    Thrown((i32, i32)),
    TwoHanded,
    Versatile,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Ammunition {

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
}