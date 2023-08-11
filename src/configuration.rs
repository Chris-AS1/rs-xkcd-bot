use crate::errors::Error;
use anyhow::Context;
use config::{Config, File, FileFormat};
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub bot: BotSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BotSettings {
    pub token: String,
    pub xkcd_url: String,
    pub user_agent: String,
    pub test: String,
}

impl Settings {
    pub fn new() -> Result<Settings, Error> {
        let environment: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| Environment::Production.as_str().into())
            .try_into()
            .expect("failed to load APP_ENV");

        let mut builder = Config::builder();
        builder = builder.add_source(File::new("config/base", FileFormat::Yaml));
        builder = builder.add_source(File::new(
            format!("config/{}", environment.as_str()).as_str(),
            FileFormat::Yaml,
        ));

        // allows to override config variables from env. eg. APP__TOKEN
        builder = builder.add_source(config::Environment::default().prefix("APP").separator("__"));

        match builder
            .build()
            .context("failed to build configs")?
            .try_deserialize()
        {
            Ok(settings) => Ok(settings),
            Err(e) => Err(Error::ConfigError(e)),
        }
    }
}

impl DatabaseSettings {
    pub fn get_url(&self) -> String {
        format!("redis://{}:{}", &self.host, &self.port)
    }
}

#[derive(Debug, EnumIter)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn to_string(&self) -> String {
        String::from(self.as_str())
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            o => Err(format!(
                "{} is not a supported environment mode. Use `development` or `production`",
                o
            )),
        }
    }
}
