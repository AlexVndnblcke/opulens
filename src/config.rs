use std::{fs, io::Error, path::PathBuf};

use clap::Parser;
use log::{debug, info};
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(name = "opulens")]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", default_value = "config.toml")]
    config: PathBuf,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

impl Cli {
    pub fn read_config(&self) -> Result<Config, ConfigError> {
        let _convert_to_err = |e: Error| -> ConfigError {
            ConfigError::UnreadableFile(format!("{} (failed to read: {:?}", e, self.config))
        };

        let canonicalized_file = self.config.canonicalize().map_err(_convert_to_err)?;
        info!("reading config from file {:?}", canonicalized_file);
        let toml_config = fs::read_to_string(&canonicalized_file).map_err(_convert_to_err)?;

        debug!("Raw TOML input: {:?}", toml_config);
        // Explicit assignment necessary for `toml::from_str` considers config as invalid otherwise.
        let config: Config = toml::from_str(toml_config.as_str())
            .map_err(|e| ConfigError::InvalidConfig(e.message().to_string()))?;
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub enum ConfigError {
    UnreadableFile(String),
    InvalidConfig(String), //(toml::de::Error),
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mongodb: MongoConfig,
    pub log_config: PathBuf,
}

type Uri = String;

#[derive(Deserialize, Debug)]
pub struct MongoConfig {
    pub connection_uri: Uri,
}

#[cfg(test)] // Ensures this module is only included in test builds
mod tests {
    use std::path::PathBuf;

    use crate::config::Config;

    #[test]
    fn test_config() {
        let mock_config = r#"
log_config = "log.toml"

[mongodb]
connection_uri = "mongodb://user:pass@mock_mongo/mock_db"

"#;

        println!("{}", mock_config);

        let config: Config = toml::from_str(mock_config).unwrap();

        assert_eq!(
            config.mongodb.connection_uri,
            String::from("mongodb://user:pass@mock_mongo/mock_db")
        );
        assert_eq!(config.log_config, PathBuf::from("log.toml"));
    }
}
