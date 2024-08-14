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

    pub fn get_active(&mut self) -> Option<Entity> {
        Some(self.entities[self.active].clone())
    }

    pub fn get_active_index(&self) -> usize {
        self.active
    }

    pub fn push(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
