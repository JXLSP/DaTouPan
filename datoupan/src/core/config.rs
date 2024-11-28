use config::Config;
use std::{fs, sync::OnceLock};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(cfg_file: &str) {
    let path = fs::canonicalize(cfg_file)
        .unwrap_or_else(|_| panic!("Cannot find config file: {:?}", cfg_file));

    let cfg = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|_| panic!("Cannot find config file: {:?}", cfg_file));

    let _ = CONFIG.set(cfg);
}

pub fn global_config() -> &'static Config {
    CONFIG.get().unwrap_or_else(|e| panic!("Global config not available: {}", e))
}