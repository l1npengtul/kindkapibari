use crate::schema::{users::*, *};
use poem::http::Request;
use poem_openapi::auth::Bearer;
use poem_openapi::{OAuthScopes, SecurityScheme};

pub enum KKBOAuthScopes {
    PublicRead,
    PublicWrite,
    BadgesRead,
    EmailRead,
    EmailWrite,
    ConnectionsRead,
    PreferencesRead,
    PreferencesWrite,
    UserdataRead,
    UserdataWrite,
    ApplicationsRead,
    ApplicationsWrite,
    Records,
}

struct KKBOAuthAuthorization(user::Model);
