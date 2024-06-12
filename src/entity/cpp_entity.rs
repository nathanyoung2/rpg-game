use super::{Entity, EntityType};
use crate::moves::Move;

pub struct CppEntity;

impl EntityType for CppEntity {
    fn new(level: u32) -> Entity {
        Entity::new("C++", 200, level, 10, 30, 99, 20, vec![Move::Error], vec![])
    }
}
