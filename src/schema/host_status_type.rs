use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug)]
pub enum HostStatusType {
    Online,
    Connected,
    Other(String),
}

impl<'de> Deserialize<'de> for HostStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let status: String = Deserialize::deserialize(deserializer)?;
        Ok(match status.as_str() {
            "Online" => HostStatusType::Online,
            "Connected" => HostStatusType::Connected,
            other => HostStatusType::Other(other.to_owned()),
        })
    }
}

impl Serialize for HostStatusType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            HostStatusType::Online => serializer.serialize_str("Online"),
            HostStatusType::Connected => serializer.serialize_str("Connected"),
            HostStatusType::Other(value) => serializer.serialize_str(value),
        }
    }
}
