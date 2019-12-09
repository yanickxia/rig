use config::{Config, ConfigError, Environment, File};
use log::debug;

use crate::api::Api;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::load().unwrap();
    pub static ref APIS: Vec<Api> = Apis::load().unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub server: Server,
    pub tasks: Tasks,
}

#[derive(Debug, Deserialize)]
pub struct Apis {
    pub apis: Vec<Api>
}

#[derive(Debug, Deserialize)]
pub struct Tasks {
    pub api: ApiTask
}

#[derive(Debug, Deserialize)]
pub struct ApiTask {
    pub interval: u64
}


impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/application.yaml"))?;
        s.merge(Environment::with_prefix("RIG"))?;
        debug!("Loaded Configurations: {:?}", s);
        s.try_into()
    }
}

impl Apis {
    pub fn load() -> Result<Vec<Api>, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/api.json"))?;
        debug!("Loaded API Configurations: {:?}", s);
        s.try_into::<Apis>().map(|it| it.apis)
    }

    pub fn file_load(file: &str) -> Result<Vec<Api>, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(file))?;
        debug!("Loaded API Configurations: {:?}", s);
        s.try_into::<Apis>().map(|it| it.apis)
    }
}