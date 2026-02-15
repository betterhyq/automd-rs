//! With-automdrs block generator: appends auto-updated footer.

use log::trace;

/// Config for with-automdrs block (e.g. optional message).
#[derive(Debug, Default, Clone)]
pub struct WithAutomdrsConfig {
    pub message: String,
}

pub fn generate(config: &WithAutomdrsConfig) -> Vec<String> {
    trace!("config: {:?}", config);
    vec![format!(
        "\n---\n\n_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_\n"
    )]
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
