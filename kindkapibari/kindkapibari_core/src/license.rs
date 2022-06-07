#[cfg(feature = "server")]
use sea_orm::{
    sea_query::{ValueType, ValueTypeErr},
    ColumnType, QueryResult, TryGetError, TryGetable, Value,
};
use std::string::ParseError;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct SPDXLicense {
    name: String,
    id: String,
    osi: bool,
    libre: bool,
    addendum: String,
}

impl SPDXLicense {
    pub fn new<S: AsRef<str>>(spdx_str: S) -> Result<Self, ParseError> {
        spdx_str.as_ref().parse::<&dyn License>().map(|license| {
            let name = license.name().to_string();
            let id = license.id().to_string();
            let osi = license.is_osi_approved();
            let libre = license.is_fsf_libre();
            let addendum = license.see_also().join(" , ");
            SPDXLicense {
                name,
                id,
                osi,
                libre,
                addendum,
            }
        })
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn osi(&self) -> bool {
        self.osi
    }
    pub fn libre(&self) -> bool {
        self.libre
    }
    pub fn addendum(&self) -> &str {
        &self.addendum
    }
}

impl From<String> for SPDXLicense {
    fn from(spdx_id: String) -> Self {
        SPDXLicense::new(spdx_id).unwrap_or(SPDXLicense {
            name: "All Rights Reserved".to_string(),
            id: "AllRightsReserved".to_string(),
            osi: false,
            libre: false,
            addendum: "".to_string(),
        })
    }
}

impl From<SPDXLicense> for String {
    fn from(s: SPDXLicense) -> Self {
        s.id().to_owned()
    }
}

impl Default for SPDXLicense {
    fn default() -> Self {
        SPDXLicense {
            name: "All Rights Reserved".to_string(),
            id: "AllRightsReserved".to_string(),
            osi: false,
            libre: false,
            addendum: "".to_string(),
        }
    }
}

#[cfg(feature = "server")]
impl TryGetable for SPDXLicense {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        Ok(SPDXLicense::from(String::try_get(res, pre, col)?))
    }
}

#[cfg(feature = "server")]
impl From<SPDXLicense> for sea_orm::Value {
    fn from(g: SPDXLicense) -> Self {
        sea_orm::Value::String(Some(Box::new(g.to_string())))
    }
}

#[cfg(feature = "server")]
impl ValueType for SPDXLicense {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(gender_str)) => Ok(SPDXLicense::from(gender_str.as_ref().as_str())),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(SPDXLicense).to_string()
    }

    fn column_type() -> ColumnType {
        ColumnType::Text
    }
}
