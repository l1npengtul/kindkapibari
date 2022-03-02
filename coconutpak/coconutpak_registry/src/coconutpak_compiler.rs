use crate::config::ServerCfg;
use atomic_enum::atomic_enum;
use flume::{Receiver, Sender};
use rusty_pool::ThreadPool;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use bson::Document;
use chrono::{DateTime, Utc};
use tabox::configuration::SandboxConfiguration;
use uuid::Uuid;
use coconutpak_compiler::error::CompilerError;

#[atomic_enum]
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompileStatus {
    Rejected,
    Queued,
    Running,
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileTask {
    pub package_name: String,
    pub package_id: Uuid,
    pub package_version: Version,
    #[serde(skip)]
    pub bytes_snap: Vec<u8>,
}

impl PartialEq for CompileTask {
    fn eq(&self, other: &Self) -> bool {
        self.package_id.eq(&other.package_id) && self.package_version.eq(&other.package_version)
    }
}

impl Eq for CompileTask {}

impl PartialOrd for CompileTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.package_id.partial_cmp(&other.package_id) {
            Some(o) => Some(o),
            None => self.package_version.partial_cmp(&other.package_version),
        }
    }
}

impl Hash for CompileTask {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.package_id.hash(state);
        self.package_version.hash(state);
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompileTaskHandler {
    pub package_name: String,
    pub package_id: Uuid,
    pub package_version: Version,
    pub status: AtomicCompileStatus,
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CompletedTask {
    pub package_name: String,
    pub package_id: Uuid,
    pub package_version: Version,
    pub bson_bytes_snap: Document,
    pub time_started: DateTime<Utc>,
    pub time_completed: DateTime<Utc>,
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct FailedTask {
    pub package_name: String,
    pub package_id: Uuid,
    pub package_version: Version,
    pub error: CompilerError,
    pub logs: String,
    pub time_started: DateTime<Utc>,
    pub time_completed: DateTime<Utc>,
}

pub struct CompilerService {
    server_config: Arc<ServerCfg>,
    sandbox_config: SandboxConfiguration,
    pending: Vec<CompileTask>,
    task_sender: Sender<CompileTask>,
    task_receiver_thread: Receiver<CompileTask>,
    coconut_receiver: Receiver<Result<>>
    pool: ThreadPool,
}
