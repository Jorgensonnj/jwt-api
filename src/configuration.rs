use std::collections::HashMap;
use config::{Config, ConfigError, File};
use secrecy::SecretString;
use serde::Deserialize;
//use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub authentication: AuthenticationSettings,
    pub database: Option<DatabaseSettings>,
    pub modules: Option<HashMap<String, ModuleSettings>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationSettings {
    pub driver: String,
    pub host: String,
    pub port: u16,
}

impl ApplicationSettings {
    // Server string
    pub fn address_string(&self) -> String {
        format!(
            "{}://{}:{}",
            self.driver,
            self.host,
            self.port,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthenticationSettings {
    pub password: SecretString,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database_name: String,
    pub require_ssl: bool
}

//impl DatabaseSettings {
//    pub fn connect_options(&self) -> SqliteConnectOptions {
//
//        SqliteConnectOptions::new()
//            .host(&self.host)
//            .username(&self.username)
//            .password(self.password.expose_secret())
//            .port(self.port)
//            .database(&self.database_name)
//    }
//}

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleSettings {
    pub driver: String,
    pub host: String,
    pub port: u16,
}

impl ModuleSettings {
    // Module connection string
    pub fn address_string(&self) -> String {
        format!(
            "{}://{}:{}",
            self.driver,
            self.host,
            self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {

    let application_root_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let configuration_directory = application_root_path.join("configurations");

    // determine the type of environment
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let settings = Config::builder()
        .add_source(File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(File::from(
            configuration_directory.join(
                format!("{}.yaml", environment.as_str())
            ),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    // convert configuration into settings
    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(
                format!("{} is not a supported environment.", other)
            ),
        }
    }
}
