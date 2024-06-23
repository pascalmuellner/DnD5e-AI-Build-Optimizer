use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

impl Item {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ItemList{
    pub items: Vec<Item>,
}

impl ItemList {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let item_list: Vec<Item> = serde_json::from_reader(reader).expect("could not read");
        Self{items: item_list}
    }
    pub fn get_item(&self, item_id: i32) -> Option<Item> {
        for item in self.items.clone() {
            if item.id == item_id {
                return Some(item);
            }
        }
        return None;
    }
}