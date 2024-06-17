use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

pub struct PythonEntity;

impl EntityBuilder for PythonEntity {
    fn build(level: u32) -> Entity {
        Entity::new(
            EntityType::Python,
            150,
            level,
            35,
            10,
            94,
            5,
            vec![Move::Deadline],
            vec![],
            vec![Move::Speed],
        )
    }
}
