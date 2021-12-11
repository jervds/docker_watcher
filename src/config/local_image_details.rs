use serde::{Deserialize, Serialize};
use std::fs;
use crate::{Dockerhub, GitlabPipelines};
use log::{error, info, warn};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalImageDetails {
    pub name: String,
    pub image: String,
    pub project_id: String,
    pub branch: String,
    #[serde(default)]
    pub last_build: Option<String>,
}

impl LocalImageDetails {
    pub fn retrieve_last_local_build(self) -> Option<Self> {
        let last_run_time = GitlabPipelines::last_run_for(&self);
        match last_run_time {
            None => None,
            Some(pipeline_runtime) => Some(Self::from((self, pipeline_runtime)))
        }
    }

    pub fn should_be_refreshed(&self) -> bool {
        info!("image {} being checked",self.name);
        match Dockerhub::has_newer_version_for(&self) {
            None => false,
            Some(has_newer_version) => has_newer_version
        }
    }

    pub fn refresh_local_image(self) -> Self {
        warn!(">>>>>>> refresh image {} on project id {}", self.name, self.project_id);
        self
    }

    pub fn load_config(config_file: String) -> Vec<Self> {
        let config = fs::read_to_string(&config_file);
        match config {
            Ok(loaded_config) => Self::parse_config(loaded_config),
            Err(_) => {
                error!("Failed to open configuration file {}",config_file);
                Vec::new()
            }
        }
    }

    fn parse_config(loaded_config: String) -> Vec<LocalImageDetails> {
        let cfg = serde_json::from_str::<Vec<LocalImageDetails>>(&*loaded_config);
        match cfg {
            Ok(loaded) => loaded,
            Err(_) => {
                error!("Failed to parse configuration");
                Vec::new()
            }
        }

    }
}

impl From<(LocalImageDetails, String)> for LocalImageDetails {
    fn from((image,last_build_time): (LocalImageDetails, String)) -> Self {
        Self {
            last_build: Some(last_build_time),
            ..image
        }
    }
}