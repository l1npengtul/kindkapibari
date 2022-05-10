use sea_orm::{
    sea_query::{ColumnType, Nullable, ValueType, ValueTypeErr},
    DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};

pub struct DBVec<T> {
    internal: Vec<T>,
}

impl<T> DBVec<T> {
    pub fn new() -> DBVec<T> {
        DBVec {
            internal: Vec::new(),
        }
    }
}

impl<T> From<DBVec<T>> for Vec<T> {
    fn from(dbvec: DBVec<T>) -> Self {
        dbvec.internal
    }
}

impl<T> Deref for DBVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T> DerefMut for DBVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}

impl<T> Serialize for DBVec<T>
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

impl<'de, T> Deserialize<'de> for DBVec<T>
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

impl<T> From<DBVec<T>> for Value
where
    T: Serialize,
{
    fn from(db: DBVec<T>) -> Self {
        Value::Bytes(pot::to_vec(&db.internal).ok().map(Box::new))
    }
}

impl<T> TryGetable for DBVec<T>
where
    T: Serialize,
{
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        pot::from_slice(&Vec::<u8>::try_get(res, pre, col)?)
            .map_err(|why| TryGetError::DbErr(DbErr::Custom(why.to_string())))
    }
}

impl<T> ValueType for DBVec<T>
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
        format!("DBVec<{}>", stringify!(T))
    }

    fn column_type() -> ColumnType {
        ColumnType::Binary(None)
    }
}

impl<T> Nullable for DBVec<T> {
    fn null() -> Value {
        Value::Bytes(None)
    }
}

impl<T> From<Vec<T>> for DBVec<T> {
    fn from(v: Vec<T>) -> Self {
        DBVec { internal: v }
    }
}

impl<T> From<DBVec<T>> for Vec<T> {
    fn from(dbvec: DBVec<T>) -> Self {
        dbvec.internal
    }
}
