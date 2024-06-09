use crate::moves;
use crate::moves::Move;
use crate::moves::MoveNotFoundError;

pub struct Entity {
    pub health: u32,
    pub max_health: u32,
    pub name: String,
    attack: u32,
    defense: u32,
    accuracy: u32,
    moves: Vec<Move>,
}

impl Entity {
    pub fn new(name: &str, max_health: u32, attack: u32, defense: u32, accuracy: u32) -> Self {
        Entity {
            name: String::from(name),
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
        if self.health > self.max_health {
            self.health = self.max_health;
        }
        self.health
    }

    pub fn execute_move(
        &mut self,
        target: &mut Entity,
        move_index: usize,
    ) -> Result<String, MoveNotFoundError> {
        let mv: Move = match self.moves.get(move_index) {
            Some(mv) => mv.clone(),
            None => return Err(MoveNotFoundError),
        };

        let output = format!("{} used {} on {}", self.name, mv, target.name);
        moves::execute(mv, self, target);

        Ok(output)
    }

    pub fn set_moves(&mut self, mut moves: Vec<Move>) {
        moves.truncate(4);
        self.moves = moves;
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
        let mut player = Entity::new("RandomPlayer", 200, 0, 5, 0);
        assert_eq!(player.health, 200);
        player.damage(198);
        assert_eq!(player.health, 12);
    }
}
