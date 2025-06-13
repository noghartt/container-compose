use std::collections::HashMap;

use serde::{Deserialize};
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Service {
  pub name: Option<String>,
  pub image: String,
  pub ports: Vec<String>,
  #[serde(deserialize_with = "deserialize_environment")]
  pub environment: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Compose {
  pub version: String,
  pub services: HashMap<String, Service>,
}

pub fn deserialize_yaml(yaml: &str) -> Result<Compose, serde_yaml::Error> {
  match serde_yaml::from_str(yaml) {
    Ok(compose) => Ok(compose),
    Err(e) => Err(e),
  }
}

fn deserialize_environment<'a, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'a>,
{
    struct EnvVisitor;

    impl<'a> Visitor<'a> for EnvVisitor {
        type Value = HashMap<String, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map or a list of key=value strings")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'a>,
        {
            let mut map = HashMap::new();
            while let Some((key, value)) = access.next_entry()? {
                map.insert(key, value);
            }
            Ok(map)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'a>,
        {
            let mut map = HashMap::new();
            while let Some(entry) = seq.next_element::<String>()? {
                let parts: Vec<&str> = entry.splitn(2, '=').collect();
                if parts.len() == 2 {
                    map.insert(parts[0].to_string(), parts[1].to_string());
                } else {
                    return Err(de::Error::custom(format!("Invalid environment variable: {}", entry)));
                }
            }
            Ok(map)
        }
    }

    deserializer.deserialize_any(EnvVisitor)
}
