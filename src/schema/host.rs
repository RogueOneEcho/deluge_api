use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub id: String,
    #[allow(clippy::struct_field_names)]
    pub host: String,
    pub port: u16,
    pub user: String,
}
