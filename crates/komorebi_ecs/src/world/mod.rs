use std::mem::ManuallyDrop;

use crate::component::Components;

pub struct World {
    pub components: Components,
}

impl Default for World {
    fn default() -> Self {
        Self {
            components: Default::default(),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_non_send_resource<R: 'static>(&mut self, resource: R) {
        // let component_id = self.components.init_non_send::<R>();
        // TODO: insert resource
    }
}
