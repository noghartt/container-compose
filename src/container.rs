use std::{process::Command, collections::HashMap};
use serde::Deserialize;

#[allow(dead_code, unused_variables)]

pub fn get_containers_list() -> Result<Vec<Container>, ()> {
    let mut command = Command::new("container");
    command
        .arg("ls")
        .arg("--format")
        .arg("json");

    let Ok(output) = command.output() else {
        eprintln!("Failed to get containers list");
        return Err(());
    };

    let Ok(containers) = serde_json::from_slice(&output.stdout) else {
        eprintln!("Failed to parse containers list");
        return Err(());
    };

    Ok(containers)
}

pub fn stop_container(container_ids: Vec<String>) -> Result<(), ()> {
    let mut command = Command::new("container");
    command
        .arg("stop");
    
    for id in container_ids {
        command.arg(id);
    }

    let Ok(_) = command.output() else {
        eprintln!("Failed to stop containers");
        return Err(());
    };

    Ok(())
}

pub fn remove_container(container_ids: Vec<String>) -> Result<(), ()> {
    let mut command = Command::new("container");
    command
        .arg("rm");
    
    for id in container_ids {
        command.arg(id);
    }

    let Ok(_) = command.output() else {
        eprintln!("Failed to remove containers");
        return Err(());
    };

    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
pub struct Container {
    pub networks: Vec<Network>,
    pub status: String,
    pub configuration: Configuration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Network {
    pub address: String,
    pub gateway: String,
    pub network: String,
    pub hostname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub resources: Resources,
    pub labels: HashMap<String, String>,
    pub hostname: String,
    pub sysctls: HashMap<String, String>,
    pub networks: Vec<String>,
    // pub initProcess: InitProcess,
    pub id: String,
    pub rosetta: bool,
    // pub runtimeHandler: String,
    pub platform: Platform,
    pub mounts: Vec<Mount>,
    pub image: Image,
    pub dns: Dns,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Resources {
    pub cpus: u32,
    // pub memoryInBytes: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InitProcess {
    pub environment: Vec<String>,
    pub arguments: Vec<String>,
    pub executable: String,
    // pub workingDirectory: String,
    pub terminal: bool,
    pub user: User,
    // pub supplementalGroups: Vec<u32>,
    pub rlimits: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: Id,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Id {
    pub uid: u32,
    pub gid: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Platform {
    pub os: String,
    pub architecture: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mount {
    // Fill in fields as needed
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub reference: String,
    pub descriptor: Descriptor,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Descriptor {
    pub size: u64,
    pub digest: String,
    // pub annotations: HashMap<String, String>,
    // pub media_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dns {
    pub nameservers: Vec<String>,
    // pub search_domains: Vec<String>,
    pub options: Vec<String>,
}
