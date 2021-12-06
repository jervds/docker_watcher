mod providers;
mod config;
mod gitlab;

use crate::config::local_image_details::{LocalImageDetails};
use crate::gitlab::pipelines::{GitlabPipelines};
use crate::providers::dockerhub::Dockerhub;

fn main() -> anyhow::Result<()> {
    LocalImageDetails::load_config(String::from("config.json"))
        .into_iter()
        .filter_map(|it| LocalImageDetails::retrieve_last_local_build(&it))
        .map(|it | LocalImageDetails::check_last_build_time(&it))
        .map(|it| { LocalImageDetails::refresh_image(&it) })
        .for_each(drop);
    Ok(())
}

