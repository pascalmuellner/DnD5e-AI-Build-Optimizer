use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use vizia::prelude::*;

use crate::equipment::*;
use crate::subclass::*;
use crate::unit::DieType;
use crate::weapon;
use crate::ArmorList;
use crate::ArmorSlots;
use crate::WeaponList;

#[derive(Lens, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub class_name: ClassName,
    pub sub_class: Option<SubClass>,
    pub sub_class_unlock_level: i32,
    pub hit_die: DieType,
    pub class_level: i32,
    pub starting_gear: Equipment,
}

impl Class {
    pub fn level_up(&mut self) {
        self.class_level += 1;
        if self.class_level == self.sub_class_unlock_level {
            // TODO: add subclass
        }
    }
    pub fn create_artificer() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Artificer,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_barbarian() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Barbarian,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D12,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_bard() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Bard,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_cleric() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Cleric,
            sub_class: None,
            sub_class_unlock_level: 1,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_druid() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Druid,
            sub_class: None,
            sub_class_unlock_level: 2,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_fighter() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Fighter,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D10,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_monk() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Monk,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_paladin() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Paladin,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_ranger() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Ranger,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_rogue() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Rogue,
            sub_class: None,
            sub_class_unlock_level: 3,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_sorcerer() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Sorcerer,
            sub_class: None,
            sub_class_unlock_level: 1,
            hit_die: DieType::D6,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_warlock() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Warlock,
            sub_class: None,
            sub_class_unlock_level: 1,
            hit_die: DieType::D8,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
    pub fn create_wizard() -> Self {
        let weapon_list = WeaponList::new("src/Data/weapons.json");
        let armor_list = ArmorList::new("src/Data/armors.json");
        let dagger = weapon_list.get_weapon(1);
        let chest = armor_list.get_armor(1);
        let class = Class {
            class_name: ClassName::Wizard,
            sub_class: None,
            sub_class_unlock_level: 2,
            hit_die: DieType::D6,
            class_level: 1,
            starting_gear: Equipment {
                armor: ArmorSlots {
                    chest,
                    boots: None,
                    gloves: None,
                    helmet: None,
                    shield: None,
                },
                melee_weapon_mainhand: dagger.clone(),
                melee_weapon_offhand: dagger.clone(),
                ranged_weapon: None,
            },
        };
        return class;
    }
}

#[derive(Data, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
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
        vec![
            ClassName::Artificer,
            ClassName::Barbarian,
            ClassName::Bard,
            ClassName::Cleric,
            ClassName::Druid,
            ClassName::Fighter,
            ClassName::Monk,
            ClassName::Paladin,
            ClassName::Ranger,
            ClassName::Rogue,
            ClassName::Sorcerer,
            ClassName::Warlock,
            ClassName::Wizard,
        ]
    }
}
impl Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
