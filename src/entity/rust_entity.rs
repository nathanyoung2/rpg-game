use super::EntityType;
use crate::moves::Move;
struct RustEntity;

const NAME: &str = "Rust";
const MAX_HEALTH: u32 = 200;
const ATTACK: u32 = 10;
const DEFENSE: u32 = 30;
const ACCURACY: u32 = 99;
const MOVES: Vec<Move> = vec![];
const WEAKNESSES: Vec<Move> = vec![];

impl EntityType for RustEntity {
    fn get_name() -> &'static str {
        NAME
    }
    fn get_max_health() -> u32 {
        MAX_HEALTH
    }
    fn get_attack() -> u32 {
        ATTACK
    }
    fn get_defense() -> u32 {
        DEFENSE
    }
    fn get_accuracy() -> u32 {
        ACCURACY
    }
    fn get_moves() -> Vec<Move> {
        MOVES
    }
    fn get_weaknesses() -> Vec<Move> {
        WEAKNESSES
    }
}
