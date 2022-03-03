use crate::config::ServerCfg;
use ahash::RandomState;
use chrono::{DateTime, Utc};
use coconutpak_compiler::error::CompilerError;
use dashmap::DashMap;
use flume::{Receiver, Sender};
use rusty_pool::ThreadPool;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    future::Future,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileTask {
    pub build_id: Uuid,
    #[serde(skip)]
    pub bytes_snap: Vec<u8>,
}

impl PartialEq for CompileTask {
    fn eq(&self, other: &Self) -> bool {
        self.build_id.eq(&other.build_id)
    }
}

impl Eq for CompileTask {}

impl PartialOrd for CompileTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.build_id.partial_cmp(&other.build_id)
    }
}

impl Hash for CompileTask {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.build_id.hash(state);
    }
}

impl Future for CompileTask {
    type Output = Result<CompletedTask, FailedTask>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        //
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileTaskHandler {
    pub package_name: String,
    pub package_id: Uuid,
    pub package_version: Version,
    pub build_id: Uuid,
    pub status: AtomicCompileStatus,
}

impl CompileTaskHandler {
    pub fn from_build_id(build_id: Uuid) -> Self {
        // all these other fields dont matter for hash
        CompileTaskHandler {
            package_name: "".to_string(),
            package_id: Uuid::default(),
            package_version: Version::new(0, 0, 0),
            build_id,
            status: AtomicCompileStatus::from_compile_status(CompileStatus::Queued),
        }
    }
}

impl Deref for CompileTaskHandler {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.build_id
    }
}

impl DerefMut for CompileTaskHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.build_id
    }
}

impl Hash for CompileTaskHandler {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.build_id.hash(state);
    }
}

impl PartialEq for CompileTaskHandler {
    fn eq(&self, other: &Self) -> bool {
        self.build_id.eq(&other.build_id)
    }
}

impl PartialOrd for CompileTaskHandler {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.build_id.partial_cmp(&other.build_id)
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CompletedTask {
    pub build_id: Uuid,
    pub bson_bytes_snap: Vec<u8>,
    pub time_started: DateTime<Utc>,
    pub time_completed: DateTime<Utc>,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq)]
pub struct FailedTask {
    pub build_id: Uuid,
    pub error: CompilerError,
    pub logs: String,
    pub time_started: DateTime<Utc>,
    pub time_completed: DateTime<Utc>,
}

pub struct CompilerService {
    server_config: Arc<ServerCfg>,
    sandbox_config: SandboxConfiguration,
    tasks: Arc<DashMap<CompileTaskHandler, CompileTask, RandomState>>,
    task_sender: Sender<CompileTask>,
    task_receiver_thread: Receiver<CompileTask>,
    coconut_receiver: Receiver<Result<CompletedTask, FailedTask>>,
    coconut_sender_thread: Sender<Result<CompletedTask, FailedTask>>,
}

impl CompilerService {}
