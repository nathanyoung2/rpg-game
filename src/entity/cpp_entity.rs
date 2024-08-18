use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

use macroquad::texture::Texture2D;

pub struct CppEntity;

impl EntityBuilder for CppEntity {
    fn build(level: u32, texture: Option<Texture2D>) -> Entity {
        Entity::new(
            EntityType::Cpp,
            200,
            level,
            40,
            20,
            99,
            20,
            vec![Move::IntParse, Move::Speed],
            vec![Move::Deadline],
            vec![Move::Speed],
            texture,
        )
    }
}
