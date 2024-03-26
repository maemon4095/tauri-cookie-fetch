#[derive(Debug)]
pub enum Redirect {
    Follow,
    Manual,
    Limit { limit: usize },
}

impl<'de> serde::Deserialize<'de> for Redirect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KeyLimit;
        impl<'de> serde::de::Deserialize<'de> for KeyLimit {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = KeyLimit;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`limit`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "limit" => Ok(KeyLimit),
                            _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &self)),
                        }
                    }
                }

                deserializer.deserialize_str(V)
            }
        }

        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Redirect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`follow`, `manual`, or `{ limit: number }`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "follow" => Ok(Redirect::Follow),
                    "manual" => Ok(Redirect::Manual),
                    _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &self)),
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let Some((_, limit)) = map.next_entry::<KeyLimit, usize>()? else {
                    return Err(<A::Error as serde::de::Error>::missing_field("limit"));
                };

                Ok(Redirect::Limit { limit })
            }
        }

        deserializer.deserialize_any(V)
    }
}
