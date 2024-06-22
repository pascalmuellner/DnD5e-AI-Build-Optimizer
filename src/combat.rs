use crate::unit::Unit;
#[derive(Debug, Clone)]
pub struct Combat {
    player_units: Vec<Unit>,
    enemy_units: Vec<Unit>,
    pub turn_order: Vec<CombatUnit>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CombatUnit {
    unit: Unit,
    initiative: i32,
    alive: bool,
}

impl Combat {
    pub fn new(player_units: &Vec<Unit>, enemy_units: &Vec<Unit>) -> Self {
        let mut turn_order = Vec::<CombatUnit>::new();
        for unit in player_units {
            let init = unit.calculate_initiative();
            let combat_unit = CombatUnit {
                unit: unit.clone(),
                initiative: init,
                alive: true,
            };
            turn_order.push(combat_unit);
        }
        for unit in enemy_units.iter() {
            let init = unit.calculate_initiative();
            let combat_unit = CombatUnit {
                unit: unit.clone(),
                initiative: init,
                alive: true,
            };
            turn_order.push(combat_unit);
        }
        turn_order.sort_by_key(|unit| (unit.initiative * -1));
        Self {
            player_units: player_units.to_vec(),
            enemy_units: enemy_units.to_vec(),
            turn_order,
        }
    }

    /// Returns the start of this [`Combat`].
    pub fn start(&mut self) {
        println!("Turn Order: {:#?}", self.turn_order);
    }

    pub fn fight(&mut self, mut combat_unit: CombatUnit) {
        if combat_unit.unit.action_count > 0 {
            if self.player_units.contains(&&combat_unit.unit) {
                combat_unit.unit.melee_attack(&mut self.enemy_units[0]);
                println!("enemy hp: {:#?}", self.enemy_units[0].hitpoints);
            } else {
                combat_unit.unit.melee_attack(&mut self.player_units[0]);
                println!("player hp: {:#?}", self.player_units[0].hitpoints);
            }
        }
    }
}
