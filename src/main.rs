mod providers;
mod config;
mod gitlab;

use crate::config::local_image_details::{LocalImageDetails};
use crate::gitlab::pipelines::{GitlabPipelines};
use crate::providers::dockerhub::Dockerhub;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    LocalImageDetails::load_config(String::from("config.json"))
        .into_iter()
        .filter_map(LocalImageDetails::retrieve_last_local_build)
        .filter(LocalImageDetails::should_be_refreshed)
        .map(LocalImageDetails::refresh_local_image)
        .for_each(drop);
    Ok(())
}

