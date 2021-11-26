use serde::{Deserialize, Serialize};
use std::{env, error, process};


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn get_last_pipeline_run_time(project: String, branch: String) -> Result<String> {
    //TODO handle the pagination
    //curl --head --header "PRIVATE-TOKEN: <your_access_token>" "https://gitlab.example.com/api/v4/projects/9/issues/8/notes?per_page=3&page=2"
    //TODO handle the case in which no date is returned
    let gitlab_config = load_gitlab_config();
    let api_url = format!("https://innersource.soprasteria.com/api/v4/projects/{}/pipelines/", project);
    println!("Querying on: {}", api_url);

    let client = reqwest::blocking::Client::new();

    let pipeline = client.get(api_url).bearer_auth(gitlab_config.token).send()
        .unwrap_or_else(|err| {
            eprintln!("Error when calling: {}", err);
            process::exit(1);
        })
        .text()
        .unwrap_or_else(|err| {
            eprintln!("Error when calling: {}", err);
            process::exit(2);
        })
        .parse_json()
        .pipelines.into_iter()
        //TODO also check pipeline status
        .find(|pipeline| pipeline.r#ref == branch)
        .unwrap_or_else(|| {
            eprintln!("No pipeline found for master!");
            process::exit(3);
        })
        ;
    println!("pipeline last run time: {}",pipeline.created_at);
    Ok(pipeline.created_at.clone())
}

fn load_gitlab_config() -> GitlabConfig {
    let cfg = GitlabConfig {
        token: env::var("GITLAB_TOKEN")
            .unwrap_or_else(|err| {
                eprintln!("No token found. Please define GITLAB_TOKEN environment variable - {}",err);
                process::exit(4);
            })
    };

    cfg
}

trait ParseJson {
    fn parse_json(&self) -> GitlabPipelines;
}

impl ParseJson for String {
    fn parse_json(&self) -> GitlabPipelines {
        let pipelines_from_json: Vec<GitlabPipelineApiDescription> = serde_json::from_str(&*self).unwrap_or_else(|err| {
            eprintln!("Error when parsing response: {} on {}", err, self);
            process::exit(1);
        });
        GitlabPipelines{
            pipelines: pipelines_from_json,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GitlabPipelines {
    pipelines: Vec<GitlabPipelineApiDescription>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitlabPipelineApiDescription {
    id: i32,
    created_at: String,
    status: String,
    r#ref: String,
}

struct GitlabConfig {
    token: String,
}