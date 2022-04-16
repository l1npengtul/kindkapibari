use sea_orm::{
    sea_query::{ColumnType, Nullable, ValueType, ValueTypeErr},
    DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::hash::Hash;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

pub type DBHashSet<T: Hash> = DBHashMap<T, ()>;

pub struct DBHashMap<K, V>
where
    K: Hash,
{
    internal: HashMap<K, V>,
}

impl<K, V> DBHashMap<K, V> {
    pub fn new() -> DBHashMap<K, V> {
        DBHashMap {
            internal: HashMap::default(),
        }
    }
}

impl<K, V> From<DBHashMap<K, V>> for HashMap<K, V> {
    fn from(dbhashmap: DBHashMap<K, V>) -> Self {
        dbhashmap.internal
    }
}

impl<K, V> Deref for DBHashMap<K, V>
where
    K: Hash,
{
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<K, V> DerefMut for DBHashMap<K, V>
where
    K: Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}

impl<K, V> Serialize for DBHashMap<K, V>
where
    K: Serialize + Hash,
    V: Serialize,
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

impl<'de, K, V> Deserialize<'de> for DBHashMap<K, V>
where
    K: Serialize + Hash,
    V: Serialize,
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

impl<K, V> From<DBHashMap<K, V>> for sea_orm::Value
where
    K: Serialize + Hash,
    V: Serialize,
{
    fn from(db: DBHashMap<K, V>) -> Self {
        sea_orm::Value::Bytes(pot::to_vec(&db.internal).ok().map(Box::new))
    }
}

impl<K, V> TryGetable for DBHashMap<K, V>
where
    K: Serialize + Hash,
    V: Serialize,
{
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        pot::from_slice(&Vec::<u8>::try_get(res, pre, col)?)
            .map_err(|why| TryGetError::DbErr(DbErr::Custom(why.to_string())))
    }
}

impl<K, V> ValueType for DBHashMap<K, V>
where
    K: Serialize + Hash,
    V: Serialize,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Bytes(Some(bytes)) => pot::from_slice::<Self>(&bytes).map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        format!("DBHashMap<{}, {}>", stringify!(K), stringify!(V))
    }

    fn column_type() -> ColumnType {
        ColumnType::Binary(None)
    }
}

impl<K, V> Nullable for DBHashMap<K, V> {
    fn null() -> Value {
        Value::Bytes(None)
    }
}
