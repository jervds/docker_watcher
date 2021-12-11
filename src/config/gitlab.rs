use std::{env};
use log::{error};

pub struct GitlabConfig {
    pub token: String,
}

impl GitlabConfig {
    pub fn load() -> Option<Self> {
        match env::var("GITLAB_TOKEN") {
            Ok(env_token) => Some(GitlabConfig { token: env_token }),
            Err(_) => {
                error!("Failed to load gitlab configuration, make sure that GITLAB_TOKEN is defined.");
                return None
            }
        }
    }

    pub fn pipeline_api(project: String) -> String {
        format!("https://innersource.soprasteria.com/api/v4/projects/{}/pipelines/", project)
    }

}