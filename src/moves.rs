use crate::entity::{Entity, EntityName};
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Move {
    IntParse,
}

impl fmt::Display for Move {
    /// Implement display to be able to display the names of the moves.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::IntParse => write!(f, "Parse an integer"),
        }
    }
}

impl Move {
    /// Execute itself.
    /// Each move has unique functionality.
    pub fn execute(&self, caller: &mut Entity, enemy: &mut Entity, attack_multiplier: f64) {
        match self {
            Move::IntParse => {
                println!(
                    "A integer needs to be parsed into a string. This may cause an error! {} and {} attempt to handle it",
                    caller.name, enemy.name
                );

                let get_specific_dialogue = |name: &EntityName| {
                    return match name {
                        EntityName::Rust => "Rust returned a result type that can be matched on.",
                        EntityName::Cpp => "C++ needed a catch block to avoid a runtime error.",
                    };
                };
                println!("{}", get_specific_dialogue(&caller.name));
                println!("{}", get_specific_dialogue(&enemy.name));

                if caller.error_handling > enemy.error_handling {
                    println!(
                        "{} handled the error better than {}",
                        caller.name, enemy.name
                    );
                    attack(caller, 20);
                    attack(enemy, (80 as f64 * attack_multiplier) as u32);
                    println!("{} took more damage", enemy.name);
                } else if enemy.error_handling > caller.error_handling {
                    println!(
                        "{} handled the error better than {}",
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
