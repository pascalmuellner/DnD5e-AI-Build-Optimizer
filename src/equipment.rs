use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::unit::DieType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Equipment {
    pub armor: ArmorSlots,
    pub melee_weapon: Option<Weapon>,
    pub ranged_weapon: Option<Weapon>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Armor {
    pub name: String,
    pub armor_class: i32,
    pub armor_type: Option<ArmorType>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub name: String,
    pub damage_die: DieType,
    pub damage_die_count: i32,
    pub range: i32,
    pub size: WeaponSize,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum WeaponSize {
    OneHanded,
    TwoHanded,
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