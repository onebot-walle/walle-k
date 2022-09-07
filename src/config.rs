use kook::prelude::Config as KookConfig;
use serde::{Deserialize, Serialize};
use walle_core::config::ImplConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    kook: KookConfig,
    onebot: ImplConfig,
}
