use serde::{Deserialize, Serialize};
use futures::executor::block_on;
use chrono::{DateTime, NaiveDate, NaiveTime};
use crate::web::web_requests::make_call;
use crate::config::docker_images::ImageToCheck;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub(crate) fn check_image_validity(image: &ImageToCheck) -> Result<bool> {
    let res = block_on(make_call(&image.registry));
    match res {
        Ok(body) => check_last_push_date(body,image),
        Err(_) => panic!("Couldn't reach the repository")
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DockerImageGenericDescription {
    id: i32,
    pub(crate) images: Vec<ImageDetail>
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageDetail {
    digest: String,
    pub(crate) last_pushed: String,
    architecture: String,
    os: String,
}

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn check_last_push_date(body: String, image: &ImageToCheck) -> Result<bool> {
    let docker_image_generic_description: DockerImageGenericDescription = serde_json::from_str(&*body)?;
    let image_last_build = NaiveDate::parse_from_str(&image.last_build,"%Y-%m-%d")?
        .and_time(NaiveTime::from_hms(0,0,0));

    for image_detail in docker_image_generic_description.images.iter() {
        let last_pushed_date = DateTime::parse_from_rfc3339(&image_detail.last_pushed)?.naive_utc();
        if last_pushed_date.ge(&image_last_build) {
            return Ok(true)
        }
    }
    Ok(false)
}