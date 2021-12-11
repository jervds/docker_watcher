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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_api_should_return_valid_url() {
        let url = "https://innersource.soprasteria.com/api/v4/projects/123456/pipelines/";
        assert_eq!(GitlabConfig::pipeline_api("123456".to_string()),url)
    }

    #[test]
    fn load_config_should_load_gitlab_token_and_return_some() {
        let key = "GITLAB_TOKEN";
        env::set_var(key,"123456");
        assert_eq!(GitlabConfig::load().is_some(),true)
    }

    #[test]
    fn load_config_should_load_gitlab_token_and_return_token() {
        let key = "GITLAB_TOKEN";
        env::set_var(key,"123456");
        assert_eq!(GitlabConfig::load().unwrap().token,"123456")
    }

    #[test]
    fn load_config_should_return_none_when_env_var_for_gitlab_token_is_not_defined() {
        // if environment variable is not set, the remove_var can panic.
        let key = "GITLAB_TOKEN";
        env::set_var(key,"123456");
        env::remove_var(key);
        assert_eq!(GitlabConfig::load().is_none(),true)
    }

}