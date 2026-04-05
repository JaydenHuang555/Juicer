use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use fetchlib::{
    client::{Client, Error},
    remote_file_system::RemoteFileSystem,
};

use crate::{
    input::{input, loop_bool_input_str},
    profile::Profile,
};

pub fn get_local_path() -> Result<(PathBuf, bool), Option<ExitCode>> {
    let mut buff = String::new();
    loop {
        println!("Download path: (enter directory if fname = log name)");
        let input = input(std::io::stdin(), &mut buff);
        if let Err(e) = input {
            eprintln!("Unable to get user input due to {}", e);
            return Err(Some(ExitCode::from(32)));
        }
        let path = Path::new(&buff);
        if path.exists() {
            if path.is_dir() {
                return Ok((path.to_path_buf(), false));
            } else {
                let dec = loop_bool_input_str(
                    "Path exits. Would you like to override? (y / n)",
                    "y",
                    "n",
                );
                if dec.is_err() {
                    eprintln!("Unable to get user input due to {}", dec.unwrap_err());
                    return Err(Some(ExitCode::from(32)));
                }
                if dec.unwrap() {
                    println!("Overriding");
                    return Ok((path.to_path_buf(), true));
                }
            }
        }
        buff.clear();
    }
}

pub fn last_log(client: &Client, profile: &Profile) -> Option<ExitCode> {
    let local_inputs = get_local_path();
    if let Err(err) = local_inputs {
        if let Some(e) = err {
            return Some(e);
        }
        return None;
    }
    let local = local_inputs.unwrap();
    let is_path_to_file = local.1;
    let path = local.0;
    let destination;

    let latest = client.last_mod_file(&profile.logs);
    if let Err(e) = latest {
        eprintln!("Failed to get latest file");
        return Some(ExitCode::from(19));
    }

    let last_file = latest.unwrap();

    let source = last_file.path.clone();

    if !is_path_to_file {
        destination = path.join(source).with_extension("wpilog");
    } else {
        destination = path;
    }

    println!("Last File: {:?}", last_file);
    match client.read_file_to_file(last_file.path.as_path(), destination.as_path(), true) {
        Ok(s) => {
            println!("Finished downloading log file");
            return None;
        }
        Err(e) => {
            eprintln!("Failed to write log file to file due to {}", e);
            return None;
        }
    }
}
