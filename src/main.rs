use rpg_game::entity::{CppEntity, EntityBuilder, RustEntity};

fn main() {
    // create player and enemy for prototype.
    let mut player = RustEntity::build(0);
    let mut enemy = CppEntity::build(0);

    // game loop
    loop {
        // display player and enemy health stats
        println!("Player Health: {}/{}", player.health, player.max_health);
        println!("Enemy Health: {}/{}", enemy.health, enemy.max_health);

        if rpg_game::take_moves(&mut player, &mut enemy) {
            break;
        }
    }
}
