//! With automdrs block generator.

use log::trace;

#[derive(Debug, Default, Clone)]
pub struct WithAutomdrsConfig {
    pub message: String,
}

pub fn generate(config: &WithAutomdrsConfig) -> Vec<String> {
    trace!("config: {:?}", config);
    vec![format!("---<br>_🤖 auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_")]
}