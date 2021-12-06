use serde::{Deserialize, Serialize};
use std::fs;
use crate::{Dockerhub, GitlabPipelines};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalImageDetails {
    pub name: String,
    pub base_image_registry: String,
    pub project_id: String,
    pub branch: String,
    #[serde(default)]
    pub last_build: Option<String>,
    #[serde(default)]
    pub trigger_pipeline: Option<bool>,
}

impl LocalImageDetails {
    pub fn retrieve_last_local_build(&self) -> Option<Self> {
        let last_run_time = GitlabPipelines::last_run_for(&self);
        match last_run_time {
            None => None,
            Some(pipeline_runtime) => Some(Self::from((self, pipeline_runtime)))
        }
    }

    pub fn check_last_build_time(&self) -> Self {
        let trigger = Dockerhub::is_local_image_up_to_date(&self);
        Self::from((self, trigger))
    }

    pub fn refresh_image(&self) -> Self {
        match self.trigger_pipeline {
            None => println!("++++++ Unable to retrieve information for {}",self.name),
            Some(trigger) => {
                match trigger {
                    true => println!(">>>>>>> refresh image {} on project id {}", self.name, self.project_id),
                    false => println!(">>>>>>> Do not refresh image for {} on project id {}",self.name, self.project_id)
                }
            }
        }
        self.clone()
    }

    pub fn load_config(config_file: String) -> Vec<Self> {
        let config = fs::read_to_string(&config_file);
        match config {
            Ok(loaded_config) => Self::parse_config(loaded_config),
            Err(_) => {
                println!("Failed to open configuration file");
                Vec::new()
            }
        }
    }

    fn parse_config(loaded_config: String) -> Vec<LocalImageDetails> {
        let cfg = serde_json::from_str::<Vec<LocalImageDetails>>(&*loaded_config);
        match cfg {
            Ok(loaded) => loaded,
            Err(_) => {
                println!("Failed to parse configuration");
                Vec::new()
            }
        }

    }
}

impl From<(&LocalImageDetails, String)> for LocalImageDetails {
    fn from((image,last_build_time): (&LocalImageDetails, String)) -> Self {
        Self {
            name: (*image.name).to_string(),
            base_image_registry: (*image.base_image_registry).to_string(),
            project_id: (*image.project_id).to_string(),
            branch: (*image.branch).to_string(),
            last_build: Some(last_build_time),
            trigger_pipeline: None
        }
    }
}

impl From<(&LocalImageDetails, Option<bool>)> for LocalImageDetails {
    fn from((image,_trigger_pipeline): (&LocalImageDetails,  Option<bool>)) -> Self {
        Self {
            name: (*image.name).to_string(),
            base_image_registry: (*image.base_image_registry).to_string(),
            project_id: (*image.project_id).to_string(),
            branch: (*image.branch).to_string(),
            // TODO this is ugly !
            last_build: Some((*image.last_build.as_ref().unwrap()).to_string()),
            trigger_pipeline: _trigger_pipeline
        }
    }
}

//TODO Implement tests on both from methods