pub mod download;

use std::process::ExitCode;

use fetchlib::client::Client;
use fetchlib::inputs::Inputs;
use fetchlib::remote_file_system::RemoteFileSystem;

use crate::options::Action;
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
            if let Some(e) = download::download(inputs, profile, &client, &sftp) {
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
        Action::Size => {
            let log_path = profile.logs();
            match sftp.file_metadata(log_path.as_path()) {
                Ok(data) => {
                    if let Some(size) = data.size {
                        println!("Size for log dir ({}): {}", log_path.display(), size);
                    } else {
                        eprintln!("Unable to get size from meta data due to it being empty");
                        return Some(ExitCode::from(13));
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Unable to get meta data for {} due to {}",
                        log_path.display(),
                        e
                    );
                    return Some(ExitCode::from(13));
                }
            }
        }
    }
    return None;
}
