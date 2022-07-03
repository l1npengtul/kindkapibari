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

impl From<$to_impl> for sea_orm::Value {
    fn from(v: $to_impl) -> Self {
        let json = match serde_json::to_value(v).unwrap() {
            serde_json::Value::Null => sea_orm::query::JsonValue::Null,
            serde_json::Value::Bool(b) => sea_orm::query::JsonValue::Bool(b),
            serde_json::Value::Number(n) => sea_orm::query::JsonValue::Number(n),
            serde_json::Value::String(s) => sea_orm::query::JsonValue::String(s),
            serde_json::Value::Array(a) => sea_orm::query::JsonValue::Array(a),
            serde_json::Value::Object(o) => sea_orm::query::JsonValue::Object(o),
        };
        sea_orm::Value::Json(Some(Box::new(json)))
    }
}

impl sea_orm::sea_query::Nullable for $to_impl {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Json(None)
    }
}

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

impl sea_orm::sea_query::ValueType for $to_impl {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::value::ValueTypeErr> {
        let json_value = sea_orm::sea_query::value::sea_value_to_json_value(&v);
        let serde_value = match json_value {
            sea_orm::query::JsonValue::Null => serde_json::Value::Null,
            sea_orm::query::JsonValue::Bool(b) => serde_json::Value::Bool(b),
            sea_orm::query::JsonValue::Number(n) => serde_json::Value::Number(n),
            sea_orm::query::JsonValue::String(s) => serde_json::Value::String(s),
            sea_orm::query::JsonValue::Array(a) => serde_json::Value::Array(a),
            sea_orm::query::JsonValue::Object(o) => serde_json::Value::Object(o),
        };
        serde_json::from_value(serde_value)
            .map_err(|_| sea_orm::sea_query::value::ValueTypeErr)
    }

    fn type_name() -> String {
        stringify!($to_impl).to_string()
    }

    fn column_type() -> sea_orm::sea_query::table::ColumnType {
        sea_orm::sea_query::table::ColumnType::JsonBinary
    }
}

        )+
    };
}
