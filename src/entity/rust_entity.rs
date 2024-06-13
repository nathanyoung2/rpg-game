use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

pub struct RustEntity;

impl EntityBuilder for RustEntity {
    fn build(level: u32) -> Entity {
        Entity::new(
            EntityType::Rust,
            200,
            level,
            10,
            30,
            99,
            50,
            vec![Move::IntParse],
            vec![],
            vec![Move::Speed],
        )
    }
}
