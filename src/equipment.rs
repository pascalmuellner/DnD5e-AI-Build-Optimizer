use std::collections::HashMap;

use crate::unit::DieType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Equipment {
    pub armor: HashMap<ArmorSlot, Armor>,
    pub melee_weapon: Option<Weapon>,
    pub ranged_weapon: Option<Weapon>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Armor {
    pub name: String,
    pub armor_class: i32,
    pub armor_type: Option<ArmorType>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub name: String,
    pub damage_die: DieType,
    pub damage_die_count: i32,
    pub range: i32,
    pub size: WeaponSize,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WeaponSize {
    OneHanded,
    TwoHanded,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ArmorSlot {
    Chest,
    Boots,
    Gloves,
    Helmet,
    Shield,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}