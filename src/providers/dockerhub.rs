use crate::LocalImageDetails;
use crate::providers::dockerhub_image_description::DockerHubImageDescription;


pub struct Dockerhub;

impl Dockerhub {
    pub fn has_newer_version_for(local_image: &LocalImageDetails) -> Option<bool> {
        let maybe_dockerhub_image = Dockerhub::get_details_for(local_image.base_image_registry.clone());
        match maybe_dockerhub_image {
            Ok(dockerhub_image) => dockerhub_image.is_newer_than(local_image),
            Err(_) => {
                println!("Error retrieving data from Dockerhub");
                None
            }
        }
    }

    fn get_details_for(registry: String) -> anyhow::Result<DockerHubImageDescription> {
        let res = reqwest::blocking::get(registry)?.text()?;
        let dockerhub_image_details = serde_json::from_str::<DockerHubImageDescription>(&*res)?;
        Ok(dockerhub_image_details)
    }
}

