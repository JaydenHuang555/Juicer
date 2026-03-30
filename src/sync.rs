use std::{fs, io, process::ExitCode, sync::OnceLock};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

use crate::{config::Config, options::Options, paths, profile::Profile};

pub fn verify_config() -> Config {
    if !paths::config_path().exists() {
        println!("Config not found, creating new config");
        let config = Config::default();
        if let Err(e) = config.unload(paths::config_path().as_path()) {
            eprintln!("Error in unloading config due to {}", e);
            return config;
        }
    }
    match Config::load(paths::config_path().as_path()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("error in loading config due to {}", e);
            println!("resorting to default config");
            return Config::default();
        }
    }
}

pub fn loop_bool_input_str(msg: &str, yes: &str, no: &str) -> Result<bool, std::io::Error> {
    let mut input = String::new();
    loop {
        println!("{}", msg);
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Unable to get input due to {}", e);
            return Err(e);
        }
        input.pop();
        if input.to_lowercase() == yes.to_lowercase() {
            return Ok(true);
        } else if input.to_lowercase() == no.to_lowercase() {
            return Ok(false);
        }
        input.clear();
    }
}

pub fn unload_profile(profile: &Profile) -> Option<ExitCode> {
    let contents = serde_json::to_string(profile);

    if let Err(e) = contents {
        eprintln!("Unable to unload profile due to {}", e);
        let input = loop_bool_input_str("Would you like to continue?", "y", "n");
        if let Err(e) = input {
            eprintln!("Failed to get input due to {}", e);
            return Some(ExitCode::from(3));
        }
        let decession = input.unwrap();
        if decession {
            println!("continueing. skipping unloading");
            return None;
        } else {
            return Some(ExitCode::from(4));
        }
    }
    let output_path = paths::profiles_path().join(profile.name());

    if output_path.exists() {
        let dec = loop_bool_input_str(
            "Already existing profile found. Would you like to overwrite? (y / n)",
            "y",
            "n",
        );
        if let Err(e) = dec {
            eprintln!("Failed to get input due to {}", e);
            return Some(ExitCode::from(4));
        }
        let input = dec.unwrap();
        if !input {
            println!("skipping unloading");
            return None;
        }
    }

    println!("{}", output_path.display());
    if let Err(e) = fs::File::create(output_path.as_path()) {
        // TODO: add repeating checks
        eprintln!("Unable to create profile file due to {}", e);
        return Some(ExitCode::from(6));
    }

    if let Err(e) = fs::write(output_path.as_path(), contents.unwrap()) {
        // TODO: add repeating checks
        eprintln!("unable to write profile to file due to {}", e);
        return Some(ExitCode::from(1));
    }

    None
}

pub fn current_profile(config: &mut Config) -> Result<Profile, ExitCode> {
    let path = paths::profiles_path().join(config.current_profile());
    match fs::read_to_string(path) {
        Ok(contents) => {
            let convert = serde_json::from_str(contents.as_str());
            if let Err(e) = convert {
                eprintln!("Profile loading failed due to {}", e);
                return Err(ExitCode::from(9));
            }
            return Ok(convert.unwrap());
        }
        Err(e) => {
            eprintln!("Failed to load profile from file due to {}", e);
            return Err(ExitCode::from(7));
        }
    }
}

pub fn unload_config(config: &Config) -> Option<ExitCode> {
    let path = paths::config_path();
    if let Err(e) = config.unload(path.as_path()) {
        eprintln!("Failed to unload config due to {}", e);
        return Some(ExitCode::from(32));
    }
    None
}
