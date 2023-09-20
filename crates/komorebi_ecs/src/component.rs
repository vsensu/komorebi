use std::{
    any::{Any, TypeId},
    borrow::Cow,
    collections::HashMap,
};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ComponentId(usize);

#[derive(Debug, Default)]
pub struct Components {
    components: Vec<ComponentInfo>,
    resource_indices: HashMap<TypeId, usize>,
}

impl Components {
    pub fn init_non_send<T: Any>(&mut self) -> ComponentId {
        self.get_or_insert_resource_with(TypeId::of::<T>(), || {
            ComponentDescriptor::new_non_send::<T>()
        })
    }

    fn get_or_insert_resource_with(
        &mut self,
        type_id: TypeId,
        func: impl FnOnce() -> ComponentDescriptor,
    ) -> ComponentId {
        let components = &mut self.components;
        let index = self.resource_indices.entry(type_id).or_insert_with(|| {
            let descriptor = func();
            let index = components.len();
            components.push(ComponentInfo::new(ComponentId(index), descriptor));
            index
        });
        ComponentId(*index)
    }
}

#[derive(Debug)]
pub struct ComponentInfo {
    id: ComponentId,
    descriptor: ComponentDescriptor,
}

impl ComponentInfo {
    /// Create a new [`ComponentInfo`].
    pub(crate) fn new(id: ComponentId, descriptor: ComponentDescriptor) -> Self {
        ComponentInfo { id, descriptor }
    }
}

pub struct ComponentDescriptor {
    name: Cow<'static, str>,
}

impl ComponentDescriptor {
    fn new_non_send<T: Any>() -> Self {
        Self {
            name: Cow::Borrowed(std::any::type_name::<T>()),
        }
    }
}

impl std::fmt::Debug for ComponentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentDescriptor")
            .field("name", &self.name)
            .finish()
    }
}
