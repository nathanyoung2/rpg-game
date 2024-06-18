use crate::entity::{Entity, EntityType, Stat};
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Move {
    IntParse,
    Speed,
    MultiThread,
    Deadline,
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
            Move::IntParse => int_parse_move(caller, enemy, attack_multiplier * multiplier),
            Move::Speed => speed_move(caller, enemy, attack_multiplier * multiplier),
            Move::MultiThread => multi_thread_move(enemy),
            Move::Deadline => deadline_move(caller, enemy, attack_multiplier * multiplier),
        }

        if is_not_effective {
            println!("It wasn't very effective");
        }

        if is_super_effective {
            println!("It was super effective");
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

fn deadline_move(caller: &mut Entity, enemy: &mut Entity, attack_multiplier: f64) {
    const DAMAGE: f64 = 30.0;
    println!("{} and {} needed to meet a deadline", caller, enemy);
    println!(
        "Due to {}'s simplicity, it was able to make the deadline",
        caller
    );
    println!("{} took damage", enemy);
    enemy.damage((DAMAGE * attack_multiplier) as u32, Some(Move::Deadline));
}

/// Execute the 'Multi Thread' move
fn multi_thread_move(enemy: &mut Entity) {
    // Define constants
    const ACCURACY_CHANGE: i32 = -10;

    // check if the move has no effect.
    if let EntityType::Rust = enemy.entity_type {
        println!("But it had no effect");
        return;
    };

    println!(
        "A race condition was overlooked, the enemy {}'s accuracy has fallen due to undefined behavior",
        enemy
    );

    // execute the move
    enemy.change_stat(Stat::Accuracy, ACCURACY_CHANGE);
}

/// Execute the 'Compile Fast' move.
fn speed_move(caller: &mut Entity, enemy: &mut Entity, attack_multiplier: f64) {
    // define damage constant.
    const DAMAGE: f64 = 35.0;
    println!(
        "{} Showed of it's fast compile time and attacked {} first",
        caller, enemy
    );
    // attack the opponent.
    enemy.damage((DAMAGE * attack_multiplier) as u32, Some(Move::Speed));
}

/// Execute the 'Parse an Integer' move.
fn int_parse_move(caller: &mut Entity, enemy: &mut Entity, attack_multiplier: f64) {
    // define damage constants.
    const LARGER_DAMAGE: u32 = 50;
    const SMALLER_DAMAGE: u32 = 15;

    println!(
        "A string needs to be parsed into an integer. This may cause an error! {} and {} attempt to handle it",
        caller, enemy
    );

    // closure that returns the entity specific dialogue.
    let get_specific_dialogue = |name: &EntityType| {
        return match name {
            EntityType::Rust => "Rust returned a result type that can be matched on.",
            EntityType::Cpp => "C++ needed a catch block to avoid a runtime error.",
            EntityType::Python => "Python needed a catch block to avoid a runtime error.",
        };
    };

    // get the entity specific dialogue and print it out.
    println!("{}", get_specific_dialogue(&caller.entity_type));
    println!("{}", get_specific_dialogue(&enemy.entity_type));

    // the move has different functionality depending on who's error handling stat is higher.
    if caller.error_handling > enemy.error_handling {
        // in the case that the opponent has a lower error handling stat, it takes more damage.
        println!("{} handled the error better than {}", caller, enemy);

        // deal damage
        caller.damage(SMALLER_DAMAGE, None);
        enemy.damage(
            (LARGER_DAMAGE as f64 * attack_multiplier) as u32,
            Some(Move::IntParse),
        );
        println!("{} took more damage", enemy);
    } else if enemy.error_handling > caller.error_handling {
        // in the case that the opponent has a higher enemy stat, the caller takes more damage.
        println!("{} handled the error better than {}", enemy, caller);

        // deal damage
        caller.damage(LARGER_DAMAGE, None);
        enemy.damage(
            (SMALLER_DAMAGE as f64 * attack_multiplier) as u32,
            Some(Move::IntParse),
        );
        println!("{} took more damage", caller);
    } else {
        // in the case where they both have equal error handling stats,
        // lower damage is dealt to both
        println!("Both languages have equal error handling abilities");

        // deal damage
        caller.damage(SMALLER_DAMAGE, None);
        enemy.damage(
            (SMALLER_DAMAGE as f64 * attack_multiplier) as u32,
            Some(Move::IntParse),
        );
    }
}
