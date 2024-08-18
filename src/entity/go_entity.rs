use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

use macroquad::texture::Texture2D;

pub struct GoEntity;

impl EntityBuilder for GoEntity {
    fn build(level: u32, texture: Option<Texture2D>) -> Entity {
        Entity::new(
            EntityType::Go,
            200,
            level,
            10,
            30,
            99,
            50,
            vec![Move::IntParse, Move::MultiThread, Move::Async],
            vec![],
            vec![Move::Async],
            texture,
        )
    }
}
