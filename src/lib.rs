pub mod entity;
pub mod moves;
pub mod ui;

mod team;

pub use team::Team;

use entity::Entity;
use std::collections::VecDeque;
use std::io;
use std::num::ParseIntError;

use rand::prelude::*;

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
