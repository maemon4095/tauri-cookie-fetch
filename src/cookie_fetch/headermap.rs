use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use reqwest::header::{HeaderName, HeaderValue};
use serde::{
    de::DeserializeSeed,
    ser::{SerializeMap, SerializeSeq},
};

#[derive(Debug)]
pub struct HeaderMap(reqwest::header::HeaderMap);

impl Deref for HeaderMap {
    type Target = reqwest::header::HeaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for HeaderMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl HeaderMap {
    pub fn new() -> Self {
        Self(reqwest::header::HeaderMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(reqwest::header::HeaderMap::with_capacity(capacity))
    }
}

impl Into<reqwest::header::HeaderMap> for HeaderMap {
    fn into(self) -> reqwest::header::HeaderMap {
        self.0
    }
}

impl From<reqwest::header::HeaderMap> for HeaderMap {
    fn from(value: reqwest::header::HeaderMap) -> Self {
        Self(value)
    }
}

impl serde::Serialize for HeaderMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut headermap = serializer.serialize_map(Some(self.len()))?;

        for key in self.keys() {
            headermap.serialize_entry(key.as_str(), &GetAllWrapper(self.get_all(key)))?;
        }

        headermap.end()
    }
}

struct GetAllWrapper<'a>(reqwest::header::GetAll<'a, HeaderValue>);

impl<'a> serde::Serialize for GetAllWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for val in self.0.iter() {
            match val.to_str() {
                Ok(s) => seq.serialize_element(s)?,
                Err(e) => return Err(<S::Error as serde::ser::Error>::custom(e.to_string())),
            }
        }
        seq.end()
    }
}

impl<'de> serde::Deserialize<'de> for HeaderMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'a> serde::de::Visitor<'a> for Visitor {
            type Value = HeaderMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("reqwest::header::HeaderMap")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                let mut map = match access.size_hint() {
                    Some(s) => HeaderMap::with_capacity(s),
                    None => HeaderMap::new(),
                };

                loop {
                    let Some(key): Option<&str> = access.next_key()? else {
                        break;
                    };
                    let key = match HeaderName::from_str(key) {
                        Ok(k) => k,
                        Err(e) => {
                            return Err(<A::Error as serde::de::Error>::custom(e.to_string()));
                        }
                    };

                    access.next_value_seed(HeaderMapValueSeed(key, &mut map))?
                }

                Ok(map)
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

struct HeaderMapValueSeed<'a>(HeaderName, &'a mut HeaderMap);
impl<'de, 'a> DeserializeSeed<'de> for HeaderMapValueSeed<'a> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<'a>(HeaderName, &'a mut HeaderMap);

        impl<'a, 'de> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = ();

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("sequence of reqwest::header::HeaderValue")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                while let Some(v) = seq.next_element::<&str>()? {
                    match HeaderValue::from_str(v) {
                        Ok(v) => self.1.append(&self.0, v),
                        Err(e) => {
                            return Err(<A::Error as serde::de::Error>::custom(e.to_string()))
                        }
                    };
                }

                Ok(())
            }
        }

        deserializer.deserialize_seq(Visitor(self.0, self.1))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn deserialize_headermap() {
        let result = serde_json::from_str(r#"{"k":["v0","v1","v2"]}"#);
        let headermap: HeaderMap = result.unwrap();

        let set: HashSet<&str> = headermap
            .get_all("k")
            .iter()
            .map(|e| e.to_str().unwrap())
            .collect();

        assert_eq!(set, HashSet::from(["v0", "v1", "v2"]));
    }

    #[test]
    fn serialize_headermap() {
        let mut map = HeaderMap::new();

        map.append("k", HeaderValue::from_str("v0").unwrap());
        map.append("k", HeaderValue::from_str("v1").unwrap());
        map.append("k", HeaderValue::from_str("v2").unwrap());

        let result = serde_json::to_string(&map).unwrap();

        assert_eq!(&result, r#"{"k":["v0","v1","v2"]}"#);
    }
}
