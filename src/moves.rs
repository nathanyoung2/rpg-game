use crate::entity::{Entity, Stat};
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Move {
    Attack,
    Heal,
    DebuffAcc,
    BuffDef,
}

impl fmt::Display for Move {
    /// Implement display to be able to display the names of the moves.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Attack => write!(f, "Attack"),
            Move::Heal => write!(f, "Heal"),
            Move::DebuffAcc => write!(f, "Debuff Accuracy"),
            Move::BuffDef => write!(f, "Buff Defense"),
        }
    }
}

impl Move {
    /// Execute itself.
    /// Each move has unique functionality.
    pub fn execute(&self, caller: &mut Entity, enemy: &mut Entity, attack_multiplier: f64) {
        match self {
            Move::Attack => {
                // add the multiplied attack onto the original attack.
                attack(enemy, 50 + (50.0 * attack_multiplier) as u32);
                println!("{} attacked {}", caller.name, enemy.name);
            }
            Move::Heal => {
                heal(caller, 35);
                println!("{} healed its HP", caller.name);
            }
            Move::DebuffAcc => {
                enemy.change_stat(Stat::Accuracy, -5);
                println!("{} lowered {}'s accuracy", caller.name, enemy.name);
            }
            Move::BuffDef => {
                caller.change_stat(Stat::Defense, 5);
                println!("{} raised its defense", caller.name);
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
