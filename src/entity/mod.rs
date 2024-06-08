use crate::moves;
use crate::moves::Move;

pub struct Entity {
    pub health: u32,
    pub max_health: u32,
    attack: u32,
    defense: u32,
    accuracy: u32,
    moves: Vec<Move>,
}

impl Entity {
    pub fn new(max_health: u32, attack: u32, defense: u32, accuracy: u32) -> Self {
        Entity {
            health: max_health,
            max_health,
            attack,
            defense,
            accuracy,
            moves: Vec::new(),
        }
    }

    pub fn damage(&mut self, damage: u32) -> u32 {
        let defended_damage = damage as f64 - (damage as f64 * (self.defense as f64 / 100.0));
        let defended_damage = defended_damage as u32;

        if defended_damage >= self.health {
            self.health = 0;
        } else {
            self.health -= defended_damage;
        }
        self.health
    }

    pub fn heal(&mut self, health: u32) -> u32 {
        self.health += health;
        self.health
    }

    pub fn execute_move(&mut self, target: &mut Entity, mv: Move) {
        if self.moves.contains(&mv) {
            moves::execute(mv, self, target);
        }
    }

    pub fn set_moves(&mut self, mut moves: Vec<Move>) {
        moves.truncate(4);
        self.moves = moves;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_damage() {
        let mut player = Entity::new(200, 0, 5, 0);
        assert_eq!(player.health, 200);
        player.damage(198);
        assert_eq!(player.health, 12);
    }
}
