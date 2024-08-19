use crate::entity::Entity;

pub struct Team {
    pub entities: Vec<Entity>,
    active: usize,
}

impl Team {
    pub fn new() -> Self {
        Team {
            entities: Vec::with_capacity(6),
            active: 0,
        }
    }

    pub fn set_active(&mut self, index: usize) -> Result<(), ()> {
        if index < self.entities.len() {
            self.active = index;
            return Ok(());
        }
        Err(())
    }

    pub fn get_active(&mut self) -> Option<&mut Entity> {
        Some(&mut self.entities[self.active])
    }

    pub fn get_active_index(&self) -> usize {
        self.active
    }

    pub fn push(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{EntityBuilder, EntityType, PythonEntity, RustEntity};

    #[test]
    fn get_active_test() {
        let mut team = Team::new();
        let rust = RustEntity::build(0, None);
        let python = PythonEntity::build(0, None);

        team.push(rust);
        team.push(python);

        assert_eq!(EntityType::Rust, team.get_active().unwrap().entity_type);
        let _ = team.set_active(1);
        assert_eq!(EntityType::Python, team.get_active().unwrap().entity_type);
    }
}
