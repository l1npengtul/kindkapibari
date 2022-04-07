pub mod api_key;
pub mod bans;
pub mod coconutpak;
pub mod coconutpak_data;
pub mod coconutpak_history;
pub mod reports;
pub mod session;
pub mod subscribers;
pub mod user;

#[macro_export]
macro_rules! impl_to_redis_args {
    ($typ:ty) => {
        impl redis::ToRedisArgs for $typ {
            fn write_redis_args<W>(&self, out: &mut W)
            where
                W: ?Sized + RedisWrite,
            {
                out.write_arg(&pot::to_vec(self).unwrap_or_default())
            }
        }
    };
}
