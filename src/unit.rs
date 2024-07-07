use std::fmt;
use std::fmt::Display;
use num::Integer;
use rand::Rng;
use vizia::prelude::*;
use serde::{Deserialize, Serialize};

use crate::equipment::*;
use crate::armor::*;
use crate::weapon::*;
use crate::item::*;

use std::fs::File;
use std::io::BufReader;
use std::vec;

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
        let file = File::open("src/Items/goblin.json").expect("Unable to open file");
        let reader = BufReader::new(file);
        let goblin_gear: Equipment = serde_json::from_reader(reader).expect("could not read");
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
            equipment: goblin_gear
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
            let hit_roll_value = get_random_dice_value(DieType::D20);
            if hit_roll_value + self.stats.get_dexterity_modifier() >= target.calculate_armor_class() {
                let damage_value = get_random_dice_value(DieType::D6);
                println!("Attack hit!");
                println!("Damage: {}", damage_value);
                target.hitpoints -= damage_value;
            }
            else {
                println!("Attack missed!");
            }

        }
        else {
            print!("No action left!");
        }
    }
    pub fn calculate_initiative(&self) -> i32 {
        get_random_dice_value(DieType::D20)
    }
    pub fn calculate_armor_class(&self) -> i32 {
        let mut armor_class = 0;
        let dex_modifier = self.stats.get_dexterity_modifier();
        if self.equipment.armor.chest != None {
            let armor_type = self.equipment.armor.chest.as_ref().unwrap().armor_type.as_ref().unwrap();
            if armor_type == &ArmorType::Light {
                armor_class = dex_modifier;
            }
            else if armor_type == &ArmorType::Medium {
                if dex_modifier > 2 {
                    armor_class = 2;
                }
                else {
                    armor_class = dex_modifier;
                }
            }
            armor_class += self.equipment.armor.chest.as_ref().unwrap().armor_class;
        }
        if self.equipment.armor.shield != None {
            armor_class += self.equipment.armor.shield.as_ref().unwrap().armor_class;
        }
        return armor_class;
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
#[derive(Clone, Lens, Copy, Debug, PartialEq, Eq)]
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
    pub fn get_intelligence_modifier(&self) -> i32 {
        (self.intelligence - 10) % 2
    }
    pub fn get_constitution_modifier(&self) -> i32 {
        (self.constitution - 10) % 2
    }
    pub fn get_strength_modifier(&self) -> i32 {
        (self.strength - 10) % 2
    }
    pub fn get_dexterity_modifier(&self) -> i32 {
        (self.dexterity - 10) % 2
    }
    pub fn get_wisdom_modifier(&self) -> i32 {
        (self.wisdom - 10) % 2
    }
    pub fn get_charisma_modifier(&self) -> i32 {
        (self.charisma - 10) % 2
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitpointsType {
    Random,
    Average,
}
#[derive(Data, Clone, Copy, Debug, PartialEq, Eq)]
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
impl ClassName {
    pub fn get_all_class_names() -> Vec<ClassName> {
        vec![ClassName::Artificer, ClassName::Barbarian, ClassName::Bard, ClassName::Cleric, ClassName::Druid, ClassName::Fighter, ClassName::Monk, ClassName::Paladin, ClassName::Ranger, ClassName::Rogue, ClassName::Sorcerer, ClassName::Warlock, ClassName::Wizard]
    }
}
impl Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
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
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
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
