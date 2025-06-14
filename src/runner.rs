use std::{fs, process::Command, path::Path};

use crate::{deserializer, container};

pub fn create_and_run_containers(path: Vec<String>) {
  let compose_file = Path::new(".").join("docker-compose.yaml");
  let Ok(yaml) = fs::read_to_string(path.first().unwrap_or(&compose_file.to_str().unwrap().to_string())) else {
    panic!("No docker-compose.yaml file found");
  };

  let compose = deserializer::deserialize_yaml(&yaml).unwrap();

  for (name, service) in compose.services.iter() {
    let container_name = service.name.clone().unwrap_or(name.clone());

    // call a command in terminal
    let mut output = Command::new("container");
    output
      .arg("run")
      .arg("--name")
      .arg(container_name.clone());

    for (key, value) in service.environment.iter() {
      let env_var = format!("{}={}", key, value);
      println!("-e env {}", env_var);
      output.arg("-e");
      output.arg(env_var);
    }


    let output = output
      .arg("-d")
      .arg(service.image.clone());

    let output = dbg!(output);
    let output = output.output().expect("Failed to execute process");

    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());

    if service.ports.len() > 0 {
      println!("Found ports in service, container does not support mapping port yet. Running socat fallback.)");
      let command = Command::new("container")
        .arg("inspect")
        .arg(container_name.clone())
        .output()
        .expect("Failed to execute process");

      let value = String::from_utf8(command.stdout).unwrap();
      let container = serde_json::from_str::<Vec<container::Container>>(&value).unwrap()[0].clone();

        for port in service.ports.iter() {
          let port = port.split(":").collect::<Vec<&str>>();
          let host_port = port[0].parse::<u16>().unwrap();
          let container_port = port[1].parse::<u16>().unwrap();

          let container_ip = format!("{}:{}", container.configuration.networks[0], container_port);

          let output = Command::new("socat")
            .arg(format!("TCP-LISTEN:{},fork", host_port))
            .arg(format!("TCP:{}", container_ip))
            .spawn()
            .expect("Failed to execute process");

          println!("socat running on pid {}", output.id());
          println!("socat status: {:?}", output);
        }
    }
  }
}
