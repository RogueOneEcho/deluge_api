use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub result: bool,
    pub error: Option<Value>,
    pub id: Option<usize>,
}

impl Display for ApiResponse {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).unwrap_or_else(|e| e.to_string());
        write!(formatter, "{json}")
    }
}
