use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClientConnectionStateUpdate {
    pub new_state: ConnectionState,
    pub message: String,
}
