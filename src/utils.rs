use std::{fs, path::Path};

use crate::deserializer::{deserialize_yaml, Compose};

pub fn deserialize_compose_file(path: Option<String>) -> Result<Compose, serde_yaml::Error> {
  let compose_file = Path::new(".").join("docker-compose.yaml");
  let Ok(yaml) = fs::read_to_string(path.unwrap_or(compose_file.to_str().unwrap().to_string())) else {
    panic!("No docker-compose.yaml file found");
  };

  deserialize_yaml(&yaml)
}
