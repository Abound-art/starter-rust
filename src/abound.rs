use image::ImageError;
use image::RgbImage;
use serde::de::DeserializeOwned;
use std::env;
use std::fs::File;
use std::io;
use thiserror::Error;

pub const CONFIG_PATH_ENV_VAR: &str = "ABOUND_CONFIG_PATH";
pub const OUTPUT_PATH_ENV_VAR: &str = "ABOUND_OUTPUT_PATH";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse JSON")]
    Serde(#[from] serde_json::Error),
    #[error("Failed to load config from env")]
    Env(#[from] env::VarError),
    #[error("Failed to read config file")]
    File(#[from] io::Error),
    #[error("Failed to write image")]
    Image(#[from] ImageError),
}

pub fn load_config<T: DeserializeOwned>() -> Result<T, Error> {
    let path = env::var(CONFIG_PATH_ENV_VAR)?;
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}

pub fn save_png(img: RgbImage) -> Result<(), Error> {
    let path = env::var(OUTPUT_PATH_ENV_VAR)?;
    img.save(path)?;
    Ok(())
}
