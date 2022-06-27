use kindkapibari_core::{impl_attr_err, AttrString};
use oxide_auth::primitives::scope::Scope;
use poem_openapi::OAuthScopes;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

pub fn make_kkbscope_scope(kkb: impl AsRef<[KKBScope]>) -> Scope {
    kkb.as_ref()
        .iter()
        .map(|x| x.to_attr_string())
        .join(" ")
        .parse::<Scope>()
        .unwrap()
}
