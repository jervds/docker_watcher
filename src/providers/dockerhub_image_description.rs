use serde::{Deserialize, Serialize};
use chrono::DateTime;
use crate::LocalImageDetails;
use log::{error};

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerHubImageDescription {
    id: i32,
    last_updated: String,
}

impl DockerHubImageDescription {
    pub fn is_newer_than(&self, image: &LocalImageDetails) -> Option<bool> {
        match self.is_more_recent_than(&image) {
            Ok(newer) => Some(newer),
            Err(_) => {
                error!("Error when comparing image dates: {} vs {}",&self.last_updated, image.last_build.as_ref().unwrap());
                None
            }
        }
    }

    fn is_more_recent_than(&self,image: &LocalImageDetails) -> anyhow::Result<bool>  {
        let local_image_last_build = DateTime::parse_from_rfc3339(image.last_build.as_ref().unwrap())?;
        let last_pushed_date = DateTime::parse_from_rfc3339(&self.last_updated)?;
        if last_pushed_date.ge(&local_image_last_build) {
            return Ok(true)
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dockerhub(push_date: &str) -> DockerHubImageDescription {
        DockerHubImageDescription {
            id: 0,
            last_updated: push_date.to_string()
        }
    }

    fn local_image_built_at(build_date: &str) -> LocalImageDetails {
        LocalImageDetails {
            name: String::from("sample"),
            image: "".to_string(),
            project_id: String::from("sample"),
            branch: String::from("sample"),
            last_build: Some(build_date.to_string())
        }
    }

    #[test]
    fn is_newer_than_should_return_some_when_dockerhub_image_is_newer() {
        let local_image = local_image_built_at("2021-09-09T21:20:21.385571Z");
        let dockerhub_image = sample_dockerhub("2022-09-09T21:20:21.385571Z");
        assert_eq!(dockerhub_image.is_newer_than(&local_image).is_some(), true);
    }

    #[test]
    fn is_newer_than_should_return_true_when_dockerhub_image_is_newer() {
        let local_image = local_image_built_at("2021-09-09T21:20:21.385571Z");
        let dockerhub_image = sample_dockerhub("2022-09-09T21:20:21.385571Z");
        assert_eq!(dockerhub_image.is_newer_than(&local_image).unwrap(), true);
    }

    #[test]
    fn is_newer_than_should_return_some_when_dockerhub_image_is_older() {
        let local_image = local_image_built_at("2022-09-09T21:20:21.385571Z");
        let dockerhub_image = sample_dockerhub("2021-09-09T21:20:21.385571Z");
        assert_eq!(dockerhub_image.is_newer_than(&local_image).is_some(), true);
    }

    #[test]
    fn is_newer_than_should_return_false_when_dockerhub_image_is_older() {
        let local_image = local_image_built_at("2022-09-09T21:20:21.385571Z");
        let dockerhub_image = sample_dockerhub("2021-09-09T21:20:21.385571Z");
        assert_eq!(dockerhub_image.is_newer_than(&local_image).unwrap(), false);
    }

    #[test]
    fn is_newer_than_should_return_none_when_local_image_build_date_is_in_incorrect_format() {
        let local_image = local_image_built_at("aaa");
        let dockerhub_image = sample_dockerhub("2021-09-09T21:20:21.385571Z");
        assert_eq!(dockerhub_image.is_newer_than(&local_image).is_none(), true);
    }

    #[test]
    fn is_newer_than_should_return_none_when_dockerhub_image_push_date_is_in_incorrect_format() {
        let local_image = local_image_built_at("2022-09-09T21:20:21.385571Z");
        let dockerhub_image = sample_dockerhub("aaa");
        assert_eq!(dockerhub_image.is_newer_than(&local_image).is_none(), true);
    }

}