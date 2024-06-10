use rpg_game::entity::Entity;
use rpg_game::moves::Move;

fn main() {
    let mut player = Entity::new("Player1", 200, 20, 20, 90);
    player.set_moves(vec![
        Move::Attack,
        Move::Heal,
        Move::BuffDef,
        Move::DebuffAcc,
    ]);

    let mut enemy = Entity::new("Enemy1", 200, 10, 10, 90);
    enemy.set_moves(vec![Move::Attack, Move::Heal]);

    loop {
        println!("Player Health: {}/{}", player.health, player.max_health);
        println!("Enemy Health: {}/{}", enemy.health, enemy.max_health);

        rpg_game::player_move(&mut player, &mut enemy);
        if enemy.health == 0 {
            println!("The enemy has died, you win");
            break;
        }
        rpg_game::enemy_move(&mut enemy, &mut player);
        if player.health == 0 {
            println!("The player has died, you lose");
            break;
        }
    }
}
