use crate::schema::HostStatusType;
use serde::de::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct HostStatus {
    id: String,
    status: HostStatusType,
    version: String,
}

impl<'de> Deserialize<'de> for HostStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: [String; 3] = Deserialize::deserialize(deserializer)?;
        let status: HostStatusType = serde_json::from_str(&format!(r#""{}""#, array[1]))
            .map_err(|e| Error::custom(e.to_string()))?;
        Ok(HostStatus {
            id: array[0].clone(),
            status,
            version: array[2].clone(),
        })
    }
}
