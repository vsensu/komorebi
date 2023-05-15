mod vec;

pub use vec::*;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use downcast_rs::{impl_downcast, Downcast};
use hibitset::{BitIter, BitSet, BitSetAnd, BitSetLike};

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

impl<'a, S: Storage> IntoIterator for &'a Masked<S> {
    type Item = &'a S::Component;
    type IntoIter = SingleIter<'a, S>;

    fn into_iter(self) -> SingleIter<'a, S> {
        SingleIter {
            bits: (&self.mask).iter(),
            storage: &self.inner,
        }
    }
}

pub struct SingleIter<'a, S> {
    bits: BitIter<&'a BitSet>,
    storage: &'a S,
}

impl<'a, S: Storage> Iterator for SingleIter<'a, S> {
    type Item = &'a S::Component;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.bits.next()?;
        unsafe { Some(self.storage.get(i)) }
    }
}

#[doc(hidden)]
pub trait Get<'a>: 'a {
    type Item: 'a;
    unsafe fn get(&'a mut self, i: u32) -> Self::Item;
}

impl<'a, S: Storage> Get<'a> for &'a S {
    type Item = &'a S::Component;
    unsafe fn get(&'a mut self, i: u32) -> &'a S::Component {
        Storage::get(*self, i)
    }
}

impl<'a, S: Storage> Get<'a> for &'a mut S {
    type Item = &'a mut S::Component;
    unsafe fn get(&'a mut self, i: u32) -> &'a mut S::Component {
        Storage::get_mut(*self, i)
    }
}

#[doc(hidden)]
pub trait Join<'a>: Sized {
    type Bits: BitSetLike;
    type Get: Get<'a>;

    fn into_parts(self) -> (Self::Bits, Self::Get);

    fn join(self) -> JoinIter<'a, Self> {
        let (bits, get) = self.into_parts();
        JoinIter {
            bits: bits.iter(),
            get,
        }
    }
}

impl<'a, T: Join<'a>> Join<'a> for (T,) {
    type Bits = T::Bits;
    type Get = T::Get;
    fn into_parts(self) -> (Self::Bits, Self::Get) {
        self.0.into_parts()
    }
}

impl<'a, S: Storage> Join<'a> for &'a Masked<S> {
    type Bits = &'a BitSet;
    type Get = &'a S;
    fn into_parts(self) -> (&'a BitSet, &'a S) {
        (&self.mask, &self.inner)
    }
}

impl<'a, S: Storage> Join<'a> for &'a mut Masked<S> {
    type Bits = &'a BitSet;
    type Get = &'a mut S;
    fn into_parts(self) -> (&'a BitSet, &'a mut S) {
        (&self.mask, &mut self.inner)
    }
}

pub struct JoinIter<'a, T: Join<'a>> {
    bits: BitIter<T::Bits>,
    get: T::Get,
}

impl<'a, T: Join<'a>> Iterator for JoinIter<'a, T> {
    type Item = <T::Get as Get<'a>>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.bits.next()?;
        Some(unsafe {
            Get::get(
                // Sound because we never use the same `i` twice
                std::mem::transmute::<_, &'a mut T::Get>(&mut self.get),
                i,
            )
        })
    }
}

macro_rules! bit_and_ty {
    ($name:ty) => { $name };
    ($first:ty, $($rest:ty),+) => {
        BitSetAnd<$first, bit_and_ty!($($rest),+)>
    }
}

macro_rules! bit_and_expr {
    ($name:expr) => { $name };
    ($first:expr, $($rest:expr),+) => {
        BitSetAnd($first, bit_and_expr!($($rest),+))
    }
}

macro_rules! tuple_impl {
    ($($name: ident),*) => {
        impl<'a, $($name: Join<'a>),*> Join<'a> for ($($name),*) {
            type Bits = bit_and_ty!($($name::Bits),*);
            type Get = ($($name::Get),*);
            #[allow(non_snake_case)]
            fn into_parts(self) -> (Self::Bits, Self::Get) {
                let ($($name),*) = self;
                let ($($name),*) = ($($name.into_parts()),*);
                (bit_and_expr!($($name.0),*), ($($name.1),*))
            }
        }

        impl<'a, $($name: Get<'a>),*> Get<'a> for ($($name),*) {
            type Item = ($($name::Item),*);
            unsafe fn get(&'a mut self, i: u32) -> Self::Item {
                #[allow(non_snake_case)]
                let ($(ref mut $name),*) = self;
                ($($name.get(i)),*)
            }
        }
    }
}

tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);
tuple_impl!(A, B, C, D, E, F, G);
tuple_impl!(A, B, C, D, E, F, G, H);
tuple_impl!(A, B, C, D, E, F, G, H, I);
tuple_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF);
// tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG);
