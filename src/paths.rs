use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::{fs, io, process::ExitCode, sync::OnceLock};

use crate::juicer;

static DIRECTORIES: OnceLock<ProjectDirs> = OnceLock::new();

pub fn directories() -> &'static ProjectDirs {
    DIRECTORIES.get_or_init(|| {
        if let Some(e) = ProjectDirs::from("com", "jayden", "juicer") {
            return e;
        };
        eprintln!("Unable to get directories");
        std::process::exit(3);
    })
}

pub fn config_path() -> PathBuf {
    directories().config_dir().join("config.json")
}

pub fn profiles_path() -> PathBuf {
    directories().data_dir().join("profiles")
}

pub fn ensure_created_path(path: &Path) -> Result<(), std::io::Error> {
    if !path.exists() {
        if let Err(e) = std::fs::create_dir_all(path) {
            eprintln!("Unable to create path {} due to {}", path.display(), e);
            return Err(e);
        }
    }
    Ok(())
}

pub fn ensure_created() -> Result<(), std::io::Error> {
    let paths = [config_path(), profiles_path()];
    for path in paths {
        if let Err(e) = ensure_created_path(path.as_path()) {
            return Err(e);
        }
    }
    return Ok(());
}

macro_rules! enum_to_string {
    ($enum:ident) => {
        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}
pub(crate) use enum_to_string;
