#[derive(Debug)]
pub enum FetchError {
    Reqwest(reqwest::Error),
    InvalidCookieDomain(String),
    InvalidCookie { domain: String, name: String },
    InvalidUrl,
    NotAllowed,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::NotAllowed => f.write_str("url not allowed on the configured scope"),
            FetchError::InvalidCookieDomain(domain) => {
                write!(f, "invalid cookie domain `{}`", domain)
            }
            FetchError::InvalidCookie { domain, name } => {
                write!(f, "invalid cookie `{}` of domain `{}`", name, domain)
            }
            FetchError::InvalidUrl => f.write_str("invalid url"),
            FetchError::Reqwest(e) => <_ as std::fmt::Display>::fmt(e, f),
        }
    }
}
impl std::error::Error for FetchError {}
