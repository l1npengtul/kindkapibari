use axum::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait Suspension: Sized {
    type USERID;
    type BANID;

    async fn latest_by_user_id(id: Self::USERID) -> Option<Self>;
    fn until(&self) -> Option<DateTime<Utc>>;
    fn reason(&self) -> Option<String>;
    fn ban_id(&self) -> Self::BANID;
    fn is_ip_ban(&self) -> bool;
}

pub struct SuspendedLayer {}
