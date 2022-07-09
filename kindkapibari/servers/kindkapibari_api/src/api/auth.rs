use crate::{access::user::user_by_id, SERVERSTATE};
use kindkapibari_core::{
    auth::{FromAuth, Located},
    secret::decode_access_token,
};
use kindkapibari_schema::schema::users::user;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserAuthMdl(pub user::Model);

impl Deref for UserAuthMdl {
    type Target = user::Model;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UserAuthMdl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait::async_trait]
impl FromAuth for UserAuthMdl {
    const LOCATION: Located = Located::Header(Cow::Borrowed("Authorization"));

    async fn from_auth(provided: String) -> Option<Self> {
        let split = provided.split_once(' ');
        match split {
            Some((name, content)) => {
                if name == "Bearer" {
                    let server_state = SERVERSTATE.get()?;
                    let claims = decode_access_token(
                        content,
                        server_state
                            .config
                            .read()
                            .await
                            .signing_keys
                            .login_key
                            .as_bytes(),
                    )
                    .ok()?;
                    return Some(
                        user_by_id(server_state.clone(), claims.user_id)
                            .await
                            .ok()?
                            .into(),
                    );
                }
                None
            }
            _ => None,
        }
    }
}

impl From<user::Model> for UserAuthMdl {
    fn from(m: user::Model) -> Self {
        Self(m)
    }
}

impl From<UserAuthMdl> for user::Model {
    fn from(uam: UserAuthMdl) -> Self {
        uam.0
    }
}
