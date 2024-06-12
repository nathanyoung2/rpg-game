use rpg_game::entity::{CppEntity, EntityType, RustEntity};

fn main() {
    let mut player = RustEntity::new(0);
    let mut enemy = CppEntity::new(0);

    loop {
        println!("Player Health: {}/{}", player.health, player.max_health);
        println!("Enemy Health: {}/{}", enemy.health, enemy.max_health);

        rpg_game::player_move(&mut player, &mut enemy);
        if enemy.health == 0 {
            println!("The enemy has died, you win");
            break;
        }
        if player.health == 0 {
            println!("The enemy has died, you win");
            break;
        }
        rpg_game::enemy_move(&mut enemy, &mut player);
        if player.health == 0 {
            println!("The player has died, you lose");
            break;
        }
        if enemy.health == 0 {
            println!("The enemy has died, you win");
            break;
        }
    }
}
