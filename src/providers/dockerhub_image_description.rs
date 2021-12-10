use serde::{Deserialize, Serialize};
use chrono::DateTime;
use crate::LocalImageDetails;

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
                println!("Error when comparing image dates");
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

// TODO implement test on is_newer_than