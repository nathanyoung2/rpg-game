use rpg_game::entity::{CppEntity, EntityBuilder, PythonEntity, RustEntity};
use rpg_game::{ActionType, Team};

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

        let battle_action = rpg_game::get_battle_action();

        if let ActionType::Attack = battle_action {
            rpg_game::queue_player_move(player);
        }

        if let ActionType::Switch = battle_action {
            rpg_game::switch_player(&mut player_team);
        }

        if let ActionType::Forfeit = battle_action {
            println!("You decided that the battle was futile and quit early");
            break;
        }

        // get active player and enemy.
        let player = player_team
            .get_active()
            .expect("Active player has somehow been destroyed");

        let enemy = enemy_team
            .get_active()
            .expect("Active enemy has somehow been destroyed");

        rpg_game::queue_enemy_move(enemy);

        if rpg_game::execute_moves(player, enemy) {
            break;
        }
    }
}
