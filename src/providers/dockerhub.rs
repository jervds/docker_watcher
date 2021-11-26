use serde::{Deserialize, Serialize};
use chrono::{DateTime};
use std::fmt;
use std::{error, process};
use crate::ImageToCheckInternal;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub(crate) fn check_image_validity(image: &ImageToCheckInternal) -> Result<bool> {
    let res = reqwest::blocking::get(&image.registry)
        .unwrap_or_else(|err|{
            eprintln!("Error when calling: {}", err);
            process::exit(1);
        })
        .text()
        .unwrap_or_else(|err|{
            eprintln!("Error when calling: {}", err);
            process::exit(2);
        });
    check_last_push_date(res,image)

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
        //TODO refactor this, it's a copy code from the official doc :)
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn check_last_push_date(body: String, image: &ImageToCheckInternal) -> Result<bool> {
    let docker_image_generic_description: DockerImageGenericDescription = serde_json::from_str(&*body)?;
    let base_image_last_build = DateTime::parse_from_rfc3339(&image.last_build)?;

    for image_detail in docker_image_generic_description.images.iter() {
        let last_pushed_date = DateTime::parse_from_rfc3339(&image_detail.last_pushed)?;
        if last_pushed_date.ge(&base_image_last_build) {
            return Ok(true)
        }
    }
    Ok(false)
}