use ggegui::{egui, Gui};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, Text};
use ggez::{Context, GameResult};

use crate::text::TextQueue;
use crate::Team;

use crate::entity::Entity;
use std::io;
use std::num::ParseIntError;

use rand::prelude::*;

pub struct BattleState {
    pub player_team: Team,
    pub enemy_team: Team,
    text_queue: TextQueue,
    gui: Gui,
}

impl BattleState {
    // create the state
    pub fn new(ctx: &mut Context, player_team: Team, enemy_team: Team) -> Self {
        BattleState {
            player_team,
            enemy_team,
            text_queue: TextQueue::new(),
            gui: Gui::new(ctx),
        }
    }

    // Switch the player's character.
    fn switch_player(team: &mut Team) {
        for (i, entity) in team.entities.iter().enumerate() {
            println!("{}, {}", i, entity);
        }

        let input: usize = match get_int_input() {
            Ok(i) => i,
            Err(_) => return,
        };

        team.set_active(input);
    }

    /// Execute moves of the player and enemy.
    /// Returns True if either the player or enemy died.
    fn execute_moves(player: &mut Entity, enemy: &mut Entity) -> bool {
        let player_priority = match player.get_move_priority() {
            Some(priority) => priority,
            None => 0,
        };

        let enemy_priority = match enemy.get_move_priority() {
            Some(priority) => priority,
            None => 0,
        };

        let check = |player: &Entity, enemy: &Entity| -> bool {
            if enemy.health == 0 {
                println!("The enemy has died, you win");
            } else if player.health == 0 {
                println!("The player has died, you lose");
            } else {
                return false;
            }
            true
        };

        match player_priority >= enemy_priority {
            true => {
                player.execute_move(enemy);
                if check(player, enemy) {
                    return true;
                };
                enemy.execute_move(player);
                if check(player, enemy) {
                    return true;
                };
            }
            false => {
                enemy.execute_move(player);
                if check(player, enemy) {
                    return true;
                };
                player.execute_move(enemy);
                if check(player, enemy) {
                    return true;
                };
            }
        };
        false
    }

    /// Get user input for a move, then execute it against the enemy.
    fn queue_player_move(&mut self, player: &mut Entity) {
        loop {
            // print out the options
            self.text_queue.add("Choose your move: ");
            for (i, mv) in player.get_moves().iter().enumerate() {
                self.text_queue.add(format!("{}: {}", i, mv).as_str());
            }

            // parse the input to an integer
            let i: usize = match get_int_input() {
                Ok(i) => i,
                Err(_) => continue,
            };

            // get the move associated with the index.
            let mv = match player.get_moves().get(i) {
                Some(mv) => mv.clone(),
                None => {
                    self.text_queue
                        .add("Move was not found in the player's list of moves");
                    continue;
                }
            };

            player.queue_move(mv);
            break;
        }
    }

    fn queue_enemy_move(enemy: &mut Entity) {
        let i = rand::thread_rng().gen_range(0..=enemy.get_moves().len() - 1);
        let mv = enemy.get_moves().get(i).unwrap().clone();

        enemy.queue_move(mv);
    }

    fn get_battle_action() -> Option<ActionType> {
        Some(ActionType::Attack)
    }
}

impl EventHandler<ggez::GameError> for BattleState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();
        self.text_queue.add("Choose your action");

        egui::Window::new("BattleInterface").show(&gui_ctx, |ui| {
            if self.text_queue.ready {
                if let Some(text) = self.text_queue.get_current() {
                    ui.label(text);
                }
            }
        });
        self.gui.update(ctx);

        let battle_action = if let Some(action) = Self::get_battle_action() {
            action
        } else {
            return Ok(());
        };

        if let ActionType::Switch = battle_action {
            Self::switch_player(&mut self.player_team);
        }

        let player = self
            .player_team
            .get_active()
            .expect("Active player has somehow been destroyed");

        let enemy = self
            .enemy_team
            .get_active()
            .expect("Active enemy has somehow been destroyed");

        if let ActionType::Attack = battle_action {
            self.queue_player_move(player);
        }

        if let ActionType::Forfeit = battle_action {
            self.text_queue
                .add("You decided that the battle was futile and quit early");
            self.text_queue.ready = true;
            return Ok(());
        }

        Self::queue_enemy_move(enemy);

        if Self::execute_moves(player, enemy) {
            return Ok(());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
        canvas.finish(ctx)
    }
}

enum ActionType {
    Attack,
    Switch,
    Forfeit,
}

fn get_int_input() -> Result<usize, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let input: usize = input.trim().parse()?;
    Ok(input)
}

impl BattleState {}
