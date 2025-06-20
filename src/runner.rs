use std::{collections::HashMap, path::Path, process::Command};

use crate::{container, deserializer, utils::deserialize_compose_file};

pub fn run_services(path: Option<String>) {
    let compose = deserialize_compose_file(path).unwrap();
    for (name, service) in compose.services.iter() {
        let container_name = service.name.clone().unwrap_or(name.clone());
        let service_container = ServiceContainer::new(container_name, service);
        match service_container.run() {
          Ok(_) => println!("{}", name),
          Err(e) => println!("Error while running {}: {:?}", name, e),
        };
    }
}

pub fn stop_and_remove_services(path: Option<String>) {
    let containers = container::get_containers_list().unwrap();
    let container_ids = containers
        .iter()
        .map(|c| c.configuration.id.clone())
        .collect::<Vec<String>>();

    container::stop_container(container_ids.clone()).unwrap();
    container::remove_container(container_ids.clone()).unwrap();

    let compose = deserialize_compose_file(path).unwrap();

    let ports = compose
        .services
        .iter()
        .flat_map(|(_, service)| service.ports.clone())
        .collect::<Vec<String>>();

    for port in ports {
        let port = port.split(":").collect::<Vec<&str>>();
        let host_port = port[0].parse::<u16>().unwrap();
        let output = Command::new("lsof")
            .arg("-ti")
            .arg(format!(":{host_port}"))
            .output()
            .expect("Failed to execute process");

        if output.status.success() {
            let pid = String::from_utf8(output.stdout).unwrap();
            Command::new("kill")
                .arg(pid.trim())
                .output()
                .expect("Failed to execute process");
        }
    }
}

struct ServiceContainer {
    name: String,
    ports: Vec<String>,
    environment: HashMap<String, String>,
    image: String,
    volumes: HashMap<String, String>,
    command: Option<Vec<String>>,
    cpu_count: Option<u32>,
    mem_limit: Option<String>,
}

impl ServiceContainer {
    pub fn new(name: String, service: &deserializer::Service) -> Self {
        Self {
            name,
            ports: service.ports.clone(),
            environment: service.environment.clone(),
            image: service.image.clone(),
            volumes: service.volumes.clone(),
            command: service.command.clone(),
            cpu_count: service.cpu_count,
            mem_limit: service.mem_limit.clone(),
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let mut output = Command::new("container");
        output.arg("run").arg("--name").arg(self.name.clone());

        for (key, value) in self.environment.iter() {
            let env_var = format!("{}={}", key, value);
            output.arg("-e");
            output.arg(env_var);
        }

        for (key, value) in self.volumes.iter() {
            output.arg("--mount");

            if !Path::new(key).exists() {
                std::fs::create_dir_all(key).unwrap();
            }

            let abs_source =
                std::fs::canonicalize(key).expect("failed to canonicalize mount source path");
            let abs_source_str = abs_source.to_str().expect("non-UTF8 path");

            output.arg(format!(
                "type=bind,source={},target={}",
                abs_source_str, value
            ));
        }

        // Add CPU and memory limits if specified
        if let Some(cpu_count) = self.cpu_count {
            output.arg("--cpus").arg(cpu_count.to_string());
        }
        if let Some(mem_limit) = &self.mem_limit {
            output.arg("--memory").arg(mem_limit);
        }

        let output = output.arg("-d").arg(self.image.clone());

        if let Some(command) = &self.command {
            if command.len() == 1 && command[0].is_empty() {
                output.arg("echo").arg("No command provided");
            } else {
                for arg in command {
                    output.arg(arg);
                }
            }
        }

        match output.output() {
            Ok(output) => {
                if !output.status.success() {
                    return Err(format!("Failed to run container: {:?}", String::from_utf8(output.stderr).unwrap()));
                }
            }
            Err(e) => {
                return Err(e.kind().to_string());
            }
        }

        self.expose_service_ports();

        Ok(())
    }

    fn expose_service_ports(&self) {
        if !self.ports.is_empty() {
            println!(
                "Found ports in service, container does not support mapping port yet. Running socat fallback."
            );
            let command = Command::new("container")
                .arg("inspect")
                .arg(self.name.clone())
                .output()
                .expect("Failed to execute process");

            let value = String::from_utf8(command.stdout).unwrap();
            let container =
                serde_json::from_str::<Vec<container::Container>>(&value).unwrap()[0].clone();

            for port in self.ports.iter() {
                let port = port.split(":").collect::<Vec<&str>>();
                let host_port = port[0].parse::<u16>().unwrap();
                let container_port = port[1].parse::<u16>().unwrap();

                let container_ip =
                    format!("{}:{}", container.configuration.networks[0], container_port);

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
