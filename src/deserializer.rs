use std::collections::HashMap;

use serde::Deserialize;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: Option<String>,
    pub image: String,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_environment_variables")]
    pub environment: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_array_key_value")]
    pub volumes: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_command")]
    pub command: Option<Vec<String>>,
}

#[allow(dead_code, unused_variables)]
#[derive(Debug, Deserialize)]
pub struct Compose {
    pub version: String,
    pub services: HashMap<String, Service>,
}

pub fn deserialize_yaml(yaml: &str) -> Result<Compose, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

fn deserialize_environment_variables<'a, D>(
    deserializer: D,
) -> Result<HashMap<String, String>, D::Error>
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
                    return Err(de::Error::custom(format!(
                        "Invalid environment variable: {}",
                        entry
                    )));
                }
            }
            Ok(map)
        }
    }

    deserializer.deserialize_any(EnvVisitor)
}

fn deserialize_array_key_value<'a, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'a>,
{
    struct ArrayKeyValueVisitor;

    impl<'a> Visitor<'a> for ArrayKeyValueVisitor {
        type Value = HashMap<String, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of strings which follows this format: key:value")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'a>,
        {
            let mut map = HashMap::new();
            while let Some(entry) = seq.next_element::<String>()? {
                let parts: Vec<&str> = entry.splitn(2, ':').collect();
                if parts.len() == 2 {
                    map.insert(parts[0].to_string(), parts[1].to_string());
                } else {
                    return Err(de::Error::custom(format!("Invalid volume: {}", entry)));
                }
            }
            Ok(map)
        }
    }

    deserializer.deserialize_seq(ArrayKeyValueVisitor)
}

fn deserialize_command<'a, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'a>,
{
    struct CommandVisitor;

    impl<'a> Visitor<'a> for CommandVisitor {
        type Value = Option<Vec<String>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a list of strings")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let parts: Vec<&str> = v.splitn(2, ' ').collect();
            if parts.len() == 2 {
                Ok(Some(vec![parts[0].to_string(), parts[1].to_string()]))
            } else {
                Ok(Some(vec![v.to_string()]))
            }
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'a>,
        {
            let mut vec = Vec::new();
            while let Some(entry) = seq.next_element::<String>()? {
                vec.push(entry);
            }
            Ok(Some(vec))
        }
    }

    deserializer.deserialize_any(CommandVisitor)
}
