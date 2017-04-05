use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// files containing public keys in PEM
    pub keys: Vec<PathBuf>,
    /// logging level (-3 = OFF, -2 = ERROR, -1 = WARN, 0 (default) = INFO, 1 = DEBUG, 2 = TRACE)
    #[serde(default = "default_log_level")]
    pub log_level: i32,
    /// a file to write a copy of the log to
    pub log_file: Option<PathBuf>,
    /// should visplay check ssl signatures on connections from localhost
    #[serde(default = "default_require_local_auth")]
    pub require_local_auth: bool,
    /// should visplay check ssl signatures on connections not from localhost
    #[serde(default = "default_require_remote_auth")]
    pub require_remote_auth: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            keys: Vec::new(),
            log_level: default_log_level(),
            log_file: None,
            require_local_auth: default_require_local_auth(),
            require_remote_auth: default_require_remote_auth(),
        }
    }
}

fn default_log_level() -> i32 { 0 }
fn default_require_local_auth() -> bool { false }
fn default_require_remote_auth() -> bool { true }