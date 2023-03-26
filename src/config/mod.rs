use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;


#[derive(Debug,Deserialize)]
#[allow(unused)]
pub struct Hanko{
    pub url: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub hanko : Hanko
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("./config/{}", run_mode)).required(false))
            .add_source(Environment::default())
            .build()?;

        s.try_deserialize()
    }
}