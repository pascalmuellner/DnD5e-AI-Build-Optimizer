use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs::File, io::BufReader};

use crate::{unit::DieType, Item};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub item: Item,
    pub damage_die: DieType,
    pub damage_die_count: i32,
    pub range: i32,
    pub size: WeaponSize,
}

impl Weapon {
    pub fn new(item: Item, damage_die: DieType, damage_die_count: i32, range: i32, size: WeaponSize) -> Self {
        Self { item, damage_die, damage_die_count, range, size }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WeaponList {
    pub weapons: Vec<Weapon>,
}

impl WeaponList {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let weapon_list: Vec<Weapon> = serde_json::from_reader(reader).expect("could not read");
        Self{weapons: weapon_list}
    }
    pub fn get_weapon(&self, item_id: i32) -> Option<Weapon> {
        for weapon in self.weapons.clone() {
            if weapon.item.id == item_id {
                return Some(weapon);
            }
        }
        return None;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum WeaponSize {
    OneHanded,
    TwoHanded,
}