use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Arc;
use std::{env, fs};

lazy_static! {
    static ref CONFIG: Arc<Config> = {
        let contents = fs::read_to_string(format!(
            "{}/{}",
            env::current_dir().unwrap().parent().unwrap().to_string_lossy(),
            "Config.toml"
        ))
        .expect("Unable to read file");
        let conf: Config = toml::de::from_str(&contents).expect("Unable to parse TOML");
        Arc::new(conf)
    };
}
#[derive(Debug, Deserialize)]
pub struct Config {
    title: String,
    redis: Redis,
    mysql: Database,
}
#[derive(Debug, Deserialize)]
pub struct Database {
    pub db_url: String,
    pub ports: u16,
    pub password: String,
    pub connection_max: u32,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub conn_addr: String,
    pub database: u16,
}

pub fn get_db_config() -> &'static Database {
    &CONFIG.mysql
}

pub fn get_redis_config() -> &'static Redis {
    &CONFIG.redis
}
