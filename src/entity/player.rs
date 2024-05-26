use crate::entity::*;

pub struct Player {
    max_health: u32,
    attack: u32,
    defense: u32,
    accuracy: u32,
    current_health: u32,
    moves: Vec<Box<dyn Move>>,
    status: Option<Box<dyn Status>>,
}

impl Player {
    pub fn new(max_health: u32, attack: u32, defense: u32, accuracy: u32) -> Self {
        Player {
            max_health,
            attack,
            defense,
            accuracy,
            current_health: max_health,
            moves: Vec::new(),
            status: None,
        }
    }
}

impl Character for Player {
    fn take_damage(&mut self, damage: u32) {
        let defended_damage = damage as f64 - (damage as f64 * (self.defense as f64 / 100.0));
        let defended_damage = defended_damage as u32;

        if defended_damage >= self.current_health {
            self.current_health = 0
        } else {
            self.current_health -= defended_damage;
        }
    }

    fn change_stats(&mut self, stat: Stat, amount: u32, subtract: bool) {
        let stat_to_change = match stat {
            Stat::Attack => &mut self.attack,
            Stat::Defense => &mut self.defense,
            Stat::Accuracy => &mut self.accuracy,
            Stat::MaxHealth => &mut self.max_health,
        };

        if subtract {
            if amount > *stat_to_change {
                *stat_to_change = 0;
            } else {
                *stat_to_change -= amount;
            }
        } else {
            *stat_to_change += amount;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_stat_change() {
        let mut player = Player::new(100, 0, 0, 0);
        player.change_stats(Stat::Attack, 10, false);
        assert_eq!(player.attack, 10);
        player.change_stats(Stat::Attack, 10, false);
        assert_eq!(player.attack, 20);
    }

    #[test]
    fn check_stat_change_subtract() {
        let mut player = Player::new(100, 0, 10, 0);
        player.change_stats(Stat::Defense, 10, true);
        assert_eq!(player.defense, 0);
    }

    #[test]
    fn check_stat_change_lower_invalid() {
        let mut player = Player::new(100, 0, 0, 0);
        player.change_stats(Stat::Defense, 1, true);
        assert_eq!(player.defense, 0);
    }

    #[test]
    fn take_damage() {
        let mut player = Player::new(200, 0, 5, 0);
        assert_eq!(player.current_health, 200);
        player.take_damage(198);
        assert_eq!(player.current_health, 12);
    }
}
