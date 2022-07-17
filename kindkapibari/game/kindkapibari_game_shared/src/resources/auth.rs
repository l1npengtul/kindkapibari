use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Auth {
    game_session: String,
    refresh: String,
}
