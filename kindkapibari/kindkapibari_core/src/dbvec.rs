use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct DBVec<T> {
    internal: Vec<T>,
}

impl<T> DBVec<T> {
    #[must_use]
    pub fn new() -> DBVec<T> {
        DBVec {
            internal: Vec::new(),
        }
    }
}

impl<T> Default for DBVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<DBVec<T>> for Vec<T> {
    fn from(dbvec: DBVec<T>) -> Self {
        dbvec.internal
    }
}

impl<T> Deref for DBVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T> DerefMut for DBVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}

impl<T> From<Vec<T>> for DBVec<T> {
    fn from(v: Vec<T>) -> Self {
        DBVec { internal: v }
    }
}
