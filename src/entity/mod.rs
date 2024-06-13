use std::fmt;

mod cpp_entity;
pub use cpp_entity::CppEntity;

mod rust_entity;
pub use rust_entity::RustEntity;

use crate::moves::Move;
use crate::moves::MoveNotFoundError;
use rand::prelude::*;

/// trait for any
pub trait EntityBuilder {
    fn build(level: u32) -> Entity;
}

pub enum EntityType {
    Rust,
    Cpp,
}

/// holds stats for entities for battles.
pub struct Entity {
    pub health: u32,
    pub max_health: u32,
    pub entity_type: EntityType,
    pub level: u32,
    pub attack: u32,
    pub defense: u32,
    pub accuracy: u32,
    pub error_handling: u32,
    moves: Vec<Move>,
    weaknesses: Vec<Move>,
    strengths: Vec<Move>,
    queued_move: Option<Move>,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.entity_type {
            EntityType::Rust => write!(f, "Rust"),
            EntityType::Cpp => write!(f, "C++"),
        }
    }
}

impl Entity {
    fn new(
        entity_type: EntityType,
        max_health: u32,
        level: u32,
        attack: u32,
        defense: u32,
        accuracy: u32,
        error_handling: u32,
        moves: Vec<Move>,
        weaknesses: Vec<Move>,
        strengths: Vec<Move>,
    ) -> Self {
        Entity {
            entity_type,
            health: max_health,
            max_health,
            level,
            attack,
            defense,
            accuracy,
            error_handling,
            moves,
            weaknesses,
            strengths,
            queued_move: None,
        }
    }

    fn defend_damage(&self, damage: u32) -> u32 {
        let defended_damage = damage as f64 - (damage as f64 * (self.defense as f64 / 100.0));
        defended_damage as u32
    }

    fn accuracy_roll(&self) -> bool {
        rand::thread_rng().gen_ratio(self.accuracy, 100)
    }

    pub fn queue_move(&mut self, mv: Move) {
        self.queued_move = Some(mv);
    }

    pub fn damage(&mut self, damage: u32, applied_move: Option<Move>) {
        let mut damage = self.defend_damage(damage);

        if let Some(mv) = applied_move {
            if self.weaknesses.contains(&mv) {
                damage = (damage as f64 * 1.5) as u32;
            }
        }

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

    pub fn execute_move(&mut self, target: &mut Entity) {
        // get the move from the queue
        let mv = match self.queued_move {
            Some(mv) => mv,
            None => return,
        };

        println!("{} used {}...", self, mv);

        // roll to check if a miss occured,
        if !self.accuracy_roll() {
            println!("{} missed", self);
            return;
        }

        // calculate damage multipliers
        let attack_multiplier: f64 = self.attack as f64 / 100.0 + 1.0;
        let is_super_effective = self.weaknesses.contains(&mv);
        let is_not_effective = self.strengths.contains(&mv);

        // execute the move
        mv.execute(
            self,
            target,
            attack_multiplier,
            is_super_effective,
            is_not_effective,
        );
    }

    pub fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_damage() {
        let mut player = RustEntity::build(0);
        assert_eq!(player.health, 200);
        player.damage(198, None);
        assert_eq!(player.health, 62);
    }
    #[test]
    fn test_entity_display() {
        assert_eq!(format!("{}", RustEntity::build(0)), "Rust");
        assert_eq!(format!("{}", CppEntity::build(0)), "C++");
    }
}
