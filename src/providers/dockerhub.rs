use crate::LocalImageDetails;
use crate::providers::dockerhub_image_description::DockerHubImageDescription;


pub struct Dockerhub;

impl Dockerhub {
    pub fn is_local_image_up_to_date(local_image: &LocalImageDetails) -> Option<bool> {
        let dockerhub_details = Dockerhub::get_details_for(local_image.base_image_registry.clone());
        match dockerhub_details {
            Ok(details) => details.evaluate_if_newer_than(local_image),
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

