mod config;
mod handler;
pub(crate) mod parse;
pub mod structs;

pub const WALLE_K: &str = "Walle-K";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use config::*;
pub use handler::KHandler;
