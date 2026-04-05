use crate::{options::Options, profile::Profile};
use fetchlib::{
    client::{Client, Error},
    inputs::Inputs,
};

use std::{
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    process::ExitCode,
    time::Duration,
};

pub fn search(profile: &Profile, timeout: Duration) -> bool {
    let v4addr = profile.team.v4addr(profile.controller().hostid);
    let socket_addr = SocketAddr::new(IpAddr::V4(v4addr), 22);
    println!("Searching for {}", socket_addr);
    TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
}
/// connect to client once able to
pub fn wait_client(profile: &Profile, timeout: Duration) -> Result<Client, Error> {
    loop {
        // unable to connect to the client/ ip addr is unavaible
        if !search(profile, timeout) {
            eprintln!("Connection timed out");
            continue;
        // able to connect client
        } else {
            println!("Found connection");
            let inputs = Inputs::from(profile.clone());
            println!("Client inputs: {:?}", inputs);
            return Client::spawn(&inputs);
        }
    }
}

pub fn handle_connect(client: &Client, profile: &Profile) -> Option<ExitCode> {
    println!("Options: Please enter number u would like to do");
    println!("1.) Download last log");
    println!("2.) Get the size of logs dir");
    println!("3.) Clear logs directory of wpilogs");
    None
}

pub fn periodic(profile: &Profile) -> Option<ExitCode> {
    println!("Starting Scan");
    let client_wait = wait_client(profile, Duration::from_millis(200));
    if let Err(e) = client_wait {
        eprintln!("Unable to spawn client due to {}", e);
        return None;
    }
    println!("Able to spawn client");
    let client = client_wait.unwrap();
    if let Some(e) = handle_connect(&client, profile) {
        return Some(e);
    }
    None
}

pub fn scan(profile: &Profile) -> Option<ExitCode> {
    loop {
        periodic(profile);
    }
    None
}
