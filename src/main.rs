mod providers;
mod config;
mod web;
mod gitlab;

use crate::config::docker_images::load_config;
use crate::providers::dockerhub::check_image_validity;
use crate::gitlab::pipelines::get_last_pipeline_run_time;


#[tokio::main]
async fn main() {

    let last_run_time = get_last_pipeline_run_time(String::from("37671"),String::from("master"));
    match last_run_time {
        Ok(last) => println!("{}",last),
        Err(_) => panic!("Failed to load last runtime !")
    }

    let config = load_config(String::from("config.json"));
    match config {
        Ok(docker_images_to_scan) => {
            for image_to_check in docker_images_to_scan.iter() {
                let res = check_image_validity(image_to_check);
                println!("The image {} should be updated: {}",image_to_check.name,res.unwrap())
            }
        }
        Err(_) => panic!("Failed to load the configuration !")
    }
}

