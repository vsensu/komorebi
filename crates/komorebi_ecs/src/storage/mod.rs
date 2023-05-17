mod vec;

pub use vec::*;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use downcast_rs::{impl_downcast, Downcast};
use hibitset::{BitSet, BitSetLike};

pub trait AbstractStorage: Downcast + Send + 'static {
    fn free(&mut self, i: u32);
}
impl_downcast!(AbstractStorage);

impl<S: Storage> AbstractStorage for Masked<S> {
    fn free(&mut self, i: u32) {
        self.remove(i);
    }
}

pub trait Storage: Default + Send + 'static {
    type Component;

    unsafe fn insert(&mut self, i: u32, x: Self::Component);
    unsafe fn remove(&mut self, i: u32) -> Self::Component;
    unsafe fn get(&self, i: u32) -> &Self::Component;
    unsafe fn get_mut(&mut self, i: u32) -> &mut Self::Component;
}

pub struct Masked<S: Storage> {
    inner: S,
    mask: BitSet,
}

impl<S: Storage> Masked<S> {
    pub fn new(x: S) -> Self {
        Self {
            inner: x,
            mask: BitSet::new(),
        }
    }

    pub fn insert(&mut self, i: u32, x: S::Component) -> Option<S::Component> {
        unsafe {
            let old = match self.mask.add(i) {
                true => Some(self.inner.remove(i)),
                false => None,
            };
            self.inner.insert(i, x);
            old
        }
    }

    pub fn remove(&mut self, i: u32) -> Option<S::Component> {
        unsafe {
            match self.mask.remove(i) {
                true => Some(self.inner.remove(i)),
                false => None,
            }
        }
    }
}

impl<S: Storage> Drop for Masked<S> {
    fn drop(&mut self) {
        for i in (&self.mask).iter() {
            unsafe {
                self.inner.remove(i);
            }
        }
    }
}

pub struct StorageRefMut<'a, S> {
    guard: MutexGuard<'a, Box<dyn AbstractStorage>>,
    marker: PhantomData<S>,
}

impl<'a, S> StorageRefMut<'a, S> {
    pub fn new(guard: MutexGuard<'a, Box<dyn AbstractStorage>>) -> Self {
        Self {
            guard,
            marker: PhantomData,
        }
    }
}

impl<'a, S: Storage> Deref for StorageRefMut<'a, S> {
    type Target = Masked<S>;
    fn deref(&self) -> &Masked<S> {
        (**self.guard).downcast_ref::<Masked<S>>().unwrap()
    }
}

impl<'a, S: Storage> DerefMut for StorageRefMut<'a, S> {
    fn deref_mut(&mut self) -> &mut Masked<S> {
        self.guard.downcast_mut::<Masked<S>>().unwrap()
    }
}
