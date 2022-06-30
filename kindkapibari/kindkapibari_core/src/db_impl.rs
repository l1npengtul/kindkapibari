#[macro_export]
macro_rules! impl_redis {
    ($($to_impl:ty),+) => {
        $(
            impl redis::ToRedisArgs for $to_impl {
                fn write_redis_args<W>(&self, out: &mut W)
                where
                    W: ?Sized + redis::RedisWrite,
                {
                    let data = postcard::to_allocvec(&self).unwrap_or_default();
                    out.write_arg(&data);
                }
            }

            impl redis::FromRedisValue for $to_impl {
                fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                    match v {
                        redis::Value::Data(d) => {
                            Ok(postcard::from_bytes(d).map_err(|_| redis::RedisError::from( (redis::ErrorKind::ResponseError, "data deserialize") ))?)
                        }
                        _ => {
                            return Err(redis::RedisError::from( (redis::ErrorKind::TypeError, "data deserialize") ))
                        }
                    }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_sea_orm {
    ($($to_impl:ty),+) => {
        $(
impl sea_orm::TryGetable for $to_impl {
    fn try_get(
        res: &sea_orm::QueryResult,
        pre: &str,
        col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        let value = serde_json::to_value(sea_orm::entity::prelude::Json::try_get(res, pre, col)?)
            .map_err(|why| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Json(why.to_string())))?;
        serde_json::from_value(value)
            .map_err(|why| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Json(why.to_string())))
    }
}
        )+
    };
}
