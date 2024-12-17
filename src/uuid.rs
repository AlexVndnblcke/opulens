use serde::{Deserialize, Serialize};
use uuid::{Error, Uuid as _Uuid};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
pub struct Uuid(pub _Uuid);

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

// impl Deserialize for Uuid {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//             Uuid()
//     }
// }

impl Uuid {
    pub fn new_v4() -> Self {
        Uuid(_Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<Uuid, Error> {
        let _uuid = _Uuid::parse_str(input)?;
        Ok(Self(_uuid))
    }
}

impl From<Uuid> for bson::Uuid {
    fn from(value: Uuid) -> Self {
        Self::from(value.0)
    }
}
