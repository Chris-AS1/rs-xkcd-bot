use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
use serde::Serialize;
use teloxide::Bot;

#[derive(Debug, Clone)]
pub struct BotInterface {
    pub bot: Bot,
    pub bot_settings: BotSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BotSettings {
    pub token: String,
    pub xkcd_url: String,
    pub user_agent: String,
    pub test: String,
}

impl BotInterface {
    pub fn new() -> Result<BotInterface, ConfigError> {
        let bot_settings = BotInterface::load_configuration()?;
        let bot = Bot::new(&bot_settings.token);

        Ok(Self { bot, bot_settings })
    }

    pub fn from_param(token: String) -> Result<BotInterface, ConfigError> {
        let bot_settings = BotInterface::load_configuration()?;
        let bot = Bot::new(token);

        Ok(Self { bot, bot_settings })
    }

    pub fn load_configuration() -> Result<BotSettings, ConfigError> {
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
