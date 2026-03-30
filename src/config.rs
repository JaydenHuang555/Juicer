use std::{
    fmt::Display,
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{juicer, paths};

#[derive(Debug)]
pub enum ConfigErrorKind {
    IO(std::io::Error),
    Conversion(serde_json::Error),
}

impl Display for ConfigErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IO(e) => format!("IO Operation failed: {}", e),
                Self::Conversion(e) => format!("Converting failed: {}", e),
            }
        )
    }
}

#[derive(Debug)]
pub struct ConfigError {
    kind: ConfigErrorKind,
    display: Option<&'static str>,
}

impl ConfigError {
    pub fn new(kind: ConfigErrorKind, display: Option<&'static str>) -> Self {
        Self { kind, display }
    }

    pub fn io(e: std::io::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ConfigErrorKind::IO(e),
            display,
        }
    }

    pub fn conversion(e: serde_json::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ConfigErrorKind::Conversion(e),
            display,
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = self.display.clone() {
            write!(f, "({}): {}", msg, self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub profile_storage_path: PathBuf,
    pub current_profile: String,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let read = fs::read_to_string(path);
        if let Err(e) = read {
            return Err(ConfigError::io(e, Some("Unable to read from path")));
        }
        let contents = read.unwrap();
        match serde_json::from_str(&contents) {
            Ok(s) => Ok(s),
            Err(e) => Err(ConfigError::conversion(e, Some("Failed to serialize self"))),
        }
    }

    pub fn unload(&self, path: &Path) -> Result<(), ConfigError> {
        let convert = serde_json::to_string(self);
        if let Err(e) = convert {
            return Err(ConfigError::conversion(
                e,
                Some("Failed to deserialize self"),
            ));
        }
        let content = convert.unwrap();
        if let Err(e) = fs::File::create(path) {
            return Err(ConfigError::io(e, Some("Unable to access directory")));
        }
        if let Err(e) = fs::write(path, content) {
            return Err(ConfigError::io(e, Some("Unable to unload to file")));
        }
        Ok(())
    }

    pub fn current_profile(&self) -> String {
        self.current_profile.clone()
    }

    pub fn set_current_profile(&mut self, next: String) {
        self.current_profile = next;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profile_storage_path: paths::profiles_path(),
            current_profile: "JUICER_CURRENT_PROFILE".to_string(),
        }
    }
}
