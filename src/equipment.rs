use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::{armor::*, item::*, weapon::*};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Equipment {
    pub armor: ArmorSlots,
    pub melee_weapon_mainhand: Option<Weapon>,
    pub melee_weapon_offhand: Option<Weapon>,
    pub ranged_weapon: Option<Weapon>,
}