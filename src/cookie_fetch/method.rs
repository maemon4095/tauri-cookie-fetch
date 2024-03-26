use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug)]
pub struct Method(reqwest::Method);

#[allow(unused)]
impl Method {
    pub const GET: Method = Method(reqwest::Method::GET);
    pub const POST: Method = Method(reqwest::Method::POST);
    pub const PUT: Method = Method(reqwest::Method::PUT);
    pub const DELETE: Method = Method(reqwest::Method::DELETE);
    pub const HEAD: Method = Method(reqwest::Method::HEAD);
    pub const OPTIONS: Method = Method(reqwest::Method::OPTIONS);
    pub const CONNECT: Method = Method(reqwest::Method::CONNECT);
    pub const PATCH: Method = Method(reqwest::Method::PATCH);
    pub const TRACE: Method = Method(reqwest::Method::TRACE);

    pub fn from_bytes(src: &[u8]) -> Result<reqwest::Method, tauri::http::method::InvalidMethod> {
        reqwest::Method::from_bytes(src)
    }
}

impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        self.0
    }
}

impl Deref for Method {
    type Target = reqwest::Method;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Method {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'a> serde::de::Visitor<'a> for Visitor {
            type Value = Method;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Method")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match reqwest::Method::from_str(v) {
                    Ok(v) => Ok(Method(v)),
                    Err(e) => Err(E::custom(e.to_string())),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
