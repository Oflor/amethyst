//! The game world in which entities reside.

use super::dynvec::DynVec;

use std::any::{Any, TypeId};
use std::collections::{HashMap, BTreeMap};

pub type Entity = u64;

type Components = HashMap<TypeId, DynVec>;
type EntityData = HashMap<TypeId, usize>;

/// A collection of entities and their respective components.
#[derive(Debug, Clone)]
pub struct World {
    components: Components,
    ent_data: BTreeMap<Entity, EntityData>,
    next_ent: Entity,
}

impl World {
    /// Creates a new empty world.
    pub fn new() -> World {
        World {
            components: Components::new(),
            ent_data: BTreeMap::new(),
            next_ent: 0,
        }
    }

    /// Creates a new entity in the world and returns a handle to it.
    pub fn create_entity(&mut self) -> Entity {
        self.ent_data.insert(self.next_ent, EntityData::new());
        self.next_ent += 1;
        self.next_ent - 1
    }

    /// Destroys a given entity and removes its components.
    pub fn destroy_entity(&mut self, entity: Entity) {
        if let Some(data) = self.ent_data.remove(&entity) {
            for (a, b) in data {
                self.remove_component_type(a, entity);
            }
        }
    }

    /// Attaches a component to an entity and returns the component's index.
    pub fn insert_component<T: Any>(&mut self, entity: Entity, comp: T) -> Option<usize> {
        let ent_data: &mut EntityData = match self.ent_data.get_mut(&entity) {
            Some(s) => s,
            None => return None, // Return None, if no such entity exists
        };
        let t = TypeId::of::<(Entity, T)>();
        if !ent_data.contains_key(&t) {
            // If the list of type T already exists, we add our new component to it
            if let Some(c) = self.components.get_mut(&t) {
                let id = c.add((entity, comp));
                ent_data.insert(t, id); // Add new component's id to the entitiy's metadata
                return Some(id);
            }
            // Otherwise, we create a new list of type T with the new component and add it to our world
            let mut vec = DynVec::new::<(Entity, T)>();
            vec.add((entity, comp));
            self.components.insert(t, vec);
            ent_data.insert(t, 0);
            Some(0)
        } else {
            None // Return None, if this entity already has this component.
        }
    }

    pub fn remove_component<T: Any>(&mut self, entity: Entity) {
        self.remove_component_type(TypeId::of::<(Entity, T)>(), entity);
    }

    fn remove_component_type(&mut self, t: TypeId, entity: Entity) {
        use std::ops::IndexMut;
        let id = self.ent_data.get_mut(&entity).unwrap().remove(&t);
        if let (Some(c), Some(i)) = (self.components.get_mut(&t), id) {
            c.remove(i);
        }
    }

    /// Returns ith component of selected type
    pub fn component<T: Any>(&self, index: usize) -> Option<&(Entity, T)> {
        if let Some(c) = self.components.get(&TypeId::of::<(Entity, T)>()) {
            c.get_component(index)
        } else {
            None
        }
    }

    /// Returns ith mutable component of selected type
    pub fn component_mut<T: Any>(&mut self, index: usize) -> Option<&mut (Entity, T)> {
        if let Some(mut c) = self.components.get_mut(&TypeId::of::<(Entity, T)>()) {
            c.get_component_mut(index)
        } else {
            None
        }
    }

    pub fn is_alive(&self, ent: Entity) -> bool {
        self.ent_data.contains_key(&ent)
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_removing() {
        let mut w = World::new();

        let first = w.create_entity();
        let second = w.create_entity();
        let third = w.create_entity();
        assert!(first < second);
        assert!(second < third);
        assert!(w.is_alive(first));
        assert!(w.is_alive(second));
        assert!(w.is_alive(third));
        w.destroy_entity(second);
        assert!(!w.is_alive(second));
        let fourth = w.create_entity();
        assert!(fourth > third);
    }
}
