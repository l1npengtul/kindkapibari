use serde::Serialize;
use staticvec::StaticVec;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct DBArray<T, const N: usize> {
    internal: StaticVec<T, N>,
}

impl<T, const N: usize> DBArray<T, N> {
    #[must_use]
    pub fn new() -> DBArray<T, N> {
        DBArray {
            internal: StaticVec::new(),
        }
    }
}

impl<T, const N: usize> Default for DBArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> From<DBArray<T, N>> for StaticVec<T, N> {
    fn from(dbarray: DBArray<T, N>) -> Self {
        dbarray.internal
    }
}

impl<T, const N: usize> Deref for DBArray<T, N> {
    type Target = StaticVec<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T, const N: usize> DerefMut for DBArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}

impl<T, const N: usize> From<[T; N]> for DBArray<T, N> {
    fn from(arr: [T; N]) -> Self {
        DBArray {
            internal: StaticVec::new_from_array(arr),
        }
    }
}
