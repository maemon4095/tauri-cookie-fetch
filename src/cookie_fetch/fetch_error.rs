#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetchError {
    url: Option<String>,
    message: String,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for FetchError {}

impl FetchError {
    pub fn from(e: reqwest::Error) -> Self {
        FetchError {
            url: e.url().map(|e| e.to_string()),
            message: format!("{}", e),
        }
    }

    pub fn from_std_error(e: &dyn std::error::Error) -> Self {
        FetchError {
            url: None,
            message: format!("{}", e),
        }
    }
}
