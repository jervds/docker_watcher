use crate::LocalImageDetails;
use crate::providers::dockerhub_image_description::DockerHubImageDescription;
use log::{error,debug};


pub struct Dockerhub;

impl Dockerhub {
    pub fn has_newer_version_for(local_image: &LocalImageDetails) -> Option<bool> {
        match Dockerhub::registry_api_url(&local_image.image) {
            None => None,
            Some(registry) => {
                match Dockerhub::get_details_for(&registry) {
                    Ok(dockerhub_image) => dockerhub_image.is_newer_than(local_image),
                    Err(_) => {
                        error!("Error retrieving data from Dockerhub {}", registry);
                        None
                    }
                }
            }
        }

    }

    fn get_details_for(registry: &str) -> anyhow::Result<DockerHubImageDescription> {
        let res = reqwest::blocking::get(registry)?.text()?;
        let dockerhub_image_details = serde_json::from_str::<DockerHubImageDescription>(&*res)?;
        Ok(dockerhub_image_details)
    }

    fn registry_api_url(image: &str) -> Option<String> {
        debug!("Extracting api url from: {}",image);
        let image_and_tag = image.split(":").collect::<Vec<&str>>();
        if image_and_tag.len() != 2 { return None }

        let image_name = image_and_tag[0];
        let tag = image_and_tag[1];
        if image_name.contains("/") {
            Some(format!("https://hub.docker.com/v2/repositories/{}/tags/{}",image_name,tag))
        } else {
            Some(format!("https://hub.docker.com/v2/repositories/library/{}/tags/{}",image_name,tag))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_image_based_on(image_registry: &str) -> LocalImageDetails {
        LocalImageDetails {
            name: "".to_string(),
            image: image_registry.to_string(),
            project_id: "".to_string(),
            branch: "".to_string(),
            last_build: Some("2021-09-09T21:20:21.385571Z".to_string())
        }
    }

    #[test]
    fn has_newer_version_for_should_return_none_when_wrong_registry() {
        let invalid_image = sample_image_based_on("bliblablou:latest");
        assert_eq!(Dockerhub::has_newer_version_for(&invalid_image).is_none(),true)
    }

    #[test]
    fn has_newer_version_for_should_return_none_with_invalid_registry() {
        let invalid_image_registry = sample_image_based_on("bli/bla/blou:latest");
        assert_eq!(Dockerhub::has_newer_version_for(&invalid_image_registry).is_none(), true)
    }

    #[test]
    fn has_newer_version_for_should_return_some_when_correct_registry() {
        let valid_image = sample_image_based_on("postgres:latest");
        assert_eq!(Dockerhub::has_newer_version_for(&valid_image).is_some(),true)
    }

    #[test]
    fn registry_api_should_correctly_format_api_url_for_images_without_directory() {
        let expected_url = "https://hub.docker.com/v2/repositories/library/postgres/tags/latest";
        assert_eq!(Dockerhub::registry_api_url("postgres:latest").unwrap(), expected_url)
    }

    #[test]
    fn registry_api_should_correctly_format_api_url_for_images_with_directory() {
        let expected_url = "https://hub.docker.com/v2/repositories/testcontainers/ryuk/tags/latest";
        assert_eq!(Dockerhub::registry_api_url("testcontainers/ryuk:latest").unwrap(), expected_url)
    }

    #[test]
    fn registry_api_should_return_none_when_image_tag_is_missing() {
        assert_eq!(Dockerhub::registry_api_url("testcontainers/ryuk").is_none(), true)
    }

    #[test]
    fn registry_api_should_return_none_when_image_name_has_more_than_one_folder() {
        assert_eq!(Dockerhub::registry_api_url("test/containers/ryuk").is_none(), true)
    }
}
