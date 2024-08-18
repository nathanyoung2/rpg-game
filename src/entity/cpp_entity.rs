use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

pub struct CppEntity;

impl EntityBuilder for CppEntity {
    fn build(level: u32) -> Entity {
        Entity::new(
            EntityType::Cpp,
            200,
            level,
            40,
            20,
            99,
            20,
            vec![Move::IntParse, Move::Speed],
            vec![],
            vec![Move::Speed],
        )
    }
}
