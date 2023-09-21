use std::{default, mem::ManuallyDrop};

use crate::component::Components;

pub struct World {
    pub components: Components,
    systems: Vec<Box<dyn System>>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            components: Default::default(),
            systems: Vec::new(),
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

    pub fn add_system<F, P>(&mut self, system: F)
    where
        F: IntoSystem<P> + 'static,
    {
        self.systems.push(Box::new(system.into_system()));
    }

    pub fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.run();
        }
    }
}

pub trait System: 'static {
    fn run(&mut self);
}

trait ParamSystem<Param>: 'static {
    fn run(&mut self);
}

pub struct SystemWrapper<F: 'static, P> {
    func: F,
    _phantom: std::marker::PhantomData<P>,
}

pub trait IntoSystem<Param> {
    type Output: System;
    fn into_system(self) -> Self::Output;
}

impl<F, Param: 'static> IntoSystem<Param> for F
where
    F: ParamSystem<Param>,
{
    type Output = SystemWrapper<F, Param>;
    fn into_system(self) -> Self::Output {
        SystemWrapper {
            func: self,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, P: 'static> System for SystemWrapper<F, P>
where
    F: ParamSystem<P>,
{
    fn run(&mut self) {
        ParamSystem::run(&mut self.func);
    }
}

impl<F> ParamSystem<()> for F
where
    F: Fn() + 'static,
{
    fn run(&mut self) {
        self();
    }
}

impl<F, P1: Default> ParamSystem<(P1,)> for F
where
    F: Fn(P1) + 'static,
{
    fn run(&mut self) {
        // TODO: fetch param
        self(P1::default());
    }
}
