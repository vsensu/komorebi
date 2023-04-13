use crate::core;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct Column {
    data: BlobVec,
}

struct BlobVec {
    item_layout: core::Layout,
    capacity: usize,
    // Number of elements, not bytes
    len: usize,
    // the `data` ptr's layout is always `array_layout(item_layout, capacity)`
    data: core::NonNull<u8>,
    // None if the underlying type doesn't need to be dropped
    drop: Option<unsafe fn(core::OwningPtr<'_>)>,
}

pub struct SparseSet<I, V: 'static> {
    dense: Vec<V>,
    indices: Vec<I>,
    sparse: SparseArray<I, usize>,
}

struct SparseArray<I, V = I> {
    values: Vec<Option<V>>,
    marker: PhantomData<I>,
}

impl<I: SparseSetIndex, V> Default for SparseSet<I, V> {
    fn default() -> Self {
        Self {
            dense: Vec::new(),
            indices: Vec::new(),
            sparse: SparseArray::default(),
        }
    }
}

impl<I, V> SparseSet<I, V> {
    pub const fn new() -> Self {
        Self {
            dense: Vec::new(),
            indices: Vec::new(),
            sparse: SparseArray::new(),
        }
    }
}

impl<I: SparseSetIndex, V> SparseSet<I, V> {
    pub fn insert(&mut self, index: I, value: V) {
        if let Some(dense_index) = self.sparse.get(index.clone()).cloned() {
            *unsafe { self.dense.get_unchecked_mut(dense_index) } = value;
        } else {
            self.sparse.insert(index, self.dense.len());
            self.dense.push(value);
            self.indices.push(index);
        }
    }
}

impl<I: SparseSetIndex, V> SparseSet<I, V> {
    #[inline]
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    #[inline]
    pub fn contains(&self, index: I) -> bool {
        self.sparse.contains(index)
    }

    pub fn get(&self, index: I) -> Option<&V> {
        self.sparse.get(index).map(|i| &self.dense[*i])
    }

    pub fn get_mut(&mut self, index: I) -> Option<&mut V> {
        self.sparse
            .get(index)
            .map(|i| unsafe { self.dense.get_unchecked_mut(*i) })
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.dense.iter()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.dense.iter_mut()
    }
}

impl<I: SparseSetIndex, V> Default for SparseArray<I, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I, V> SparseArray<I, V> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            values: Vec::new(),
            marker: PhantomData,
        }
    }
}

impl<I: SparseSetIndex, V> SparseArray<I, V> {
    #[inline]
    pub fn contains(&self, index: I) -> bool {
        self.values[index.sparse_set_index()].is_some()
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&V> {
        self.values[index.sparse_set_index()].as_ref()
    }

    #[inline]
    pub fn insert(&mut self, index: I, value: V) {
        let index = index.sparse_set_index();
        if index >= self.values.len() {
            self.values.resize_with(index + 1, || None);
        }
        self.values[index] = Some(value);
    }

    #[inline]
    pub fn remove(&mut self, index: I) -> Option<V> {
        self.values[index.sparse_set_index()].take()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

trait SparseSetIndex: Copy + PartialEq + Eq + Hash {
    fn sparse_set_index(&self) -> usize;
    fn get_sparse_set_index(value: usize) -> Self;
}
