use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// list of api keys
    pub keys: Vec<String>,
    /// logging level (-3 = OFF, -2 = ERROR, -1 = WARN, 0 (default) = INFO, 1 = DEBUG, 2 = TRACE)
    #[serde(default = "default_log_level")]
    pub log_level: i32,
    /// a file to write a copy of the log to
    pub log_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            keys: Vec::new(),
            log_level: default_log_level(),
            log_file: None,
        }
    }
}

fn default_log_level() -> i32 { 0 }