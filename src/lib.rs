pub mod entity;
pub mod moves;
mod team;

pub use team::Team;

use entity::Entity;
use std::io;
use std::num::ParseIntError;

use rand::prelude::*;

use moves::Move;

// Switch the player's character.
pub fn switch_player(team: &mut Team) {
    for (i, entity) in team.entities.iter().enumerate() {
        println!("{}, {}", i, entity);
    }

    let input: usize = match get_int_input() {
        Ok(i) => i,
        Err(_) => return,
    };

    team.set_active(input);
}

pub enum ActionType {
    Attack,
    Switch,
    Forfeit,
}

pub fn get_battle_action() -> ActionType {
    println!("[0]: Attack");
    println!("[1]: Switch characters");
    println!("[2]: Forfeit");

    let input: usize = loop {
        match get_int_input() {
            Ok(i) => break i,
            Err(_) => continue,
        }
    };

    if input == 1 {
        return ActionType::Switch;
    }
    if input == 2 {
        return ActionType::Forfeit;
    } else {
        return ActionType::Attack;
    }
}

fn get_int_input() -> Result<usize, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let input: usize = input.trim().parse()?;
    Ok(input)
}

/// Execute moves of the player and enemy.
/// Returns True if either the player or enemy died.
pub fn take_moves(
    player: &mut Entity,
    player_priority: MovePriority,
    enemy: &mut Entity,
    enemy_priority: MovePriority,
) -> bool {
    let check = |player: &Entity, enemy: &Entity| -> bool {
        if enemy.health == 0 {
            println!("The enemy has died, you win");
        } else if player.health == 0 {
            println!("The player has died, you lose");
        } else {
            return false;
        }
        true
    };

    match player_move_priority >= enemy_move_priority {
        true => {
            player.execute_move(enemy);
            if check(player, enemy) {
                return true;
            };
            enemy.execute_move(player);
            if check(player, enemy) {
                return true;
            };
        }
        false => {
            enemy.execute_move(player);
            if check(player, enemy) {
                return true;
            };
            player.execute_move(enemy);
            if check(player, enemy) {
                return true;
            };
        }
    };
    false
}

#[derive(PartialEq, PartialOrd)]
struct MovePriority(u8);

/// Get user input for a move, then execute it against the enemy.
fn player_move(player: &mut Entity) -> MovePriority {
    loop {
        // print out the options
        println!("Choose your move: ");
        for (i, mv) in player.get_moves().iter().enumerate() {
            println!("{}: {}", i, mv);
        }

        // parse the input to an integer
        let i: usize = match get_int_input() {
            Ok(i) => i,
            Err(_) => continue,
        };

        // get the move associated with the index.
        let mv = match player.get_moves().get(i) {
            Some(mv) => mv.clone(),
            None => {
                println!("Move was not found in the player's list of moves");
                continue;
            }
        };

        player.queue_move(mv);
        return MovePriority(mv.get_priority());
    }
}

fn enemy_move(enemy: &mut Entity) -> MovePriority {
    let i = rand::thread_rng().gen_range(0..=enemy.get_moves().len() - 1);
    let mv = enemy.get_moves().get(i).unwrap().clone();

    enemy.queue_move(mv);
    MovePriority(mv.get_priority())
}
