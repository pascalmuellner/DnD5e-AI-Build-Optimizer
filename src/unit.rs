use num::Integer;
use std::collections::HashMap;
use rand::Rng;
use vizia::prelude::*;

use crate::equipment::{Armor, ArmorSlot, ArmorType, Equipment, Weapon, WeaponSize};

#[derive(Lens, Debug, PartialEq, Eq, Clone)]
pub struct Unit {
    pub unit_type: UnitType,
    pub character_name: String,
    pub character_level: i32,
    pub hitpoints: i32,
    pub hitpoints_type: HitpointsType,
    pub movement_speed: i32,
    pub starting_class: Option<Class>,
    pub additional_classes: Vec<Class>,
    pub action_count: i32,
    pub stats: StatBlock,
    pub equipment: Equipment,
}

impl Unit {
    pub fn create_player_character(name: String, class: Class, stats: StatBlock, hitpointstype: HitpointsType, equip: Equipment) -> Self {
        let hp: i32;
        if hitpointstype == HitpointsType::Random {
            hp = get_random_dice_value(class.hit_die);
        } else {
            hp = get_average_dice_value(class.hit_die);
        }

        let character = Unit {
            unit_type: UnitType::Player,
            character_name: name,
            character_level: class.class_level,
            hitpoints: hp,
            hitpoints_type: hitpointstype,
            movement_speed: 30,
            starting_class: Some(class),
            additional_classes: Vec::new(),
            action_count: 1,
            stats,
            equipment: equip
        };
        return character;
    }
    pub fn create_goblin() -> Unit{
        let dagger = Weapon{
            name: "Dagger".to_string(),
            damage_die: DieType::D6,
            damage_die_count: 1,
            range: 5,
            size: WeaponSize::OneHanded,
        };
        let chest = Armor{
            name: "Leather Armor".to_string(),
            armor_class: 11,
            armor_type: Some(ArmorType::Light),
        };
        let shield = Armor{
            name: "Shield".to_string(),
            armor_class: 2,
            armor_type: Some(ArmorType::Shield),
        };
        let mut armor: HashMap<ArmorSlot, Armor> = HashMap::new();
        armor.insert(ArmorSlot::Chest, chest);
        armor.insert(ArmorSlot::Shield, shield);
        let equip = Equipment{
            armor,
            melee_weapon: Some(dagger),
            ranged_weapon: None,
        };
        Unit {
            unit_type: UnitType::Enemy,
            character_name: "Goblin".to_string(),
            character_level: 1,
            hitpoints: 8,
            hitpoints_type: HitpointsType::Average,
            movement_speed: 20,
            starting_class: None,
            additional_classes: Vec::new(),
            action_count: 1,
            stats: StatBlock::new(8, 14, 10, 10, 8, 8),
            equipment: equip
        }
    }
    /// level up a class for the character
    pub fn level_up(&mut self, class: Class) {
        // check if the class is the starting class
        if self.starting_class.unwrap().class_name == class.class_name {
            self.starting_class.unwrap().level_up();
        }
        // check if the class is a multi class and get the index if it is
        else if let Some(index) = self
            .additional_classes
            .iter()
            .position(|i| i.class_name == class.class_name)
        {
            self.additional_classes[index].level_up();
        }
        // add the class as multi class
        else {
            self.add_additional_class(class);
        }

        // also update character level and hit points
        self.character_level += 1;
        if self.hitpoints_type == HitpointsType::Random {
            self.hitpoints += get_random_dice_value(class.hit_die);
        } else {
            self.hitpoints += get_average_dice_value(class.hit_die);
        }
    }
    pub fn melee_attack(&mut self, target: &mut Unit){
        if self.action_count > 0 {
            self.action_count -= 1;
            let damage_value = get_random_dice_value(DieType::D6);
            println!("Damage: {}", damage_value);
            target.hitpoints -= damage_value;
        }
        else {
            print!("No action left!");
        }
    }
    pub fn calculate_initiative(&self) -> i32 {
        get_random_dice_value(DieType::D20)
    }
    fn add_additional_class(&mut self, class: Class) {
        self.additional_classes.push(class);
    }
}

#[derive(Lens, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Class {
    pub class_name: ClassName,
    pub hit_die: DieType,
    pub class_level: i32,
}

impl Class {
    pub fn level_up(&mut self) {
        self.class_level += 1;
    }
    pub fn create_artificer() -> Self {
        let class = Class {
            class_name: ClassName::Artificer,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_barbarian() -> Self {
        let class = Class {
            class_name: ClassName::Barbarian,
            hit_die: DieType::D12,
            class_level: 1,
        };
        return class;
    }
    pub fn create_bard() -> Self {
        let class = Class {
            class_name: ClassName::Bard,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_cleric() -> Self {
        let class = Class {
            class_name: ClassName::Cleric,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_druid() -> Self {
        let class = Class {
            class_name: ClassName::Druid,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_fighter() -> Self {
        let class = Class {
            class_name: ClassName::Fighter,
            hit_die: DieType::D10,
            class_level: 1,
        };
        return class;
    }
    pub fn create_monk() -> Self {
        let class = Class {
            class_name: ClassName::Monk,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_paladin() -> Self {
        let class = Class {
            class_name: ClassName::Paladin,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_ranger() -> Self {
        let class = Class {
            class_name: ClassName::Ranger,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_rogue() -> Self {
        let class = Class {
            class_name: ClassName::Rogue,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_sorcerer() -> Self {
        let class = Class {
            class_name: ClassName::Sorcerer,
            hit_die: DieType::D6,
            class_level: 1,
        };
        return class;
    }
    pub fn create_warlock() -> Self {
        let class = Class {
            class_name: ClassName::Warlock,
            hit_die: DieType::D8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_wizard() -> Self {
        let class = Class {
            class_name: ClassName::Wizard,
            hit_die: DieType::D6,
            class_level: 1,
        };
        return class;
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatBlock {
    pub intelligence: i32,
    pub constitution: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub wisdom: i32,
    pub charisma: i32,
}
impl StatBlock {
    pub fn new(strength: i32, dexterity: i32, constitution: i32, intelligence: i32, wisdom: i32, charisma: i32) -> Self {
        StatBlock{
            intelligence,
            constitution,
            strength,
            dexterity,
            wisdom,
            charisma,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitpointsType {
    Random,
    Average,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassName {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitType {
    Player,
    Enemy,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyType {
    Humanoid,
    Insect,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HumanoidType{
    Goblin,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DieType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

fn get_random_dice_value(die: DieType) -> i32 {
    match die {
        DieType::D4 => rand::thread_rng().gen_range(1..=4),
        DieType::D6 => rand::thread_rng().gen_range(1..=6),
        DieType::D8 => rand::thread_rng().gen_range(1..=8),
        DieType::D10 => rand::thread_rng().gen_range(1..=10),
        DieType::D12 => rand::thread_rng().gen_range(1..=12),
        DieType::D20 => rand::thread_rng().gen_range(1..=20),
    }
}

fn get_average_dice_value(die: DieType) -> i32 {
    match die {
        DieType::D4 => average(1..=4, 4),
        DieType::D6 => average(1..=6, 6),
        DieType::D8 => average(1..=8, 8),
        DieType::D10 => average(1..=10, 10),
        DieType::D12 => average(1..=12, 12),
        DieType::D20 => average(1..=20, 20),
    }
}

fn average(numbers: std::ops::RangeInclusive<i32>, count: i32) -> i32 {
    let sum: i32 = numbers.into_iter().sum();
    sum.div_ceil(&count)
}
