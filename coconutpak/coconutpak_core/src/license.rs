use license::{License, ParseError};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
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
