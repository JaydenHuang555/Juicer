use std::process::ExitCode;

use directories::ProjectDirs;
use fetchlib::client::Client;
use fetchlib::fs::sort::FileSortType;
use fetchlib::inputs::Inputs;
use fetchlib::remote_file_system::RemoteFileSystem;
use fetchlib::remote_interact::RemoteTransferProtocol;
use fetchlib::sftp::Sftp;
use once_cell::sync::Lazy;

use crate::options::{Action, DownloadInputs, DownloadMode};
use crate::{config::Config, options::Options, paths, profile::Profile};

use crate::{execute, sync};

pub fn connect(profile: &Profile) -> Result<Client, ExitCode> {
    let inputs = Inputs::from(profile.clone());
    println!("Client inputs: {:?}", inputs);
    let spawn = Client::spawn_ssh(&inputs);
    if let Err(e) = spawn {
        eprintln!("Failed to spawn client due to {}", e);
        return Err(ExitCode::from(20));
    }
    println!("Able to spawn client");
    return Ok(spawn.unwrap());
}

pub fn download(
    inputs: &DownloadInputs,
    profile: &Profile,
    client: &Client,
    sftp: &Sftp,
) -> Option<ExitCode> {
    match inputs.mode {
        DownloadMode::First => {
            let log_path = profile.logs();
            let first_operation = sftp.dirsort_file(log_path.as_path(), FileSortType::FirstCreated);
            if let Err(e) = first_operation {
                eprintln!("Failed to get the first created file due to {}", e);
                return Some(ExitCode::from(81));
            }
            let first = first_operation.unwrap();
            println!("First file is {:?}", first);
            println!("Downloading file");
            if let Err(e) =
                client.read_file_to_file(first.path.as_path(), inputs.destination.as_path(), false)
            {
                eprintln!("Failed to download file due to {}", e);
                return Some(ExitCode::from(31));
            }
        }
        DownloadMode::Select => {
            let path = profile
                .logs()
                .join(inputs.log_name.clone().unwrap())
                .with_extension(".wpilog");
            if !sftp.path_exists(path.as_path()) {
                eprintln!("Path does not exist");
                return Some(ExitCode::from(82));
            }
            println!("Downloading file");
            if let Err(e) =
                client.read_file_to_file(path.as_path(), inputs.destination.as_path(), false)
            {
                eprintln!("Failed to download file due to {}", e);
                return Some(ExitCode::from(31));
            }
        }
        DownloadMode::LastModified => {
            let log_paths = profile.logs();
            let last_operation = sftp.dirsort_file(log_paths.as_path(), FileSortType::LastModified);
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

    println!("Starting sftp");
    let sftp_operation = client.sftp();
    if let Err(e) = sftp_operation {
        eprintln!("Unable to start sftp due to {}!", e);
        return Some(ExitCode::from(45));
    }
    let sftp = sftp_operation.unwrap();
    println!("Able to start sftp");

    match action {
        Action::Download(inputs) => {
            if let Some(e) = download(inputs, profile, &client, &sftp) {
                return Some(e);
            }
        }
        Action::List => match sftp.listdir(profile.logs().as_path()) {
            Ok(files) => {
                for file in files {
                    println!("{:?}", file);
                }
            }
            Err(e) => {
                eprintln!("Unable to list files due to {}", e);
                return Some(ExitCode::from(91));
            }
        },
    }
    return None;
}
