use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

use macroquad::texture::Texture2D;

pub struct JsEntity;

impl EntityBuilder for JsEntity {
    fn build(level: u32, texture: Option<Texture2D>) -> Entity {
        Entity::new(
            EntityType::Js,
            150,
            level,
            50,
            10,
            94,
            5,
            vec![Move::Deadline, Move::Async],
            vec![Move::Speed],
            vec![Move::Async],
            texture,
        )
    }
}
