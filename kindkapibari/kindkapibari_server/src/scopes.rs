use kindkapibari_core::{impl_attr_err, AttrString};
use poem_openapi::OAuthScopes;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    OAuthScopes,
    AttrString,
)]
pub enum KKBScope {
    PublicRead,
    BadgesRead,
    EmailRead,
    ConnectionsRead,
    PreferencesRead,
    UserdataRead,
    ApplicationsRead,
    RecordsRead,
    OfflineRead,
    // Danger Zone
}

impl_attr_err!(KKBScope);
