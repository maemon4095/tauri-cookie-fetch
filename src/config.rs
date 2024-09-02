use crate::scope::Scope;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub scope: Scope,
}
