pub mod defaults;

use std::net::IpAddr;

use fetchlib::key::credentials::Credentials;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Controller {
    pub default_addr: IpAddr,
    pub hostid: u8,
    pub credentials: Credentials,
}

impl Controller {
    pub fn new(default_addr: IpAddr, hostid: u8, credentials: Credentials) -> Self {
        Self {
            default_addr,
            hostid,
            credentials,
        }
    }

    pub fn default_addr(&self) -> IpAddr {
        self.default_addr
    }

    pub fn credentials(&self) -> Credentials {
        self.credentials.clone()
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ControllerType {
    Rio1,
    Rio2,
    SystemCore,
}

impl ControllerType {
    pub fn controller(&self) -> Controller {
        match self {
            Self::Rio1 | Self::Rio2 => defaults::rio().clone(),
            Self::SystemCore => defaults::systemcore().clone(),
        }
    }
}

crate::paths::enum_to_string!(ControllerType);
