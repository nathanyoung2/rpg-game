pub mod entity;
pub mod moves;

use entity::Entity;
use std::io;

use rand::prelude::*;

pub fn player_move(player: &mut Entity, enemy: &mut Entity) {
    loop {
        println!("Choose your move: ");
        for (i, mv) in player.get_moves().iter().enumerate() {
            println!("{}: {}", i, mv);
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the line");

        let i: usize = match input.trim_end().parse() {
            Ok(i) => i,
            Err(_) => continue,
        };

        match player.execute_move(enemy, i) {
            Ok(output) => println!("{output}"),
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        break;
    }
}

pub fn enemy_move(enemy: &mut Entity, player: &mut Entity) {
    let chosen_move = rand::thread_rng().gen_range(0..=enemy.get_moves().len() - 1);

    // There is no way that this will return a MoveNotFound Error so we can safely unwrap.
    let output = enemy.execute_move(player, chosen_move).unwrap();
    println!("{output}");
}
