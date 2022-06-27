use crate::{access::auth::login::verify_login_token, user};
use kindkapibari_core::{
    auth::{FromAuth, Located},
    secret::SentSecret,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AuthedUser(user::Model);

#[async_trait::async_trait]
impl FromAuth for AuthedUser {
    const LOCATION: Located = Located::Cookie(Cow::from("kkb_lgtkn"));

    async fn from_auth(provided: String) -> Option<Self> {
        let sent_token = SentSecret::from_str_token(provided)?;
        verify_login_token(crate::SERVERSTATE.get()?.clone(), sent_token)
            .await
            .ok()
            .flatten()
            .into()
    }
}

impl From<user::Model> for AuthedUser {
    fn from(m: user::Model) -> Self {
        Self(m)
    }
}

impl Deref for AuthedUser {
    type Target = user::Model;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AuthedUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
