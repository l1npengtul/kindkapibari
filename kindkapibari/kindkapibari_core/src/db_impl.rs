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
                            return Err(redis::RedisError::from(eyre::Report::msg(
                                "expected data in redis",
                            )))
                        }
                    }
                }
            }
        )+
    };
}

// TODO: Consider using JSONB?

#[macro_export]
macro_rules! impl_sea_orm {
    ($($to_impl:ty),+) => {
        $(
        impl From<$to_impl> for sea_orm::Value {
            fn from(v: $to_impl) -> Self {
                sea_orm::Value::Bytes(pot::to_vec(&v).ok().map(Box::new))
            }
        }

        impl sea_orm::TryGetable for $to_impl {
            fn try_get(res: &sea_orm::QueryResult, pre: &str, col: &str) -> Result<Self, sea_orm::error::TryGetError> {
                pot::from_slice(&Vec::<u8>::try_get(res, pre, col)?)
                    .map_err(|why| sea_orm::error::TryGetError::DbErr(sea_orm::error::DbErr::Custom(why.to_string())))
            }
        }

        impl sea_orm::ValueType for $to_impl {
            fn try_from(v: Value) -> Result<Self, sea_orm::error::ValueTypeErr> {
                match v {
                    sea_orm::Value::Bytes(Some(bytes)) => pot::from_slice::<Self>(&bytes).map_err(|_| sea_orm::error::ValueTypeErr),
                    _ => Err(sea_orm::error::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($to_impl).to_string()
            }

            fn column_type() -> sea_orm::ColumnType {
                sea_orm::ColumnType::Binary(None)
            }
        }

        impl Nullable for $to_impl {
            fn null() -> sea_orm::Value {
                sea_orm::Value::Bytes(None)
            }
        }
        )+
    };
}
