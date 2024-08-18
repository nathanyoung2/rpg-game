use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

use macroquad::texture::Texture2D;

pub struct RustEntity;

impl EntityBuilder for RustEntity {
    fn build(level: u32, texture: Option<Texture2D>) -> Entity {
        Entity::new(
            EntityType::Rust,
            200,
            level,
            10,
            30,
            99,
            50,
            vec![Move::IntParse, Move::MultiThread],
            vec![],
            vec![Move::Speed],
            texture,
        )
    }
}
