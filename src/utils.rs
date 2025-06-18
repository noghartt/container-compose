use std::{fs, path::Path};

use crate::deserializer::{Compose, deserialize_yaml};

pub fn deserialize_compose_file(path: Option<String>) -> Result<Compose, serde_yaml::Error> {
    if let Some(path) = path {
        let yaml = fs::read_to_string(path).expect("Failed to read the provided compose file");
        return deserialize_yaml(&yaml);
    };

    let default_candidates = ["docker-compose.yaml", "docker-compose.yml"];

    for candidate in default_candidates.iter() {
        let path = Path::new(".").join(candidate);
        if path.exists() {
            let yaml = fs::read_to_string(path).expect("Failed to read compose file");
            return deserialize_yaml(&yaml);
        }
    }

    panic!("No docker-compose.yaml or docker-compose.yml file found");
}
