mod cpp_entity;
pub use cpp_entity::CppEntity;

mod rust_entity;
pub use rust_entity::RustEntity;

use crate::moves::Move;
use crate::moves::MoveNotFoundError;
use rand::prelude::*;

pub trait EntityType {
    fn new(level: u32) -> Entity;
}

pub struct Entity {
    pub health: u32,
    pub max_health: u32,
    pub name: String,
    pub level: u32,
    attack: u32,
    defense: u32,
    accuracy: u32,
    error_handling: u32,
    moves: Vec<Move>,
    weaknesses: Vec<Move>,
}

impl Entity {
    fn new(
        name: &str,
        max_health: u32,
        level: u32,
        attack: u32,
        defense: u32,
        accuracy: u32,
        error_handling: u32,
        moves: Vec<Move>,
        weaknesses: Vec<Move>,
    ) -> Self {
        Entity {
            name: String::from(name),
            health: max_health,
            max_health,
            level,
            attack,
            defense,
            accuracy,
            error_handling,
            moves,
            weaknesses,
        }
    }

    fn defend_damage(&self, damage: u32) -> u32 {
        let defended_damage = damage as f64 - (damage as f64 * (self.defense as f64 / 100.0));
        defended_damage as u32
    }

    fn accuracy_roll(&self) -> bool {
        rand::thread_rng().gen_ratio(self.accuracy, 100)
    }

    pub fn damage(&mut self, damage: u32) {
        let damage = self.defend_damage(damage);

        if damage >= self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    pub fn heal(&mut self, health: u32) {
        if health >= self.max_health - self.health {
            self.health = self.max_health;
        } else {
            self.health += health;
        }
    }

    pub fn execute_move(
        &mut self,
        target: &mut Entity,
        move_index: usize,
    ) -> Result<(), MoveNotFoundError> {
        let mv: Move = match self.moves.get(move_index) {
            Some(mv) => mv.clone(),
            None => return Err(MoveNotFoundError),
        };
        println!("{} used {}...", self.name, mv);
        if !self.accuracy_roll() {
            println!("{} missed", self.name);
            return Ok(());
        }
        let attack_multiplier: f64 = self.attack as f64 / 100.0 + 1.0;
        mv.execute(self, target, attack_multiplier);
        Ok(())
    }

    pub fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }

    pub fn change_stat(&mut self, stat: Stat, amount: i32) {
        let mut final_amount = amount;
        let stat_to_change = match stat {
            Stat::Attack => &mut self.attack,
            Stat::Defense => &mut self.defense,
            Stat::Accuracy => {
                if amount >= (100 - self.accuracy) as i32 {
                    final_amount = (100 - self.accuracy) as i32;
                }
                &mut self.accuracy
            }
            Stat::ErrorHandling => &mut self.error_handling,
        };

        if amount < 0 && amount.abs() > *stat_to_change as i32 {
            final_amount = -(*stat_to_change as i32);
        }

        *stat_to_change = (*stat_to_change as i32 + final_amount) as u32;
    }
    pub fn get_stat(&self, stat: Stat) -> u32 {
        match stat {
            Stat::Attack => self.attack,
            Stat::Defense => self.defense,
            Stat::Accuracy => self.accuracy,
            Stat::ErrorHandling => self.error_handling,
        }
    }
}

pub enum Stat {
    Attack,
    Defense,
    Accuracy,
    ErrorHandling,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_damage() {
        let mut player = Entity::new("RandomPlayer", 200, 0, 5, 0);
        assert_eq!(player.health, 200);
        player.damage(198);
        assert_eq!(player.health, 12);
    }

    #[test]
    fn change_stat() {
        let mut player = Entity::new("RandomPlayer", 200, 0, 5, 90);
        assert_eq!(player.defense, 5);
        player.change_stat(Stat::Defense, -10);
        assert_eq!(player.defense, 0);
        player.change_stat(Stat::Defense, 5);
        assert_eq!(player.defense, 5);
        player.change_stat(Stat::Accuracy, 20);
        assert_eq!(player.accuracy, 100);
    }
}
