use std::{env};
use log::{error};

pub struct GitlabConfig {
    pub token: String,
    pub api_url:String
}

impl GitlabConfig {
    pub fn load() -> Option<Self> {
        match GitlabConfig::load_from_env() {
            Ok(cfg) => Some(cfg),
            Err(_) => {
                error!("Failed to load gitlab configuration, make sure that GITLAB_TOKEN is defined.");
                return None
            }
        }
    }

    fn load_from_env() -> anyhow::Result<GitlabConfig> {
        let token_ = env::var("GITLAB_TOKEN")?;
        let api_url_ = env::var("GITLAB_API")?;
        Ok(GitlabConfig {
            token: token_,
            api_url: api_url_,
        })
    }

    pub fn pipeline_api(&self, project: String) -> String {
        format!("{}/projects/{}/pipelines/", self.api_url, project)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_api_should_return_valid_url() {
        let cfg = GitlabConfig {
            token: "aaa".to_string(),
            api_url: "http://foo.bar".to_string()
        };
        let url = "http://foo.bar/projects/123456/pipelines/";
        assert_eq!(cfg.pipeline_api("123456".to_string()),url)
    }

    #[test]
    fn load_config_should_load_gitlab_token_and_return_some() {
        let token = "GITLAB_TOKEN";
        env::set_var(token,"123456");
        let api = "GITLAB_API";
        env::set_var(api,"123456");
        assert_eq!(GitlabConfig::load().is_some(),true)
    }

    #[test]
    fn load_config_should_load_gitlab_token_and_return_token() {
        let token = "GITLAB_TOKEN";
        env::set_var(token,"123456");
        let api = "GITLAB_API";
        env::set_var(api,"aaa");
        assert_eq!(GitlabConfig::load().unwrap().token,"123456")
    }

    #[test]
    fn load_config_should_load_gitlab_token_and_return_api() {
        let token = "GITLAB_TOKEN";
        env::set_var(token,"123456");
        let api = "GITLAB_API";
        env::set_var(api,"aaa");
        assert_eq!(GitlabConfig::load().unwrap().api_url,"aaa")
    }

    #[test]
    fn load_config_should_return_none_when_env_var_for_gitlab_cfg_is_not_defined() {
        // if environment variable is not set, the remove_var can panic.
        let key = "GITLAB_TOKEN";
        env::set_var(key,"123456");
        env::remove_var(key);
        assert_eq!(GitlabConfig::load().is_none(),true)
    }

}