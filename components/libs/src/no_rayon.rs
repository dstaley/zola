pub mod prelude {
    pub use super::{NoRayonAHashMap, NoRayonHashMap, NoRayonSlice, NoRayonSliceMut};
}

pub trait NoRayonSlice<T> {
    fn par_iter(&self) -> core::slice::Iter<'_, T>;
}
impl<T> NoRayonSlice<T> for [T] {
    fn par_iter(&self) -> core::slice::Iter<'_, T> {
        self.iter()
    }
}

pub trait NoRayonSliceMut<T> {
    fn par_iter_mut(&mut self) -> core::slice::IterMut<'_, T>;
    fn par_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: Fn(&T, &T) -> std::cmp::Ordering;
}
impl<T> NoRayonSliceMut<T> for [T] {
    fn par_iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.iter_mut()
    }
    fn par_sort_unstable_by<F>(&mut self, compare: F)
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        self.sort_unstable_by(compare)
    }
}

pub trait NoRayonHashMap<K, V> {
    fn par_iter(&self) -> std::collections::hash_map::Iter<'_, K, V>;
}
impl<K, V> NoRayonHashMap<K, V> for std::collections::HashMap<K, V> {
    fn par_iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.iter()
    }
}

pub trait NoRayonAHashMap<K, V> {
    fn par_iter(&self) -> std::collections::hash_map::Iter<'_, K, V>;
}
impl<K, V> NoRayonAHashMap<K, V> for ahash::AHashMap<K, V> {
    fn par_iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.iter()
    }
}
