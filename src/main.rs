use rpg_game::entity::{CppEntity, EntityBuilder, PythonEntity, RustEntity};
use rpg_game::moves::Move;
use rpg_game::ui::{Button, EntityStats};
use rpg_game::{ActionType, Team};

use macroquad::prelude::*;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq)]
enum State {
    Wait,
    Move,
    Dialogue,
    End,
}

#[macroquad::main("RPG Game")]
async fn main() {
    // create player and enemy teams
    let mut player_team = Team::new();
    player_team.push(RustEntity::build(0));
    player_team.push(PythonEntity::build(0));

    let mut enemy_team = Team::new();
    let enemy_cpp = CppEntity::build(0);
    enemy_team.push(enemy_cpp);

    // get active player and enemy.
    let mut player = player_team
        .get_active()
        .expect("Active player has somehow been destroyed");

    let mut enemy = enemy_team
        .get_active()
        .expect("Active enemy has somehow been destroyed");

    // load textures
    let empty_button_texture: Texture2D = load_texture("assets/empty-button.png").await.unwrap();
    let attack_button_texture: Texture2D = load_texture("assets/attack-button.png").await.unwrap();
    let mut attack_button = Button::new(&attack_button_texture, 1100.0, 600.0);

    let health_bar_texture: Texture2D = load_texture("assets/health-bar.png").await.unwrap();

    let player_ui = EntityStats::new(100.0, 600.0, &health_bar_texture);
    let enemy_ui = EntityStats::new(600.0, 100.0, &health_bar_texture);

    // change the window to fullscreen
    set_fullscreen(true);

    // initialize state
    let mut state = State::Wait;

    let mut text_queue: VecDeque<String> = VecDeque::new();
    let mut debounce = false;

    // game loop
    loop {
        // refresh frame
        clear_background(BLACK);

        // update player and enemy health.
        player_ui.update(
            player.health,
            player.max_health,
            format!("{}", player).as_str(),
        );
        enemy_ui.update(
            enemy.health,
            enemy.max_health,
            format!("{}", enemy).as_str(),
        );

        if state == State::Dialogue {
            dialogue(&mut text_queue, &mut state);
        }

        // waiting state code
        if state == State::Wait {
            wait(
                &mut text_queue,
                &mut state,
                &mut attack_button,
                &mut debounce,
            );
        }

        // player move state code
        if state == State::Move {
            let mut player_mv: Option<Move> = None;
            let mut move_buttons: HashMap<Move, Button> = HashMap::new();

            // add buttons for all the moves
            for (i, mv) in player.get_moves().iter().enumerate() {
                move_buttons.insert(
                    mv.clone(),
                    Button::new(&empty_button_texture, 1100.0, 600.0 + (75.0 * (i as f32))),
                );
            }

            // check if any of the buttons are clicked
            for (mv, mut button) in move_buttons.into_iter() {
                button.draw();
                if button.clicked() && !debounce {
                    debounce = true;
                    player_mv = Some(mv);
                } else if debounce {
                    debounce = false;
                }

                // draw text on top of the button
                draw_text(
                    format!("{}", mv).as_str(),
                    button.xpos + 10.0,
                    button.ypos + 40.0,
                    30.0,
                    WHITE,
                );
            }

            if let Some(mv) = player_mv {
                player.queue_move(mv);
                rpg_game::queue_enemy_move(&mut enemy);
                rpg_game::execute_moves(&mut player, &mut enemy, &mut text_queue);
                state = State::Dialogue;
            }
        }

        next_frame().await;
    }
}

fn dialogue(text_queue: &mut VecDeque<String>, state: &mut State) {
    let s = match text_queue.get(0) {
        Some(s) => s,
        None => {
            *state = State::Wait;
            return;
        }
    };
    draw_multiline_text(s, 50.0, 725.0, 40.0, Some(1.5), WHITE);

    if is_mouse_button_pressed(MouseButton::Left) {
        text_queue.pop_front();
    }
}

fn wait(
    text_queue: &mut VecDeque<String>,
    state: &mut State,
    attack_button: &mut Button,
    debounce: &mut bool,
) {
    // draw buttons.
    attack_button.draw();

    // do action.
    let mut battle_action = ActionType::Wait;
    if attack_button.clicked() && !*debounce {
        battle_action = ActionType::Attack;
        *debounce = true;
    } else if *debounce {
        *debounce = false;
    }

    if let ActionType::Attack = battle_action {
        *state = State::Move;
    }

    if let ActionType::Switch = battle_action {
        //rpg_game::switch_player(&mut player_team);
    }

    if let ActionType::Forfeit = battle_action {
        text_queue.push_back(String::from(
            "You decided that the battle was futile and quit early",
        ));
        *state = State::End;
    }
}
