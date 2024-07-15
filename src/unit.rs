use num::Integer;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::path::PathBuf;
use std::{fs::File, io::BufReader, io::BufWriter, io::Write};
use vizia::prelude::*;

use crate::armor::*;
use crate::class::*;
use crate::equipment::*;
use crate::species::*;
use crate::spells::*;

#[derive(Lens, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
    pub equipment: Option<Equipment>,
    pub species: Option<Species>,
    pub spell_list: Option<Vec<Spell>>,
    pub spell_slots: Option<SpellSlots>,
}

impl Unit {
    pub fn create_player_character(
        name: String,
        class: Class,
        stats: StatBlock,
        hitpointstype: HitpointsType,
        equip: Option<Equipment>,
        species: Option<Species>,
        spell_list: Option<Vec<Spell>>,
        spell_slots: Option<SpellSlots>,
    ) -> Self {
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
            equipment: equip,
            species: species,
            spell_list: spell_list,
            spell_slots: spell_slots,
        };

        if character.species != None {
            // character.stats += species.stats_increase;
        }

        return character;
    }
    pub fn create_goblin() -> Unit {
        let file = File::open("src/EnemyUnits/goblin.json").expect("Unable to open file");
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
            equipment: Some(goblin_gear),
            species: None,
            spell_list: None,
            spell_slots: None,
        }
    }
    /// level up a class for the character
    pub fn level_up(&mut self, class: Class) {
        // check if the class is the starting class
        let starting_class = self.starting_class.clone();
        if starting_class.unwrap().class_name == class.class_name {
            self.starting_class.as_mut().unwrap().level_up();
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
            self.add_additional_class(class.clone());
        }

        // also update character level and hit points
        self.character_level += 1;
        if self.hitpoints_type == HitpointsType::Random {
            self.hitpoints += get_random_dice_value(class.hit_die);
        } else {
            self.hitpoints += get_average_dice_value(class.hit_die);
        }
    }
    pub fn melee_attack(&mut self, target: &mut Unit) {
        if self.action_count > 0 {
            self.action_count -= 1;
            let hit_roll_value = get_random_dice_value(DieType::D20);
            if hit_roll_value + self.stats.get_dexterity_modifier()
                >= target.calculate_armor_class()
            {
                let damage_value = get_random_dice_value(DieType::D6);
                println!("Attack hit!");
                println!("Damage: {}", damage_value);
                target.hitpoints -= damage_value;
            } else {
                println!("Attack missed!");
            }
        } else {
            print!("No action left!");
        }
    }
    pub fn calculate_initiative(&self) -> i32 {
        get_random_dice_value(DieType::D20)
    }
    pub fn calculate_armor_class(&self) -> i32 {
        let mut armor_class = 0;
        let dex_modifier = self.stats.get_dexterity_modifier();
        if self.equipment.as_ref().unwrap().armor.chest != None {
            let armor_type = self
                .equipment
                .as_ref()
                .unwrap()
                .armor
                .chest
                .as_ref()
                .unwrap()
                .armor_type
                .as_ref()
                .unwrap();
            if armor_type == &ArmorType::Light {
                armor_class = dex_modifier;
            } else if armor_type == &ArmorType::Medium {
                if dex_modifier > 2 {
                    armor_class = 2;
                } else {
                    armor_class = dex_modifier;
                }
            }
            armor_class += self
                .equipment
                .as_ref()
                .unwrap()
                .armor
                .chest
                .as_ref()
                .unwrap()
                .armor_class;
        }
        if self.equipment.as_ref().unwrap().armor.shield != None {
            armor_class += self
                .equipment
                .as_ref()
                .unwrap()
                .armor
                .shield
                .as_ref()
                .unwrap()
                .armor_class;
        }
        return armor_class;
    }

    /// Writes the unit to a json.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn write(&self, file_path: &str) -> std::io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }

    /// Reads the unit from a json.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn new(file_path: PathBuf) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let unit: Unit = serde_json::from_reader(reader).expect("could not read");
        Self {
            unit_type: unit.unit_type,
            character_name: unit.character_name,
            character_level: unit.character_level,
            hitpoints: unit.hitpoints,
            hitpoints_type: unit.hitpoints_type,
            movement_speed: unit.movement_speed,
            starting_class: unit.starting_class,
            additional_classes: unit.additional_classes,
            action_count: unit.action_count,
            stats: unit.stats,
            equipment: unit.equipment,
            species: unit.species,
            spell_list: unit.spell_list,
            spell_slots: unit.spell_slots,
        }
    }

    fn add_additional_class(&mut self, class: Class) {
        self.additional_classes.push(class);
    }
}

#[derive(Data, Clone, Serialize, Deserialize, Lens, Copy, Debug, PartialEq, Eq)]
pub struct StatBlock {
    pub intelligence: i32,
    pub constitution: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub wisdom: i32,
    pub charisma: i32,
}
impl StatBlock {
    pub fn new(
        strength: i32,
        dexterity: i32,
        constitution: i32,
        intelligence: i32,
        wisdom: i32,
        charisma: i32,
    ) -> Self {
        StatBlock {
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

impl Add for StatBlock {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            intelligence: self.intelligence + rhs.intelligence,
            constitution: self.constitution + rhs.constitution,
            strength: self.strength + rhs.strength,
            dexterity: self.dexterity + rhs.dexterity,
            wisdom: self.wisdom + rhs.wisdom,
            charisma: self.charisma + rhs.charisma,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitpointsType {
    Random,
    Average,
}
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
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
pub enum HumanoidType {
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

#[derive(Lens, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SpellSlots {
    spell_level_1: i32,
    spell_level_2: i32,
    spell_level_3: i32,
    spell_level_4: i32,
    spell_level_5: i32,
    spell_level_6: i32,
    spell_level_7: i32,
    spell_level_8: i32,
    spell_level_9: i32,
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
