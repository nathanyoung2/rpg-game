use super::{Entity, EntityType};
use crate::moves::Move;

pub struct RustEntity;

const MOVES: Vec<Move> = vec![];
const WEAKNESSES: Vec<Move> = vec![];

impl EntityType for RustEntity {
    fn new(level: u32) -> Entity {
        Entity::new("Rust", 200, level, 10, 30, 99, MOVES, WEAKNESSES)
    }
}
