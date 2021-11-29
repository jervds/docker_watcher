use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ImageToCheck {
    pub(crate) name: String,
    pub(crate) registry: String,
    pub(crate) project_id: String,
    pub(crate) branch: String,
}

pub(crate) struct ImageToCheckInternal {
    pub(crate) name: String,
    pub(crate) registry: String,
    pub(crate) last_build: String,
    pub(crate) project_id: String,
    pub(crate) trigger_pipeline: bool,
}

pub(crate) fn load_config(config_file: String) -> Result<Vec<ImageToCheck>,Error>{
    let config = fs::read_to_string(&config_file)?;
    let docker_images_to_scan: Vec<ImageToCheck> = serde_json::from_str(&*config)?;
    Ok(docker_images_to_scan)
}