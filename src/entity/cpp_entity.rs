use super::{Entity, EntityName, EntityType};
use crate::moves::Move;

pub struct CppEntity;

impl EntityType for CppEntity {
    fn new(level: u32) -> Entity {
        Entity::new(
            EntityName::Cpp,
            200,
            level,
            10,
            30,
            99,
            20,
            vec![Move::IntParse],
            vec![],
        )
    }
}
