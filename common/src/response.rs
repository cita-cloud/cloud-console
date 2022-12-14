use serde::Serialize;

use crate::status_code::StatusCode;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub code: StatusCode,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub fn new(code: StatusCode, data: T) -> Response<T> {
        Self {
            code,
            msg: format!("{:?}", code),
            data: Some(data),
        }
    }

    pub fn success(data: T) -> Response<T> {
        Self::new(StatusCode::Success, data)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl From<StatusCode> for Response<()> {
    fn from(code: StatusCode) -> Self {
        Self {
            code,
            msg: format!("{:?}", code),
            data: None,
        }
    }
}

#[test]
fn test() {
    println!(
        "{}",
        serde_json::to_string_pretty(&std::convert::Into::<Response<()>>::into(
            StatusCode::Success
        ))
        .unwrap()
    );
}
