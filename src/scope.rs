#[derive(Debug, serde::Deserialize, Default)]
pub struct Scope {
    #[serde(deserialize_with = "deserialize_patterns")]
    #[serde(default)]
    pub allowlist: Vec<glob::Pattern>,
}

impl Scope {
    pub fn is_allowed(&self, url: &reqwest::Url) -> bool {
        self.allowlist.iter().any(|pat| pat.matches(url.as_str()))
    }
}

fn deserialize_patterns<'de, D>(deserializer: D) -> Result<Vec<glob::Pattern>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Vec<glob::Pattern>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("cookie SameSite")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut buf = match seq.size_hint() {
                Some(len) => Vec::with_capacity(len),
                None => Vec::new(),
            };

            while let Some(pattern) = seq.next_element::<String>()? {
                let pattern =
                    glob::Pattern::new(&pattern).map_err(<A::Error as serde::de::Error>::custom)?;

                buf.push(pattern);
            }

            Ok(buf)
        }
    }

    deserializer.deserialize_seq(Visitor)
}
