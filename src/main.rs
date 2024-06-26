use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

use rpg_game::entity::{CppEntity, EntityBuilder, PythonEntity, RustEntity};
use rpg_game::{ActionType, Team};

fn main() -> GameResult {
    // create player and enemy for prototype.
    let mut player_team = Team::new();
    player_team.push(RustEntity::build(0));
    player_team.push(PythonEntity::build(0));

    let mut enemy_team = Team::new();
    enemy_team.push(CppEntity::build(0));

    let (ctx, event_loop) = ContextBuilder::new("rpg_game", "Nathan Young").build()?;
    let state = BattleState::new(player_team, enemy_team);

    event::run(ctx, event_loop, state)
}

struct BattleState {
    player_team: Team,
    enemy_team: Team,
}

impl BattleState {
    fn new(player_team: Team, enemy_team: Team) -> Self {
        BattleState {
            player_team,
            enemy_team,
        }
    }
}

impl EventHandler<ggez::GameError> for BattleState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // get active player and enemy.
        let player = self
            .player_team
            .get_active()
            .expect("Active player has somehow been destroyed");

        let enemy = self
            .enemy_team
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
            rpg_game::switch_player(&mut self.player_team);
        }

        if let ActionType::Forfeit = battle_action {
            println!("You decided that the battle was futile and quit early");
            return Ok(());
        }

        // get active player and enemy.
        let player = self
            .player_team
            .get_active()
            .expect("Active player has somehow been destroyed");

        let enemy = self
            .enemy_team
            .get_active()
            .expect("Active enemy has somehow been destroyed");

        rpg_game::queue_enemy_move(enemy);

        if rpg_game::execute_moves(player, enemy) {
            return Ok(());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn battle_state() {
    // game loop
    loop {}
}
