use ggez::conf::{FullscreenType, WindowMode};
use ggez::{event, ContextBuilder, GameResult};
use rpg_game::entity::{CppEntity, EntityBuilder, PythonEntity, RustEntity};
use rpg_game::{BattleState, Team};

fn main() -> GameResult {
    // create player and enemy for prototype.
    let mut player_team = Team::new();
    player_team.push(RustEntity::build(0));
    player_team.push(PythonEntity::build(0));

    let mut enemy_team = Team::new();
    enemy_team.push(CppEntity::build(0));

    // Window Settings
    let window_mode = WindowMode {
        borderless: true,
        maximized: true,
        fullscreen_type: FullscreenType::Windowed,
        ..Default::default()
    };

    let (mut ctx, event_loop) = ContextBuilder::new("rpg_game", "Nathan Young")
        .window_mode(window_mode)
        .build()?;

    let state = BattleState::new(&mut ctx, player_team, enemy_team);
    event::run(ctx, event_loop, state)
}
