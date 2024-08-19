pub mod entity;
pub mod moves;
pub mod ui;

mod team;

pub use team::Team;

use entity::*;
use std::collections::VecDeque;

use rand::prelude::*;

pub fn active_died(team: &mut Team) -> bool {
    for (i, entity) in team.entities.iter().enumerate() {
        if entity.health > 0 {
            team.set_active(i).expect("Unexpected error");
            return false;
        }
    }
    true
}

/// Execute moves of the player and enemy.
/// Returns True if either the player or enemy died.
pub fn execute_moves(player: &mut Entity, enemy: &mut Entity, text_queue: &mut VecDeque<String>) {
    let player_priority = match player.get_move_priority() {
        Some(priority) => priority,
        None => 0,
    };

    let enemy_priority = match enemy.get_move_priority() {
        Some(priority) => priority,
        None => 0,
    };

    match player_priority >= enemy_priority {
        true => {
            player.execute_move(enemy, text_queue);
            enemy.execute_move(player, text_queue);
        }
        false => {
            enemy.execute_move(player, text_queue);
            player.execute_move(enemy, text_queue);
        }
    };
}

pub fn queue_enemy_move(enemy: &mut Entity) {
    let i = rand::thread_rng().gen_range(0..=enemy.get_moves().len() - 1);
    let mv = enemy.get_moves().get(i).unwrap().clone();

    enemy.queue_move(mv);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_died_test() {
        let mut team = Team::new();
        let rust = RustEntity::build(0, None);
        let rust2 = RustEntity::build(0, None);
        team.push(rust);
        team.push(rust2);

        team.entities[0].health = 0;
        team.entities[1].health = 1;

        assert_eq!(false, active_died(&mut team));

        team.entities[1].health = 0;
        assert_eq!(true, active_died(&mut team));
    }
}
