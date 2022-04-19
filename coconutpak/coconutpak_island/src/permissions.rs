use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Permissions {
    Server,
    Admin,
    Demote,
    Promote,
    YankSelf,
    Publish,
    Download,
}
