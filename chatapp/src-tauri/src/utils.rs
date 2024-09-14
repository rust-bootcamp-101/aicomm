use std::path::PathBuf;

const APP_DIR: &str = ".chatapp";
const LOG_DIR: &str = "log";
const CACHE_DIR: &str = "cache";
const DB_DIR: &str = "db";
const CONFIG_DIR: &str = "config";

#[allow(unused)]
#[inline]
pub(crate) fn app_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(APP_DIR)
}

#[inline]
pub(crate) fn log_dir() -> PathBuf {
    app_dir().join(LOG_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn cache_dir() -> PathBuf {
    app_dir().join(CACHE_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn db_dir() -> PathBuf {
    app_dir().join(DB_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn config_dir() -> PathBuf {
    app_dir().join(CONFIG_DIR)
}
