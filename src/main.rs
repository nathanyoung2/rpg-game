use rpg_game::entity::Entity;
use rpg_game::moves::Move;

fn main() {
    let mut player = Entity::new("Player1", 200, 20, 20, 20);
    player.set_moves(vec![Move::Attack, Move::Heal]);

    let mut enemy = Entity::new("Enemy1", 200, 20, 20, 20);
    enemy.set_moves(vec![Move::Attack, Move::Heal]);

    loop {
        println!("Player Health: {}/{}", player.health, player.max_health);
        println!("Enemy Health: {}/{}", enemy.health, enemy.max_health);
        rpg_game::player_move(&mut player, &mut enemy);
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
