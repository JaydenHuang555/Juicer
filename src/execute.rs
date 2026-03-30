use std::process::ExitCode;

use directories::ProjectDirs;
use fetchlib::client::Client;
use fetchlib::inputs::Inputs;
use fetchlib::remote_file_system::RemoteFileSystem;
use once_cell::sync::Lazy;

use crate::options::{Action, DownloadInputs, DownloadMode};
use crate::{config::Config, options::Options, paths, profile::Profile};

use crate::{execute, sync};

pub fn connect(profile: &Profile) -> Result<Client, ExitCode> {
    let inputs = Inputs::from(profile.clone());
    println!("Client inputs: {:?}", inputs);
    let spawn = Client::spawn(&inputs);
    if let Err(e) = spawn {
        eprintln!("Failed to spawn client due to {}", e);
        return Err(ExitCode::from(20));
    }
    return Ok(spawn.unwrap());
}

pub fn download(inputs: &DownloadInputs, profile: &Profile, client: &Client) -> Option<ExitCode> {
    match inputs.mode {
        DownloadMode::First => {}
        DownloadMode::Select => {}
        DownloadMode::LastModified => {
            let log_paths = profile.logs();
            let last_operation = client.last_mod_file(log_paths.as_path());
            if let Err(e) = last_operation {
                eprintln!("Failed to get the last modified file due to {}", e);
                return Some(ExitCode::from(81));
            }
            let last = last_operation.unwrap();
            println!("Last file is: {:?}", last);
            println!("Downloading file");
            if let Err(e) =
                client.read_file_to_file(last.path.as_path(), inputs.destination.as_path(), false)
            {
                eprintln!("Failed to download file due to {}", e);
                return Some(ExitCode::from(31));
            }
        }
    }
    None
}

pub fn execute(action: &Action, profile: &Profile) -> Option<ExitCode> {
    let connect = connect(profile);
    if let Err(e) = connect {
        return Some(e);
    }
    let client = connect.unwrap();
    match action {
        Action::Download(inputs) => {
            if let Some(e) = download(inputs, profile, &client) {
                return Some(e);
            }
        }
        Action::List => {}
    }
    return None;
}
