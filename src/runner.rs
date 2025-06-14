use std::{collections::HashMap, fs, path::Path, process::Command};

use crate::{container, deserializer};

pub fn run_services(path: Vec<String>) {
  let compose_file = Path::new(".").join("docker-compose.yaml");
  let Ok(yaml) = fs::read_to_string(path.first().unwrap_or(&compose_file.to_str().unwrap().to_string())) else {
    panic!("No docker-compose.yaml file found");
  };

  let compose = deserializer::deserialize_yaml(&yaml).unwrap();

  for (name, service) in compose.services.iter() {
    let container_name = service.name.clone().unwrap_or(name.clone());
    let service_container = ServiceContainer::new(container_name, service);
    service_container.run().unwrap();
  }
}

struct ServiceContainer {
  pub name: String,
  pub ports: Vec<String>,
  pub environment: HashMap<String, String>,
  pub image: String,
}

impl ServiceContainer {
  pub fn new(name: String, service: &deserializer::Service) -> Self {
    Self {
      name,
      ports: service.ports.clone(),
      environment: service.environment.clone(),
      image: service.image.clone(),
    }
  }

  pub fn run(&self) -> Result<(), ()> {
    let mut output = Command::new("container");
    output
      .arg("run")
      .arg("--name")
      .arg(self.name.clone());
  
    for (key, value) in self.environment.iter() {
      let env_var = format!("{}={}", key, value);
      output.arg("-e");
      output.arg(env_var);
    }
  
    let output = output
      .arg("-d")
      .arg(self.image.clone());
  
    match output.output() {
      Ok(output) => {
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("{}", String::from_utf8(output.stderr).unwrap());
      }
      Err(e) => {
        eprintln!("Failed to run container: {}", e);
      }
    }

    self.expose_service_ports();

    Ok(())
  }

  fn expose_service_ports(&self) {
    if self.ports.len() > 0 {
      println!("Found ports in service, container does not support mapping port yet. Running socat fallback.");
      let command = Command::new("container")
        .arg("inspect")
        .arg(self.name.clone())
        .output()
        .expect("Failed to execute process");
  
      let value = String::from_utf8(command.stdout).unwrap();
      let container = serde_json::from_str::<Vec<container::Container>>(&value).unwrap()[0].clone();
  
      for port in self.ports.iter() {
        let port = port.split(":").collect::<Vec<&str>>();
        let host_port = port[0].parse::<u16>().unwrap();
        let container_port = port[1].parse::<u16>().unwrap();

        let container_ip = format!("{}:{}", container.configuration.networks[0], container_port);

        let output = Command::new("socat")
          .arg(format!("TCP-LISTEN:{},fork", host_port))
          .arg(format!("TCP:{}", container_ip))
          .spawn();

        match output {
          Ok(output) => {
            println!("socat running on pid {}", output.id());
          }
          Err(e) => {
            eprintln!("Failed to run socat: {}", e);
          }
        }
      }
    }
  }
  
}
