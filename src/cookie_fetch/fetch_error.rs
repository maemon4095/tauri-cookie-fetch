#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum FetchError {
    FromError {
        url: Option<String>,
        message: String,
    },
    NotAllowed,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::FromError { message, .. } => f.write_str(message),
            FetchError::NotAllowed => f.write_str("url not allowed on the configured scope"),
        }
    }
}

impl std::error::Error for FetchError {}

impl FetchError {
    pub fn from(e: reqwest::Error) -> Self {
        FetchError::FromError {
            url: e.url().map(|e| e.to_string()),
            message: format!("{}", e),
        }
    }

    pub fn from_std_error(e: &dyn std::error::Error) -> Self {
        FetchError::FromError {
            url: None,
            message: format!("{}", e),
        }
    }
}
