use std::{path::PathBuf, str::FromStr};

use once_cell::sync::OnceCell;

use crate::{
    controller::{self, Controller},
    profile::Profile,
    team_number::TeamNumber,
};

pub static CITRUS_CIRCUITS: OnceCell<Profile> = OnceCell::new();

pub fn citrus_circuits() -> &'static Profile {
    CITRUS_CIRCUITS.get_or_init(|| Profile {
        name: "citrus".to_string(),
        team: TeamNumber::from(1678),
        logs: PathBuf::from("/U/logs"),
        controller: controller::defaults::rio().clone(),
    })
}
