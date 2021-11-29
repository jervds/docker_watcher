mod providers;
mod config;
mod gitlab;

use std::{process};

use crate::config::docker_images::{ImageToCheckInternal, load_config};
use crate::providers::dockerhub::check_image_validity;
use crate::gitlab::pipelines::get_last_pipeline_run_time;

fn main() {
    load_config(String::from("config.json"))
        .unwrap_or_else(|err| {
            eprintln!("Failed to load configuration file: {}",err);
            process::exit(4);
        })
        .into_iter()
        .map(|image|{
            // TODO it should be possible to improve this with a kind of copy() like in kotlin
            ImageToCheckInternal {
                name: image.name,
                registry: image.registry,
                // TODO the clone() could be avoided here?
                last_build: get_last_pipeline_run_time(image.project_id.clone(), image.branch.clone()).unwrap(),
                project_id: image.project_id,
                trigger_pipeline: false
            }
        })
        .map(|mut image| {
            image.trigger_pipeline = check_image_validity(&image).unwrap_or_else(|err| {
                eprintln!("Error when checking valididy for {} : {}", image.name,err);
                process::exit(5);
            });
            image
        })
        .map(|image| {
            if image.trigger_pipeline {
                println!(">>>>>>> refresh image {} on project id {}", image.name, image.project_id);
            }
            else {
                println!(">>>>>>> Do not refresh image for {} on project id {}",image.name, image.project_id);
            }

        })
        // TODO remove the warning here! Collected values are not used
        .collect::<()>();


}

