#[macro_export]
macro_rules! impl_redis {
    ($($to_impl:ty),+) => {
        $(
            impl redis::ToRedisArgs for $to_impl {
                fn write_redis_args<W>(&self, out: &mut W)
                where
                    W: ?Sized + redis::RedisWrite,
                {
                    let _err = pot::to_writer(self, out);
                }
            }

            impl redis::FromRedisValue for $to_impl {
                fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                    match v {
                        redis::Value::Data(d) => pot::from_slice::<Self>(&d)?,
                        _ => {
                            return Err(RedisError::from(eyre::Report::msg(
                                "expected data in redis",
                            )))
                        }
                    }
                }
            }
        )+
    };
}
