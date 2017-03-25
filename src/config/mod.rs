use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// files containing public keys in PEM
    pub keys: Vec<PathBuf>,
    /// logging level (-3 = OFF, -2 = ERROR, -1 = WARN, 0 (default) = INFO, 1 = DEBUG, 2 = TRACE)
    pub log_level: Option<i32>,
    /// a file to write a copy of the log to
    pub log_file: Option<PathBuf>,
    /// should visplay check ssl signatures on connections from localhost
    pub require_local_auth: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            keys: Vec::new(),
            log_level: Some(0),
            log_file: None,
            require_local_auth: false,
        }
    }
}