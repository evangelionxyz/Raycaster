use std::collections::HashMap;
use uuid::Uuid;
use crate::entity::entity::Entity;

pub struct World {
    pub entities: HashMap<Uuid, Entity>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self, name: &str) -> Uuid {
        let entity = Entity::create(name);
        let uuid = entity.uuid;
        self.entities.insert(uuid, entity);
        uuid
    }
    
    pub fn remove_entity(&mut self, uuid: Uuid) {
        self.entities.remove(&uuid);
    }
    
    pub fn get_entity(&mut self, uuid: Uuid) -> Option<&mut Entity> {
        self.entities.get_mut(&uuid)
    }
}