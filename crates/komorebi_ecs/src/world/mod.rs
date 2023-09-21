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
        F: IntoSystem<P>,
        P: SystemParam,
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

trait ParamSystem<Params>: 'static {
    fn run(&mut self);
}

pub trait SystemParam: 'static {}

pub struct SystemWrapper<F: 'static, Params: SystemParam> {
    func: F,
    _phantom: std::marker::PhantomData<Params>,
}

pub trait IntoSystem<Params> {
    type Output: System;
    fn into_system(self) -> Self::Output;
}

impl<F, Params: SystemParam> IntoSystem<Params> for F
where
    F: ParamSystem<Params>,
{
    type Output = SystemWrapper<F, Params>;
    fn into_system(self) -> Self::Output {
        SystemWrapper {
            func: self,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, Params: SystemParam> System for SystemWrapper<F, Params>
where
    F: ParamSystem<Params>,
{
    fn run(&mut self) {
        ParamSystem::run(&mut self.func);
    }
}

// -------------------- closure to ParamSystem begin --------------------
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

// -------------------- closure to ParamSystem end --------------------

// -------------------- param to SystemParam begin --------------------
impl SystemParam for () {}

impl<T: 'static> SystemParam for (T,) {}
// -------------------- param to SystemParam end --------------------
