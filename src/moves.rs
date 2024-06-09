use crate::entity::Entity;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Move {
    Attack,
    Heal,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Attack => write!(f, "Attack"),
            Move::Heal => write!(f, "Heal"),
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

pub fn execute(mv: Move, caller: &mut Entity, enemy: &mut Entity) {
    match mv {
        Move::Attack => attack(enemy),
        Move::Heal => heal(caller),
    }
}

fn attack(target: &mut Entity) {
    target.damage(50);
}

fn heal(target: &mut Entity) {
    target.heal(50);
}
