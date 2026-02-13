//! With automdrs block generator.

use log::trace;

#[derive(Debug, Default, Clone)]
pub struct WithAutomdrsConfig {
    pub message: String,
}

pub fn generate(config: &WithAutomdrsConfig) -> Vec<String> {
    trace!("config: {:?}", config);
    vec![format!("---\n<br>_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_")]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let config = WithAutomdrsConfig::default();
        let out = generate(&config);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("automd-rs"));
    }
}