use super::{Entity, EntityBuilder, EntityType};
use crate::moves::Move;

use macroquad::texture::Texture2D;

pub struct PythonEntity;

impl EntityBuilder for PythonEntity {
    fn build(level: u32, texture: Option<Texture2D>) -> Entity {
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
            texture,
        )
    }
}
