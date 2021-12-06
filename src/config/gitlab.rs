use std::{env};

pub struct GitlabConfig {
    pub token: String,
}

impl GitlabConfig {
    pub fn load() -> Option<Self> {
        match env::var("GITLAB_TOKEN") {
            Ok(env_token) => Some(GitlabConfig { token: env_token }),
            Err(_) => {
                println!("Failed to load configuration");
                return None
            }
        }
    }

    pub fn pipeline_api(project: String) -> String {
        format!("https://innersource.soprasteria.com/api/v4/projects/{}/pipelines/", project)
    }

}