use crate::entity::{Entity, EntityType, Stat};
use std::collections::VecDeque;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Move {
    IntParse,
    Speed,
    MultiThread,
    Deadline,
    Async,
}

#[derive(Clone, Copy, Debug)]
pub struct MoveData {
    pub move_type: Move,
    pub priority: u8,
}

impl fmt::Display for Move {
    /// Implement display to be able to display the names of the moves.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::IntParse => write!(f, "Parse an integer"),
            Move::Speed => write!(f, "Compile fast"),
            Move::MultiThread => write!(f, "Multi Thread"),
            Move::Deadline => write!(f, "Deadline"),
            Move::Async => write!(f, "Asynchronous"),
        }
    }
}

impl Move {
    /// Execute itself.
    /// Each move has unique functionality.
    pub fn execute(
        &self,
        caller: &mut Entity,
        enemy: &mut Entity,
        attack_multiplier: f64,
        is_super_effective: bool,
        is_not_effective: bool,
        text_queue: &mut VecDeque<String>,
    ) {
        // get the effectiveness multiplier.
        let multiplier = if is_super_effective {
            1.5
        } else if is_not_effective {
            0.5
        } else {
            1.0
        };

        // call the functions for the move behavior
        match self {
            Move::IntParse => {
                int_parse_move(caller, enemy, attack_multiplier * multiplier, text_queue)
            }
            Move::Speed => speed_move(caller, enemy, attack_multiplier * multiplier, text_queue),
            Move::MultiThread => multi_thread_move(enemy, text_queue),
            Move::Deadline => {
                deadline_move(caller, enemy, attack_multiplier * multiplier, text_queue)
            }
            Move::Async => async_move(caller, enemy, attack_multiplier * multiplier, text_queue),
        }

        if is_not_effective {
            text_queue.push_back(format!("It wasn't very effective"));
        }

        if is_super_effective {
            text_queue.push_back(format!("It was super effective"));
        }
    }

    /// Get the defined priority of a move
    /// The default priority is zero.
    pub fn get_priority(&self) -> u8 {
        let mut priority = 0u8;
        if let Move::Speed = self {
            priority = 255u8;
        }
        priority
    }
}

fn async_move(
    caller: &mut Entity,
    enemy: &mut Entity,
    attack_multiplier: f64,
    text_queue: &mut VecDeque<String>,
) {
    const DAMAGE: f64 = 30.0;
    text_queue.push_back(format!(
        "{} unleashed attacks asynchronously, not needing to \n pause to wait for the last attack to complete.",
        caller
    ));

    if enemy.weaknesses.contains(&Move::Async) {
        text_queue.push_back(format!(
            "{} wasn't able to see the attacks coming from outside the main thread",
            enemy,
        ));
    }

    if enemy.strengths.contains(&Move::Async) {
        text_queue.push_back(format!(
            "{} was running asynchonously on all cpu cores to block the incoming attack",
            enemy
        ));
    }

    enemy.damage((DAMAGE * attack_multiplier) as u32, Some(Move::Async));
}

fn deadline_move(
    caller: &mut Entity,
    enemy: &mut Entity,
    attack_multiplier: f64,
    text_queue: &mut VecDeque<String>,
) {
    const DAMAGE: f64 = 30.0;
    text_queue.push_back(format!(
        "{} and {} needed to meet a deadline",
        caller, enemy
    ));
    text_queue.push_back(format!(
        "Due to {}'s simplicity, it was able to make the deadline",
        caller
    ));
    text_queue.push_back(format!("{} took damage", enemy));
    enemy.damage((DAMAGE * attack_multiplier) as u32, Some(Move::Deadline));
}

/// Execute the 'Multi Thread' move
fn multi_thread_move(enemy: &mut Entity, text_queue: &mut VecDeque<String>) {
    // Define constants
    const ACCURACY_CHANGE: i32 = -10;

    // check if the move has no effect.
    if let EntityType::Rust = enemy.entity_type {
        text_queue.push_back(format!("But it had no effect"));
        return;
    };

    text_queue.push_back(format!(
        "A race condition was overlooked, the enemy {}'s accuracy has \nfallen due to undefined behavior",
        enemy
    ));

    // execute the move
    enemy.change_stat(Stat::Accuracy, ACCURACY_CHANGE);
}

/// Execute the 'Compile Fast' move.
fn speed_move(
    caller: &mut Entity,
    enemy: &mut Entity,
    attack_multiplier: f64,
    text_queue: &mut VecDeque<String>,
) {
    // define damage constant.
    const DAMAGE: f64 = 25.0;
    text_queue.push_back(format!(
        "{} Showed of it's fast compile time and attacked {} first",
        caller, enemy
    ));
    // attack the opponent.
    enemy.damage((DAMAGE * attack_multiplier) as u32, Some(Move::Speed));
}

/// Execute the 'Parse an Integer' move.
fn int_parse_move(
    caller: &mut Entity,
    enemy: &mut Entity,
    attack_multiplier: f64,
    text_queue: &mut VecDeque<String>,
) {
    // define damage constants.
    const LARGER_DAMAGE: u32 = 30;
    const SMALLER_DAMAGE: u32 = 15;

    text_queue.push_back(format!(
        "A string needs to be parsed into an integer. \nThis may cause an error! {} and {} attempt to handle it",
        caller, enemy
    ));

    // closure that returns the entity specific dialogue.
    let get_specific_dialogue = |name: &EntityType| {
        return match name {
            EntityType::Rust => "Rust returned a result type that can be matched on.",
            EntityType::Cpp => "C++ needed a catch block to avoid a runtime error.",
            EntityType::Python => "Python needed a catch block to avoid a runtime error.",
            EntityType::Js => "JavaScript needed a catch block to avoid a runtime error.",
            EntityType::Go => "Go has its own error type that is nil if there was no error",
        };
    };

    // get the entity specific dialogue and print it out.
    text_queue.push_back(format!("{}", get_specific_dialogue(&caller.entity_type)));
    text_queue.push_back(format!("{}", get_specific_dialogue(&enemy.entity_type)));

    // the move has different functionality depending on who's error handling stat is higher.
    if caller.error_handling > enemy.error_handling {
        // in the case that the opponent has a lower error handling stat, it takes more damage.
        text_queue.push_back(format!(
            "{} handled the error better than {}",
            caller, enemy
        ));

        // deal damage
        caller.damage(SMALLER_DAMAGE, None);
        enemy.damage(
            (LARGER_DAMAGE as f64 * attack_multiplier) as u32,
            Some(Move::IntParse),
        );
        text_queue.push_back(format!("{} took more damage", enemy));
    } else if enemy.error_handling > caller.error_handling {
        // in the case that the opponent has a higher enemy stat, the caller takes more damage.
        text_queue.push_back(format!(
            "{} handled the error better than {}",
            enemy, caller
        ));

        // deal damage
        caller.damage(LARGER_DAMAGE, None);
        enemy.damage(
            (SMALLER_DAMAGE as f64 * attack_multiplier) as u32,
            Some(Move::IntParse),
        );
        text_queue.push_back(format!("{} took more damage", caller));
    } else {
        // in the case where they both have equal error handling stats,
        // lower damage is dealt to both
        text_queue.push_back(format!(
            "Both languages have equal error handling abilities"
        ));

        // deal damage
        caller.damage(SMALLER_DAMAGE, None);
        enemy.damage(
            (SMALLER_DAMAGE as f64 * attack_multiplier) as u32,
            Some(Move::IntParse),
        );
    }
}
