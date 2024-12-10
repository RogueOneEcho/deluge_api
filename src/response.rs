use reqwest::StatusCode;
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub status_code: Option<u16>,
    pub result: Option<T>,
    pub error: Option<Value>,
    pub id: Option<usize>,
}

impl<T: Serialize> Response<T> {
    /// Get the result
    ///
    /// Returns an error if:
    /// - Error field is some
    /// - Status code is not set
    /// - Status code is not valid
    /// - Status code is not successful
    pub fn get_result(self, action: &str) -> Result<T, Error> {
        if let Some(error) = self.error.clone() {
            return Err(Error {
                action: action.to_owned(),
                domain: Some("Deluge API".to_owned()),
                message: format!("{error}"),
                status_code: self.status_code,
                ..Error::default()
            });
        }
        let status_code_num = self.status_code.ok_or_else(|| Error {
            action: action.to_owned(),
            domain: Some("Deluge API".to_owned()),
            message: "Status code is not set".to_owned(),
            status_code: self.status_code,
            ..Error::default()
        })?;
        let status_code = StatusCode::from_u16(status_code_num).map_err(|_| Error {
            action: action.to_owned(),
            domain: Some("Deluge API".to_owned()),
            message: "Status code is invalid".to_owned(),
            status_code: self.status_code,
            ..Error::default()
        })?;
        if !status_code.is_success() {
            let status_message = match status_code.canonical_reason() {
                None => status_code_num.to_string(),
                Some(status_message) => status_message.to_owned(),
            };
            return Err(Error {
                action: action.to_owned(),
                domain: Some("Deluge API".to_owned()),
                message: format!("Status code indicated failure: {status_message}"),
                status_code: self.status_code,
                ..Error::default()
            });
        }
        if let Some(result) = self.result {
            Ok(result)
        } else {
            Err(Error {
                action: action.to_owned(),
                domain: Some("Deluge API".to_owned()),
                message: "Result is not set".to_owned(),
                status_code: self.status_code,
                ..Error::default()
            })
        }
    }

    #[cfg(test)]
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| e.to_string())
    }
}
