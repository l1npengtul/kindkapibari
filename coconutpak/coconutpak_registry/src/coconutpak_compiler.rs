use crate::config::ServerCfg;

use chrono::{DateTime, Utc};
use coconutpak_compiler::error::CompilerError;
use bytes::
use semver::Version;
use serde::{Deserialize, Serialize};
use sled::Db as SledDb;
use std::sync::atomic::AtomicU64;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::{
        atomic::{AtomicU8, Ordering as AtomicOrdering},
        Arc,
    },
    task::{Context, Poll},
};
use tabox::configuration::SandboxConfiguration;
use uuid::Uuid;

pub type BuildId = u64;

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CompileStatus {
    Rejected = 0,
    Queued = 1,
    Running = 2,
    Succeeded = 3,
    Failed = 4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtomicCompileStatus {
    atomic: AtomicU8,
}

impl AtomicCompileStatus {
    pub fn from_compile_status(status: CompileStatus) -> Self {
        match status {
            CompileStatus::Rejected => AtomicCompileStatus {
                atomic: AtomicU8::new(0),
            },
            CompileStatus::Queued => AtomicCompileStatus {
                atomic: AtomicU8::new(1),
            },
            CompileStatus::Running => AtomicCompileStatus {
                atomic: AtomicU8::new(2),
            },
            CompileStatus::Succeeded => AtomicCompileStatus {
                atomic: AtomicU8::new(3),
            },
            CompileStatus::Failed => AtomicCompileStatus {
                atomic: AtomicU8::new(4),
            },
        }
    }

    pub fn as_compile_status(&self) -> CompileStatus {
        match self.atomic.load(AtomicOrdering::SeqCst) {
            0 => CompileStatus::Rejected,
            1 => CompileStatus::Queued,
            2 => CompileStatus::Running,
            3 => CompileStatus::Succeeded,
            4 => CompileStatus::Failed,
            _ => {
                panic!("Bad Compile Statis")
            }
        }
    }
}

impl Clone for AtomicCompileStatus {
    fn clone(&self) -> Self {
        AtomicCompileStatus {
            atomic: AtomicU8::new(self.atomic.load(AtomicOrdering::SeqCst)),
        }
    }
}

impl Deref for AtomicCompileStatus {
    type Target = AtomicU8;

    fn deref(&self) -> &Self::Target {
        &self.atomic
    }
}

impl DerefMut for AtomicCompileStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.atomic
    }
}

impl Hash for AtomicCompileStatus {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.atomic.load(AtomicOrdering::SeqCst).hash(state);
    }
}

impl PartialOrd for AtomicCompileStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.atomic
                .load(AtomicOrdering::SeqCst)
                .cmp(&other.atomic.load(AtomicOrdering::SeqCst)),
        )
    }
}

impl PartialEq for AtomicCompileStatus {
    fn eq(&self, other: &Self) -> bool {
        self.atomic
            .load(AtomicOrdering::SeqCst)
            .eq(&other.atomic.load(AtomicOrdering::SeqCst))
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CompileTaskMetadata {
    pub pak_id: Uuid,
    pub name: String,
    pub version: Version,
    pub account_id: Uuid,
    pub submit_datetime: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingCompileTask {
    pub build_id: BuildId,
    #[serde(skip)]
    pub bytes_snap: Vec<u8>,
}

impl PartialEq for PendingCompileTask {
    fn eq(&self, other: &Self) -> bool {
        self.build_id.eq(&other.build_id)
    }
}

impl Eq for PendingCompileTask {}

impl PartialOrd for PendingCompileTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.build_id.partial_cmp(&other.build_id)
    }
}

impl Hash for PendingCompileTask {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.build_id.hash(state);
    }
}

pub struct CompileTaskHandler {
    id: BuildId,
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CompletedTask {
    pub build_id: BuildId,
    pub bson_bytes_snap: Vec<u8>,
    pub time_started: DateTime<Utc>,
    pub time_completed: DateTime<Utc>,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq)]
pub struct FailedTask {
    pub build_id: BuildId,
    pub error: CompilerError,
    pub logs: String,
    pub time_started: DateTime<Utc>,
    pub time_completed: DateTime<Utc>,
}

pub struct CompilerService {
    server_config: Arc<ServerCfg>,
    sandbox_config: SandboxConfiguration,
    id_counter: AtomicU64,
    tasks: Arc,
}

impl CompilerService {}
