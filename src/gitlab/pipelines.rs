use serde::{Deserialize, Serialize};
use crate::config::gitlab::GitlabConfig;
use crate::LocalImageDetails;

#[derive(Serialize, Deserialize, Debug)]
pub struct GitlabPipelines {
    pipelines: Vec<GitlabPipelineApiDescription>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitlabPipelineApiDescription {
    id: i32,
    created_at: String,
    status: String,
    r#ref: String,
}

impl GitlabPipelines {
    pub fn last_run_for(local_image: &LocalImageDetails) -> Option<String> {
        // TODO handle the pagination
        // curl --head --header "PRIVATE-TOKEN: <your_access_token>" "https://gitlab.example.com/api/v4/projects/9/issues/8/notes?per_page=3&page=2"
        // TODO handle the case in which no date is returned
        match GitlabConfig::load() {
            None => None,
            Some(gitlab_config) => {
                match GitlabPipelines::find_pipeline(gitlab_config,&local_image) {
                    Ok(a_pipeline) => Some(a_pipeline.created_at.clone()),
                    Err(_) => None
                }
            }
        }
    }

    fn find_pipeline(cfg : GitlabConfig, local_image: &LocalImageDetails) -> anyhow::Result<GitlabPipelineApiDescription> {
        let client = reqwest::blocking::Client::new();
        let pipeline = client
            .get(GitlabConfig::pipeline_api(local_image.project_id.clone()))
            .bearer_auth(cfg.token)
            .send()?
            .text()?
            .extract_pipelines_from_json()?
            .pipelines.into_iter()
            .find(|pipeline| pipeline.r#ref == local_image.branch && pipeline.status == "success" );

        match pipeline {
            Some(a_pipeline) => Ok(a_pipeline),
            None => panic!("No pipeline found !"), //TODO should not panic ?
        }
    }
}


trait ParseJson {
    fn extract_pipelines_from_json(&self) -> anyhow::Result<GitlabPipelines>;
}

impl ParseJson for String {
    fn extract_pipelines_from_json(&self) -> anyhow::Result<GitlabPipelines> {
        let pipelines_from_json: Vec<GitlabPipelineApiDescription> = serde_json::from_str(&*self)?;
        Ok(GitlabPipelines{ pipelines: pipelines_from_json })
    }
}



