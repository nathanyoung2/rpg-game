use rpg_game::entity::{CppEntity, EntityBuilder, GoEntity, JsEntity, PythonEntity, RustEntity};
use rpg_game::moves::Move;
use rpg_game::ui::{Button, ButtonLink, EntityImageParams, EntityStats};
use rpg_game::Team;

use macroquad::prelude::*;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq, Clone)]
enum State {
    Wait,
    Move,
    Dialogue(Box<State>),
    End,
    Switch,
}

struct Battle<'a> {
    player_team: Team,
    enemy_team: Team,
    state: State,
    debounce: bool,
    debounce_step: bool,
    text_queue: VecDeque<String>,
    player_ui: EntityStats<'a>,
    enemy_ui: EntityStats<'a>,
    attack_button: Button<'a>,
    switch_button: Button<'a>,
    forfeit_button: Button<'a>,
    empty_button_texture: Texture2D,
}

#[macroquad::main("RPG Game")]
async fn main() {
    // load textures for entities
    let rust_texture = load_texture("assets/rust.png").await.ok();
    let python_texture = load_texture("assets/python.png").await.ok();
    let cpp_texture = load_texture("assets/cpp.png").await.ok();
    let js_texture = load_texture("assets/js.png").await.ok();
    let go_texture = load_texture("assets/go.png").await.ok();

    // create player and enemy teams
    let mut player_team = Team::new();
    player_team.push(RustEntity::build(0, rust_texture.clone()));
    player_team.push(PythonEntity::build(0, python_texture.clone()));

    let mut enemy_team = Team::new();
    enemy_team.push(CppEntity::build(0, cpp_texture.clone()));
    enemy_team.push(JsEntity::build(0, js_texture.clone()));
    enemy_team.push(GoEntity::build(0, go_texture.clone()));

    // load textures
    let empty_button_texture: Texture2D = load_texture("assets/empty-button.png").await.unwrap();
    let attack_button_texture: Texture2D = load_texture("assets/attack-button.png").await.unwrap();
    let switch_button_texture: Texture2D = load_texture("assets/switch-button.png").await.unwrap();
    let forfeit_button_texture: Texture2D =
        load_texture("assets/forfeit-button.png").await.unwrap();
    let health_bar_texture: Texture2D = load_texture("assets/health-bar.png").await.unwrap();

    // create ui elements
    let attack_button = Button::new(&attack_button_texture, 1100.0, 600.0);
    let switch_button = Button::new(&switch_button_texture, 1100.0, 675.0);
    let forfeit_button = Button::new(&forfeit_button_texture, 1100.0, 750.0);
    let player_ui = EntityStats::new(100.0, 600.0, &health_bar_texture);
    let enemy_ui = EntityStats::new(600.0, 100.0, &health_bar_texture);

    // change the window to fullscreen
    set_fullscreen(true);

    // initialize state
    let state = State::Wait;

    let text_queue: VecDeque<String> = VecDeque::new();
    let debounce = false;
    let debounce_step = false;

    let mut battle = Battle {
        player_team,
        enemy_team,
        debounce,
        debounce_step,
        state,
        text_queue,
        player_ui,
        enemy_ui,
        attack_button,
        switch_button,
        forfeit_button,
        empty_button_texture,
    };

    // game loop
    loop {
        // refresh frame
        clear_background(BLACK);

        if battle.update() {
            break;
        };

        next_frame().await;
    }
}

impl Battle<'_> {
    pub fn update(&mut self) -> bool {
        // update player and enemy health.
        let player = self
            .player_team
            .get_active()
            .expect("Active player has somehow been destroyed");

        let enemy = self
            .enemy_team
            .get_active()
            .expect("Active enemy has somehow been destroyed");

        self.player_ui.update(
            player.health,
            player.max_health,
            EntityImageParams {
                texture: &player.texture,
                x: 400.0,
                y: 550.0,
            },
            format!("{}", player).as_str(),
        );
        self.enemy_ui.update(
            enemy.health,
            enemy.max_health,
            EntityImageParams {
                texture: &enemy.texture,
                x: 900.0,
                y: 50.0,
            },
            format!("{}", enemy).as_str(),
        );

        match self.state {
            State::Dialogue(ref transition_state) => {
                let state_clone = transition_state.clone();
                self.dialogue(state_clone);
            }

            State::Wait => {
                self.wait_state();
            }

            State::Move => {
                self.move_state();
            }
            State::Switch => {
                self.switch_state();
            }
            State::End => {
                return true;
            }
        }

        if self.debounce && self.debounce_step {
            self.debounce = false;
            self.debounce_step = false;
        } else if self.debounce {
            self.debounce_step = true
        }

        false
    }

    fn wait_state(&mut self) {
        // draw buttons.
        self.attack_button.draw();
        self.switch_button.draw();
        self.forfeit_button.draw();

        // check for button presses and change the state accordingly
        if self.attack_button.clicked() && !self.debounce {
            self.state = State::Move;
            self.debounce = true;
        }

        if self.switch_button.clicked() && !self.debounce {
            self.state = State::Switch;
            self.debounce = true;
        }

        if self.forfeit_button.clicked() && !self.debounce {
            self.text_queue.push_back(String::from(
                "You decided that the battle was futile and quit early",
            ));
            self.state = State::Dialogue(Box::new(State::End));
            self.debounce = true;
        }
    }
    fn move_state(&mut self) {
        let mut player_mv: Option<Move> = None;
        let mut move_buttons: HashMap<Move, Button> = HashMap::new();
        let player = self.player_team.get_active().unwrap();
        let enemy = self.enemy_team.get_active().unwrap();

        // add buttons for all the moves
        for (i, mv) in player.get_moves().iter().enumerate() {
            move_buttons.insert(
                mv.clone(),
                Button::new(
                    &self.empty_button_texture,
                    1100.0,
                    550.0 + (75.0 * (i as f32)),
                ),
            );
        }

        // check if any of the buttons are clicked
        for (mv, mut button) in move_buttons.into_iter() {
            button.draw();
            if button.clicked() && !self.debounce {
                self.debounce = true;
                player_mv = Some(mv);
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
            rpg_game::queue_enemy_move(enemy);
            rpg_game::execute_moves(player, enemy, &mut self.text_queue);
            self.state = State::Dialogue(Box::new(State::Wait));
        }

        if player.health == 0 {
            self.text_queue
                .push_back(format!("The player, {}, has fallen.", player));
            if rpg_game::active_died(&mut self.player_team) {
                self.text_queue
                    .push_back(String::from("You lost the battle"));
                self.state = State::Dialogue(Box::new(State::End));
                return;
            }
        }

        if enemy.health == 0 {
            self.text_queue
                .push_back(format!("The enemy, {}, has fallen.", enemy));
            if rpg_game::active_died(&mut self.enemy_team) {
                self.text_queue
                    .push_back(String::from("You won the battle, congratulations!"));
                self.state = State::Dialogue(Box::new(State::End));
                return;
            }
        }
    }

    fn dialogue(&mut self, transition_state: Box<State>) {
        let s = match self.text_queue.get(0) {
            Some(s) => s,
            None => {
                self.state = *transition_state;
                return;
            }
        };
        draw_rectangle(0.0, 675.0, screen_width(), 200.0, BLACK);
        draw_multiline_text(s, 50.0, 725.0, 40.0, Some(1.5), WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.text_queue.pop_front();
        }
    }

    fn switch_state(&mut self) {
        let mut links: Vec<ButtonLink<usize>> = Vec::new();
        for i in 0..self.player_team.entities.len() {
            if self.player_team.entities[i].health <= 0 {
                continue;
            }
            links.push(ButtonLink {
                link: i,
                button: Button::new(
                    &self.empty_button_texture,
                    1100.0,
                    600.0 + (75.0 * (i as f32)),
                ),
            });
        }

        // check if any of the buttons are clicked
        for link in links.iter_mut() {
            link.button.draw();
            if link.button.clicked() && !self.debounce {
                self.player_team.set_active(link.link).unwrap();
                self.state = State::Wait;
                self.debounce = true;
            }

            // draw text on top of the button
            draw_text(
                format!("{}", self.player_team.entities[link.link]).as_str(),
                link.button.xpos + 10.0,
                link.button.ypos + 40.0,
                30.0,
                WHITE,
            );
        }
    }
}
