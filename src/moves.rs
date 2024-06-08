use crate::entity::Entity;

#[derive(PartialEq)]
pub enum Move {
    Attack,
    Heal,
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
