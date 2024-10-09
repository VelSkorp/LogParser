use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub log_file: String,
    pub log_levels: Vec<String>,
}

pub fn load_config() -> LogConfig {
    let builder = Config::builder();
    let settings = builder.add_source(File::with_name("config"))
        .build()
        .expect("Failed to load the configuration");

        settings.try_deserialize::<LogConfig>().expect("Failed to convert the configuration")
}