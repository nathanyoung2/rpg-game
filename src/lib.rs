pub mod entity;
use crate::entity::player;
use std::io;

fn start_battle(player: player::Player, enemy: enemy::Enemy) {
    let input = loop {
        print!("Enter a move: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read a correct string");
        let input = match input.parse() {
            Ok(i) => i,
            Err(_) => continue,
        };
        break input;
    };

    if let Some(mv) = player.moves[input] {
        mv.execute(&enemy);
    }
}
