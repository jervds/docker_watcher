use serde::{Deserialize, Serialize};
use std::{error, process};


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn get_last_pipeline_run_time(project: String, branch: String) -> Result<String> {
    //TODO handle the pagination
    //curl --head --header "PRIVATE-TOKEN: <your_access_token>" "https://gitlab.example.com/api/v4/projects/9/issues/8/notes?per_page=3&page=2"
    //TODO handle the case in which no date is returned
    let api_url = format!("https://innersource.soprasteria.com/api/v4/{}/pipelines/",project);
    println!("Querying on: {}",api_url);

    let pipeline = reqwest::blocking::get(api_url)
        .unwrap_or_else(|err|{
            eprintln!("Error when calling: {}", err);
            process::exit(1);
        })
        .text()
        .unwrap_or_else(|err|{
            eprintln!("Error when calling: {}", err);
            process::exit(2);
        })
        .parse_json()
        .pipelines.into_iter()
        .find(|pipeline| pipeline.r#ref == branch)
        .unwrap_or_else(||{
            eprintln!("No pipeline found for master!");
            process::exit(3);
        })
        ;
    // let pipelines: GitlabPipelines = serde_json::from_str(&*res)?;
    Ok(pipeline.created_at.clone())
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
    created_at: String,
    r#ref: String
}