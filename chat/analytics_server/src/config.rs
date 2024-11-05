use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub pk: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_name: String,
    pub base_dir: PathBuf,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // 思考: 这里同时打开了三个文件去判断，会影响到效率(优化做法，按优先级打开，然后再判断是否需要打开下一个)，但这里是在程序初始化的时候去做，所以问题不大，可以接受
        // read from ./analytics.yml, or /etc/config/analytics.yml, or from env analytics_CONFIG
        let ret: AppConfig = match (
            File::open("analytics.yml"),
            File::open("/etc/config/analytics.yml"),
            env::var("ANALYTICS_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader)?,
            (_, Ok(reader), _) => serde_yaml::from_reader(reader)?,
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?)?,
            _ => bail!("Config file analytics.yml not found"),
        };

        Ok(ret)
    }
}
