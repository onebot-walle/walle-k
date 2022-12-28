use std::io::{Read, Write};

pub use kook::prelude::Config as KookConfig;
use serde::{Deserialize, Serialize};
use tracing::metadata::LevelFilter;
use walle_core::config::ImplConfig;

const CONFIG_PATH: &str = "walle-k.toml";

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub meta: MetaConfig,
    pub kook: KookConfig,
    pub onebot: ImplConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MetaConfig {
    pub log_level: LevelRef,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LevelRef {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl Into<LevelFilter> for LevelRef {
    fn into(self) -> LevelFilter {
        match self {
            LevelRef::Trace => LevelFilter::TRACE,
            LevelRef::Debug => LevelFilter::DEBUG,
            LevelRef::Info => LevelFilter::INFO,
            LevelRef::Warn => LevelFilter::WARN,
            LevelRef::Error => LevelFilter::ERROR,
        }
    }
}

pub fn load_from_file() -> Option<Config> {
    let path = std::path::PathBuf::from(CONFIG_PATH);
    if path.exists() {
        let mut s = String::default();
        let Ok(mut f) = std::fs::File::open(path) else {
            println!("Open config file failed");
            return None;
        };
        if f.read_to_string(&mut s).is_err() {
            println!("Read config file failed");
            return None;
        }
        match toml::from_str::<Config>(&s) {
            Ok(config) => return Some(config),
            Err(e) => {
                println!("Load config file failed:{}", e);
                return None;
            }
        }
    } else {
        let Ok(mut f) = std::fs::File::create(path) else {
            println!("Create config file failed");
            return None;
        };
        let config = Config::default();
        if f.write_all(toml::to_string(&config).unwrap().as_bytes())
            .is_err()
        {
            println!("Write config file failed");
            return None;
        }
        Some(config)
    }
}
