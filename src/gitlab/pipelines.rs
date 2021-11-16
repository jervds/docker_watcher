use futures::executor::block_on;
use crate::web::web_requests::make_call;
use serde::{Deserialize, Serialize};
use std::{error, process};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn get_last_pipeline_run_time(project: String, branch: String) -> Result<String> {
    let api_url = format!("https://innersource.soprasteria.com/api/v4/{}/pipelines/ref/{}",project,branch);
    let pipelines = block_on(make_call(&api_url))
        .unwrap_or_else(|err|{
            eprintln!("Error when calling: {}", err);
            process::exit(1);
        })
        .parse_json();
    // let pipelines: GitlabPipelines = serde_json::from_str(&*res)?;
    Ok(pipelines.pipelines[0].created_at.clone())
}

trait ParseJson {
    fn parse_json(&self) -> GitlabPipelines;
}

impl ParseJson for String {
    fn parse_json(&self) -> GitlabPipelines {
        serde_json::from_str(&*self ).unwrap_or_else(|err|{
            eprintln!("Error when parsing response: {} on {}", err, self);
            process::exit(1);
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GitlabPipelines {
    pipelines: Vec<GitlabPipelineApiDescription>
}

#[derive(Serialize, Deserialize, Debug)]
struct GitlabPipelineApiDescription {
    id: i32,
    created_at: String
}