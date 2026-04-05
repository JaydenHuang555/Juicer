use std::path::PathBuf;

use crate::profile::Profile;

use crate::util::frc::{ControllerType, TeamNumber};

#[derive(clap::Args, Clone, Debug)]
pub struct PassedProfile {
    #[clap(long)]
    #[clap(required_if_eq("load", "pass"))]
    pub name: Option<String>,

    #[clap(long)]
    #[clap(required_if_eq("load", "pass"))]
    pub team: Option<u32>,

    #[clap(long)]
    #[clap(required_if_eq("load", "pass"))]
    pub log_path: Option<PathBuf>,

    // TODO: add support for custom controllers
    #[clap(value_enum)]
    #[clap(default_value_t = ControllerType::Rio2)]
    pub controller: ControllerType,
}

macro_rules! empty_named_vars {
    ($instance:expr, $($field:ident),*) => {{
        let mut missing = Vec::new();
        $(
            if $instance.$field.is_none() {
                missing.push(stringify!($field));
            }
        )*
        missing
    }};
}

impl PassedProfile {
    pub fn validate(&self) -> Result<(), Vec<&str>> {
        let missing = empty_named_vars!(self, name, team, log_path);
        if missing.is_empty() {
            return Ok(());
        }
        return Err(missing);
    }
}

impl From<PassedProfile> for Profile {
    fn from(value: PassedProfile) -> Self {
        Self {
            name: value.name.unwrap(),
            team: TeamNumber::from(value.team.unwrap()),
            logs: value.log_path.unwrap(),
            controller: value.controller.controller(),
        }
    }
}
