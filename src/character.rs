use character_derived_lenses::hitpoints_type;
use num::Integer;
use rand::Rng;
use vizia::prelude::*;

#[derive(Lens, Debug)]
pub struct Character {
    pub character_name: String,
    pub character_level: i32,
    pub hitpoints: i32,
    pub hitpoints_type: HitpointsType,
    pub movement_speed: i32,
    pub starting_class: Class,
    pub additional_classes: Vec<Class>,
}

impl Character {
    pub fn create_character(name: String, class: Class, hitpointstype: HitpointsType) -> Self {
        
        let mut hp = 0;
        if hitpointstype == HitpointsType::Random {
            hp = get_random_dice_value(class.hit_die);
        }
        else {
            hp = get_average_dice_value(class.hit_die);
        }

        let character = Character {
            character_name: name,
            character_level: class.class_level,
            hitpoints: hp,
            hitpoints_type: hitpointstype,
            movement_speed: 30,
            starting_class: class,
            additional_classes: Vec::new(),
        };
        return character;
    }
    /// level up a class for the character
    pub fn level_up(&mut self, class: Class) {
        // check if the class is the starting class
        if self.starting_class.class_name == class.class_name {
            self.starting_class.level_up();
        }
        // check if the class is a multi class and get the index if it is 
        else if let Some(index) = self.additional_classes.iter().position(|i| i.class_name==class.class_name) {
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
        }
        else {
            self.hitpoints += get_average_dice_value(class.hit_die);
        }
    }
    fn add_additional_class(&mut self, class: Class){
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
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_barbarian() -> Self {
        let class = Class {
            class_name: ClassName::Barbarian,
            hit_die: DieType::d12,
            class_level: 1,
        };
        return class;
    }
    pub fn create_bard() -> Self {
        let class = Class {
            class_name: ClassName::Bard,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_cleric() -> Self {
        let class = Class {
            class_name: ClassName::Cleric,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_druid() -> Self {
        let class = Class {
            class_name: ClassName::Druid,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_fighter() -> Self {
        let class = Class {
            class_name: ClassName::Fighter,
            hit_die: DieType::d10,
            class_level: 1,
        };
        return class;
    }
    pub fn create_monk() -> Self {
        let class = Class {
            class_name: ClassName::Monk,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_paladin() -> Self {
        let class = Class {
            class_name: ClassName::Paladin,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_ranger() -> Self {
        let class = Class {
            class_name: ClassName::Ranger,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_rogue() -> Self {
        let class = Class {
            class_name: ClassName::Rogue,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_sorcerer() -> Self {
        let class = Class {
            class_name: ClassName::Sorcerer,
            hit_die: DieType::d6,
            class_level: 1,
        };
        return class;
    }
    pub fn create_warlock() -> Self {
        let class = Class {
            class_name: ClassName::Warlock,
            hit_die: DieType::d8,
            class_level: 1,
        };
        return class;
    }
    pub fn create_wizard() -> Self {
        let class = Class {
            class_name: ClassName::Wizard,
            hit_die: DieType::d6,
            class_level: 1,
        };
        return class;
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
pub enum DieType{
    d4,
    d6,
    d8,
    d10,
    d12,
    d20,
}

fn get_random_dice_value(die: DieType) -> i32 {
    match die {
        DieType::d4 => rand::thread_rng().gen_range(1..=4),
        DieType::d6 => rand::thread_rng().gen_range(1..=6),
        DieType::d8 => rand::thread_rng().gen_range(1..=8),
        DieType::d10 => rand::thread_rng().gen_range(1..=10),
        DieType::d12 => rand::thread_rng().gen_range(1..=12),
        DieType::d20 => rand::thread_rng().gen_range(1..=20),
    }
}

fn get_average_dice_value(die: DieType) -> i32 {
    match die {
        DieType::d4 => average(1..=4, 4),
        DieType::d6 => average(1..=6,6),
        DieType::d8 => average(1..=8,8),
        DieType::d10 => average(1..=10,10),
        DieType::d12 => average(1..=12,12),
        DieType::d20 => average(1..=20,20),
    }
}

fn average(numbers: std::ops::RangeInclusive<i32>, count: i32) -> i32 {
    let sum: i32 = numbers.into_iter().sum();
    sum.div_ceil(&count) 
}