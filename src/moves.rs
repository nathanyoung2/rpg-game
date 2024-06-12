use crate::entity::{Entity, Stat};
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Move {
    Error,
}

impl fmt::Display for Move {
    /// Implement display to be able to display the names of the moves.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Error => write!(f, "Error"),
        }
    }
}

impl Move {
    /// Execute itself.
    /// Each move has unique functionality.
    pub fn execute(&self, caller: &mut Entity, enemy: &mut Entity, attack_multiplier: f64) {
        match self {
            Move::Error => {
                println!("An error has occured!");
                if caller.get_stat(Stat::ErrorHandling) > enemy.get_stat(Stat::ErrorHandling) {
                    println!(
                        "{}'s error handling is better than the enemy {}'s error handling...",
                        caller.name, enemy.name
                    );
                    attack(caller, 20);
                    attack(enemy, (80 as f64 * attack_multiplier) as u32);
                    println!("enemy {} took more damage", enemy.name)
                } else if enemy.get_stat(Stat::ErrorHandling) > caller.get_stat(Stat::ErrorHandling)
                {
                    println!(
                        "enemy {}'s error handling is better than {}'s error handing...",
                        enemy.name, caller.name
                    );
                    attack(caller, 80);
                    attack(enemy, (20 as f64 * attack_multiplier) as u32);
                    println!("{} took more damage", caller.name);
                } else {
                    println!("Both languages have equal error handling abilities");
                    attack(caller, 20);
                    attack(enemy, (20 as f64 * attack_multiplier) as u32);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveNotFoundError;

impl fmt::Display for MoveNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The index specified is outside of the range of the entity's moves",
        )
    }
}

fn attack(target: &mut Entity, damage: u32) {
    target.damage(damage);
}

fn heal(target: &mut Entity, health: u32) {
    target.heal(health);
}
