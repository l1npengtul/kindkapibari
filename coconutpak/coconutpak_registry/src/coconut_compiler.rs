use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CompileTask {
    data_bytes: Vec<u8>,
}

impl Deref for CompileTask {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data_bytes
    }
}

impl DerefMut for CompileTask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data_bytes
    }
}

#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CompileOutput {
    data_bytes: Vec<u8>,
}

impl Deref for CompileOutput {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data_bytes
    }
}

impl DerefMut for CompileOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data_bytes
    }
}

// Use WASMTIME and limit runtime
pub struct ThreadedCompiler {}
