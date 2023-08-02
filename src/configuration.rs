use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub token: String,
    pub test: String,
}

enum Environment {
    Development,
    Production,
}

impl Environment {
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
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            o => Err(format!(
                "{} is not a supported environment mode. Use `development` or `production`",
                o
            )),
        }
    }
}

pub fn load_configuration() -> Result<Settings, ConfigError> {
    match std::env::var("APP_ENV") {
        Ok(v) => v.try_into().unwrap_or_else(|_| Environment::Development),
        Err(_) => Environment::Development,
    };

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

    builder.build()?.try_deserialize()
}
