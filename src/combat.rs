use crate::unit::Unit;
use nalgebra::Point3;
use rand::Rng;

/// Combat is the structure that holds the combat relevant functionality.
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
    position: Point3<i32>,
}

impl Combat {
    /// Creates a new [`Combat`].
    /// Initializes the turn order for all units.
    /// Takes ownership over the units.
    pub fn new(player_units: &Vec<Unit>, enemy_units: &Vec<Unit>, close_combat: bool) -> Self {
        let mut turn_order = Vec::<CombatUnit>::new();
        let mut rng = rand::thread_rng();
        for unit in player_units {
            let init = unit.calculate_initiative();

            let position = if close_combat {
                Point3::new(0, 0, 0)
            } else {
                Point3::new(rng.gen_range(0..50), rng.gen_range(0..50), 0)
            };

            let combat_unit = CombatUnit {
                unit: unit.clone(),
                initiative: init,
                alive: true,
                position,
            };
            turn_order.push(combat_unit);
        }
        for unit in enemy_units.iter() {
            let init = unit.calculate_initiative();
            let position = if close_combat {
                Point3::new(5, 5, 5)
            } else {
                Point3::new(rng.gen_range(0..50), rng.gen_range(0..50), 0)
            };
            let combat_unit = CombatUnit {
                unit: unit.clone(),
                initiative: init,
                alive: true,
                position,
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

    /// Starts the [`Combat`].
    pub fn start(&mut self) {
        println!("Turn Order: {:#?}", self.turn_order);
    }

    /// Creates the fightg for a [`CombatUnit`].
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
    /// Ends the combat and returns the ownership of the units.
    pub fn end(self) -> (Vec<Unit>, Vec<Unit>) {
        return (self.player_units, self.enemy_units);
    }
}
