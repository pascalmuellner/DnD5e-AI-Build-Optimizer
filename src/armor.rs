use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs::File, io::BufReader};

use crate::Item;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Armor {
    pub item_id: i32,
    pub item_name: String,
    pub armor_class: i32,
    pub armor_type: Option<ArmorType>,
}

impl Armor {
    pub fn new(item_id: i32, name: String, armor_class: i32, armor_type: Option<ArmorType>) -> Self {
        Self { item_id, item_name: name, armor_class, armor_type }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ArmorList {
    pub armors: Vec<Armor>,
}

impl ArmorList {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let armor_list: Vec<Armor> = serde_json::from_reader(reader).expect("could not read");
        Self{armors: armor_list}
    }
    pub fn get_armor(&self, item_id: i32) -> Option<Armor> {
        for armor in self.armors.clone() {
            if armor.item_id == item_id {
                return Some(armor);
            }
        }
        return None;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ArmorSlots {
    pub chest: Option<Armor>,
    pub boots: Option<Armor>,
    pub gloves: Option<Armor>,
    pub helmet: Option<Armor>,
    pub shield: Option<Armor>,
}

impl ArmorSlots {
    pub fn new(chest: Option<Armor>, boots: Option<Armor>, gloves: Option<Armor>, helmet: Option<Armor>, shield: Option<Armor>) -> Self {
        Self { chest, boots, gloves, helmet, shield }
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}