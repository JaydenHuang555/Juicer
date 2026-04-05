pub mod profiles;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use fetchlib::inputs::Inputs;
use serde::{Deserialize, Serialize};

use crate::util::frc::{Controller, team_number::TeamNumber};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub team: TeamNumber,
    pub logs: PathBuf,
    pub controller: Controller,
}

impl Profile {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn team(&self) -> TeamNumber {
        self.team.clone()
    }

    pub fn logs(&self) -> PathBuf {
        self.logs.clone()
    }

    pub fn controller(&self) -> Controller {
        self.controller.clone()
    }
}

impl From<Profile> for Inputs {
    fn from(value: Profile) -> Self {
        let v4 = value.team().v4addr(value.controller.hostid);
        let credentials = value.controller().credentials();
        Self {
            addr: SocketAddr::new(IpAddr::V4(v4), 22),
            credentials,
        }
    }
}
