#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CookieProps {
    pub value: String,
    pub path: String,
    pub httponly: Option<bool>,
    pub secure: Option<bool>,
    #[serde(with = "duration_serde")]
    pub max_age: Option<cookie::time::Duration>,
    #[serde(with = "offset_datetime_serde")]
    pub expires: Option<cookie::time::OffsetDateTime>,
    #[serde(with = "same_site_serde")]
    pub same_site: Option<cookie::SameSite>,
}

mod same_site_serde {
    type Me = Option<cookie::SameSite>;

    pub fn serialize<S>(me: &Me, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Some(me) = me else {
            return serializer.serialize_none();
        };

        serializer.serialize_str(match me {
            cookie::SameSite::Strict => "Strict",
            cookie::SameSite::Lax => "Lax",
            cookie::SameSite::None => "None",
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Me, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Me;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("cookie SameSite")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                const SAMESITE_LAX: &str = "Lax";
                const SAMESITE_STRICT: &str = "Strict";
                const SAMESITE_NONE: &str = "None";

                Ok(Some(match v {
                    SAMESITE_LAX => cookie::SameSite::Lax,
                    SAMESITE_STRICT => cookie::SameSite::Strict,
                    SAMESITE_NONE => cookie::SameSite::None,
                    v @ _ => {
                        return Err(E::unknown_variant(
                            v,
                            &[SAMESITE_LAX, SAMESITE_STRICT, SAMESITE_NONE],
                        ))
                    }
                }))
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

mod duration_serde {
    type Me = Option<cookie::time::Duration>;

    pub fn serialize<S>(me: &Me, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Some(me) = me else {
            return serializer.serialize_none();
        };

        serializer.serialize_f64(me.as_seconds_f64())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Me, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Me;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Duration")
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Some(cookie::time::Duration::seconds_f64(v)))
            }
        }

        deserializer.deserialize_f64(Visitor)
    }
}

mod offset_datetime_serde {
    type Me = Option<cookie::time::OffsetDateTime>;

    const DATE_FMT: &[time::format_description::FormatItem<'_>] =cookie::time::macros::format_description!(
        "[weekday repr:short case_sensitive:true], [day] [month repr:short case_sensitive:true] [year repr:full] [hour repr:24]:[minute]:[second] GMT"
    );

    pub fn serialize<S>(me: &Me, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Some(me) = me else {
            return serializer.serialize_none();
        };

        let date_str = me
            .format(DATE_FMT)
            .map_err(<S::Error as serde::ser::Error>::custom)?;
        serializer.serialize_str(&date_str)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Me, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Me;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Duration")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let date = cookie::time::OffsetDateTime::parse(v, DATE_FMT)
                    .map_err(<E as serde::de::Error>::custom)?;
                Ok(Some(date))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_f64(Visitor)
    }
}
