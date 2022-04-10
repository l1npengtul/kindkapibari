use crate::config::Config;

use chrono::{DateTime, Utc};
use coconutpak_compiler::error::CompilerError;
use dashmap::DashMap;
use semver::Version;
use serde::{Deserialize, Serialize};
use sled::{Db as SledDb, Tree};
use std::{
    cmp::Ordering,
    future::Future,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::{
        atomic::{AtomicU64, AtomicU8, Ordering as AtomicOrdering},
        Arc,
    },
    task::{Context, Poll},
    thread::{sleep, JoinHandle},
    time::Duration,
};
use tabox::{
    configuration::SandboxConfiguration, result::SandboxExecutionResult, Sandbox,
    SandboxImplementation,
};
use uuid::Uuid;

pub struct BuildId {
    pub id: u64,
}

impl AsRef<[u8]> for BuildId {
    fn as_ref(&self) -> &[u8] {
        &self.id.to_ne_bytes()
    }
}

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompileTaskHandler {
    pub handle: SandboxImplementation,
}

impl Future for CompileTaskHandler {
    type Output = tabox::Result<SandboxExecutionResult>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.handle.child_thread.is_finished() {
            Poll::Ready(self.handle.wait())
        } else {
            let waker = cx.waker().clone();
            tokio::task::spawn(async || {
                tokio::time::sleep(Duration::new(1, 0));
                waker.wake();
            });
            Poll::Pending
        }
    }
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
    server_config: Arc<Config>,
    sandbox_config: SandboxConfiguration,
    id_counter: AtomicU64,
    task_metadata: Arc<DashMap<BuildId, CompileTaskMetadata>>,
    task_status: Arc<DashMap<BuildId, AtomicCompileStatus>>,
    // BuildId, Tarball with GZIP compression
    tasks: Tree,
    running_tasks: Arc<DashMap<BuildId, CompileTaskHandler>>,
}

impl CompilerService {
    pub fn new(server_config: Arc<Config>) -> Self {
        let mut sandbox = SandboxConfiguration::default();
        sandbox.time_limit(300);
        sandbox

        CompilerService {
            server_config,
            sandbox_config: ,
            id_counter: Default::default(),
            task_metadata: Arc::new(()),
            task_status: Arc::new(()),
            tasks: (),
            running_tasks: Arc::new(())
        }
    }
}
