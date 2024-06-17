use rpg_game::entity::{CppEntity, EntityBuilder, PythonEntity, RustEntity};
use rpg_game::{check_switch_character, Team};

fn main() {
    // create player and enemy for prototype.
    let mut player_team = Team::new();
    player_team.push(RustEntity::build(0));
    player_team.push(PythonEntity::build(0));

    let mut enemy_team = Team::new();
    let enemy_cpp = CppEntity::build(0);
    enemy_team.push(enemy_cpp);

    // game loop
    loop {
        check_switch_character(&mut player_team);
        // get active player and enemy.
        let player = player_team
            .get_active()
            .expect("Active player has somehow been destroyed");

        let enemy = enemy_team
            .get_active()
            .expect("Active enemy has somehow been destroyed");

        // display player and enemy health stats
        println!("Player Health: {}/{}", player.health, player.max_health);
        println!("Enemy Health: {}/{}", enemy.health, enemy.max_health);

        if rpg_game::take_moves(player, enemy) {
            break;
        }
    }
}
