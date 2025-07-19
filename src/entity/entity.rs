use std::any::TypeId;
use std::collections::HashMap;
use uuid::Uuid;
use crate::entity::components::{Component};

pub struct Entity {
    pub uuid: Uuid,
    pub name: String,
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Entity {
    pub fn create(name: &str) -> Self {
        Entity {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn get_component<T: Component>(&self) -> Option<&T> {
        self.components.get(&TypeId::of::<T>())
            .and_then(|comp| comp.as_any().downcast_ref::<T>())
    }

    pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>())
            .and_then(|comp| comp.as_any_mut().downcast_mut::<T>())
    }

    pub fn remove_component<T: Component>(&mut self) {
        self.components.remove(&TypeId::of::<T>());
    }
}