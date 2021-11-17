
use serde::Deserialize;
use std::env::current_exe;
use std::fs;
use std::path::Path;
use toml;

/// The master format for the configuration file
///
/// Each of the sub-modules requiring configuration has its own Config struct describing the way
/// that data should be laid out within its table. This allows changes to be made closer to where
/// they are used.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The log level to be used by the program. This value will be parsed to a `LogLevel` to
    /// determine how much logging output the program should generate. Valid values are `off`,
    /// `error`, `warn`, `info`, `debug`, and `trace`. Note that if the passed string is not one
    /// of these values, the program will error out and refuse to start.
    pub log_level: String,
}

impl Config {
    /// Creates a new set of configurations, either by reading them off the disk, or by using
    /// defaults and saving them in the correct location.
    ///
    /// It is assumed that the configuration file will be adjacent to the executable with the name
    /// `Config.toml`. For example, if the executable is `/home/user/project/foo` then the config
    /// file is expected to be at `/home/user/project/Config.toml`.
    ///
    /// No guarantees are made about case (in)sensitivity for the file name. Windows *may* allow
    /// `config.toml` (with a small 'c'), but it also may not.
    pub fn new() -> Result<Self, String> {
        let mut executable = current_exe().map_err(|x| x.to_string())?;
        executable.pop();
        let config_path = executable.join("Config.toml");

        match Self::read(config_path.as_path()) {
            Ok(config) => return Ok(config),
            Err(_) => eprintln!(
                "Unable to read configuration file. Creating default file: {}",
                config_path.to_str().unwrap_or_default()
            ),
        }

        Self::generate_new_config(config_path.as_path())
    }

    /// Attempts to read the configuration file off the disk.
    fn read(config_path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(&config_path).map_err(|x| x.to_string())?;
        toml::from_str(&text).map_err(|x| x.to_string())
    }

    /// Attempts to generate a new configuration file
    fn generate_new_config(config_path: &Path) -> Result<Self, String> {
        let default_config = include_str!("default_config.toml");
        fs::write(config_path, default_config).map_err(|x| x.to_string())?;
        toml::from_str(default_config).map_err(|x| x.to_string())
    }
}