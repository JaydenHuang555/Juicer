use std::{fs, io, process::ExitCode, sync::OnceLock};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

use crate::options::Action;
use crate::{config::Config, options::Options, paths, profile::Profile};

use crate::{execute, sync};

pub fn run(options: &Options) -> ExitCode {
    // ensure directories are set
    if let Err(_) = paths::ensure_created() {
        return ExitCode::from(82);
    }

    // aquire config
    let mut config = sync::verify_config();

    let loaded_profile;

    // profile is passed in
    if let Some(passed) = options.passed() {
        loaded_profile = Profile::from(passed.clone());
        if let Some(e) = sync::unload_profile(&loaded_profile) {
            return e;
        }
        // assign profile
        config.set_current_profile(passed.clone().name.unwrap());
        sync::unload_config(&config);
    }
    // profile is the current
    else {
        let cur = sync::current_profile(&mut config);
        if let Err(e) = cur {
            return e;
        }
        loaded_profile = cur.unwrap();
        config.set_current_profile(loaded_profile.name());
    }

    println!("{:?}", loaded_profile);

    // execute action
    if let Some(e) = execute::execute(&options.action, &loaded_profile) {
        return e;
    }

    ExitCode::SUCCESS
}
