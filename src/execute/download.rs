use std::path::Path;
use std::process::ExitCode;

use directories::ProjectDirs;
use fetchlib::client::Client;
use fetchlib::fs::FileMetaData;
use fetchlib::fs::sort::FileSortType;
use fetchlib::inputs::Inputs;
use fetchlib::remote_file_system::RemoteFileSystem;
use fetchlib::remote_interact::RemoteTransferProtocol;
use fetchlib::sftp::Sftp;
use once_cell::sync::Lazy;

use crate::options::{Action, DownloadInputs, DownloadMode};
use crate::{config::Config, options::Options, paths, profile::Profile};

use crate::{execute, sync, util};

fn file(source: &FileMetaData, destination: &Path, client: &Client) -> Option<ExitCode> {
    println!("Input download file: {:?}", source);
    if let Some(size) = source.size.clone() {
        println!("File size is {}", util::bytes::format(size));
    } else {
        eprintln!("Unable to get file size");
    }

    println!("Downloading file: {:?}", source);
    if let Err(e) = client.read_file_to_file(source.path.as_path(), destination, false) {
        eprintln!("Failed to download file due to {}", e);
        Some(ExitCode::from(31))
    } else {
        println!(
            "{} Finished downloading to {}",
            source.path.display(),
            destination.display()
        );
        None
    }
}

fn first(log_path: &Path, destination: &Path, client: &Client, sftp: &Sftp) -> Option<ExitCode> {
    let first_operation = sftp.dirsort_file(log_path, FileSortType::FirstCreated);
    if let Err(e) = first_operation {
        eprintln!("Failed to get the first created file due to {}", e);
        return Some(ExitCode::from(81));
    }
    let first = first_operation.unwrap();
    if let Err(e) = client.read_file_to_file(first.path.as_path(), destination, false) {
        eprintln!("Failed to download file due to {}", e);
        return Some(ExitCode::from(31));
    }
    None
}

pub fn last_mod(
    log_path: &Path,
    destination: &Path,
    client: &Client,
    sftp: &Sftp,
) -> Option<ExitCode> {
    let last_operation = sftp.dirsort_file(log_path, FileSortType::LastModified);
    if let Err(e) = last_operation {
        eprintln!("Failed to get the last modified file due to {}", e);
        return Some(ExitCode::from(81));
    }
    let last = last_operation.unwrap();
    if let Some(code) = file(&last, destination, client) {
        Some(code)
    } else {
        None
    }
}

pub fn download(
    inputs: &DownloadInputs,
    profile: &Profile,
    client: &Client,
    sftp: &Sftp,
) -> Option<ExitCode> {
    match inputs.mode {
        DownloadMode::First => first(
            profile.logs().as_path(),
            inputs.destination.as_path(),
            client,
            sftp,
        ),
        DownloadMode::Select => {
            let path = profile
                .logs()
                .join(inputs.log_name.clone().unwrap())
                .with_extension(".wpilog");
            if !sftp.path_exists(path.as_path()) {
                eprintln!("Path does not exist");
                return Some(ExitCode::from(82));
            }
            let meta_data = sftp.file_metadata(path.as_path());
            if let Err(e) = meta_data {
                eprintln!("Failed to get the file for {} due to {}", path.display(), e);
                return Some(ExitCode::from(21));
            }
            if let Some(code) = file(&meta_data.unwrap(), inputs.destination.as_path(), client) {
                return Some(code);
            }
            None
        }
        DownloadMode::LastModified => last_mod(
            profile.logs().as_path(),
            inputs.destination.as_path(),
            client,
            sftp,
        ),
    }
}
