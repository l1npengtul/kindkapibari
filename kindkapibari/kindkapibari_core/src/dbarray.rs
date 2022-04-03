use sea_orm::{
    sea_query::{ColumnType, Nullable, ValueType, ValueTypeErr},
    DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use staticvec::StaticVec;
use std::ops::{Deref, DerefMut};

pub struct DBArray<T, const N: usize> {
    internal: StaticVec<T, N>,
}

impl<T, const N: usize> DBArray<T, N> {
    pub fn new() -> DBArray<T, N> {
        DBArray {
            internal: StaticVec::new(),
        }
    }
}

impl<T, const N: usize> From<DBArray<T, N>> for StaticVec<T, N> {
    fn from(dbarray: DBArray<T, N>) -> Self {
        dbarray.internal
    }
}

impl<T, const N: usize> Deref for DBArray<T, N> {
    type Target = StaticVec<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T, const N: usize> DerefMut for DBArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}

impl<T, const N: usize> Serialize for DBArray<T, N>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let bytes = match pot::to_vec(&self.internal) {
            Ok(b) => b,
            Err(why) => return Err(Error::custom(why)),
        };

        serializer.serialize_bytes(&bytes)
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for DBArray<T, N>
where
    T: Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        pot::from_slice::<Self>(&Vec::<u8>::deserialize(deserializer)?)
            .map_err(|why| Error::custom(why))
    }
}

impl<T, const N: usize> From<DBArray<T, N>> for sea_orm::Value
where
    T: Serialize,
{
    fn from(db: DBArray<T, N>) -> Self {
        sea_orm::Value::Bytes(pot::to_vec(&db.internal).ok().map(Box::new))
    }
}

impl<T, const N: usize> TryGetable for DBArray<T, N>
where
    T: Serialize,
{
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        pot::from_slice(&Vec::<u8>::try_get(res, pre, col)?)
            .map_err(|why| TryGetError::DbErr(DbErr::Custom(why.to_string())))
    }
}

impl<T, const N: usize> ValueType for DBArray<T, N>
where
    T: Serialize,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Bytes(Some(bytes)) => pot::from_slice::<Self>(&bytes).map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        format!("DBArray<{}, {}>", stringify!(T), N.to_string())
    }

    fn column_type() -> ColumnType {
        ColumnType::Binary(None)
    }
}

impl<T, const N: usize> Nullable for DBArray<T, N> {
    fn null() -> Value {
        Value::Bytes(None)
    }
}
