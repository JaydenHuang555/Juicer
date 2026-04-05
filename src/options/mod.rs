pub mod passed_profiled;

use std::path::PathBuf;

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use clap::ValueHint;

pub use crate::options::passed_profiled::PassedProfile;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum DownloadMode {
    Select,
    LastModified,
    First,
}

impl ToString for DownloadMode {
    fn to_string(&self) -> String {
        match self {
            Self::Select => "select",
            Self::First => "first",
            Self::LastModified => "last-modified",
        }
        .to_string()
    }
}

#[derive(clap::Args, Clone, Debug, PartialEq)]
pub struct DownloadInputs {
    #[clap(long, short)]
    #[clap(default_value_t = DownloadMode::LastModified)]
    pub mode: DownloadMode,

    #[clap(long, short)]
    #[clap(alias = "remote")]
    #[clap(required_if_eq("mode", "select"))]
    pub log_name: Option<String>,

    #[clap(long, short)]
    #[clap(value_hint = ValueHint::FilePath)]
    pub destination: PathBuf,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Action {
    List,
    Download(DownloadInputs),
    Scan,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ProfileLoadMode {
    Current,
    Select,
    Pass,
}

impl ToString for ProfileLoadMode {
    fn to_string(&self) -> String {
        match self {
            Self::Pass => "pass",
            Self::Current => "current",
            Self::Select => "select",
        }
        .to_string()
    }
}

#[derive(Parser, Clone, Debug)]
#[command(author, version = env!("VERSION"), about)]
#[command(propagate_version = true)]
pub struct Options {
    #[clap(subcommand)]
    pub action: Action,

    #[clap(long, short)]
    #[clap(default_value_t = ProfileLoadMode::Current)]
    pub load: ProfileLoadMode,

    #[clap(long, short)]
    #[clap(required_if_eq("load", "select"))]
    pub profile: Option<String>,

    #[clap(flatten)]
    pub passed: Option<PassedProfile>,
}

impl Options {
    pub fn action(&self) -> Action {
        self.action.clone()
    }

    pub fn profile(&self) -> Option<String> {
        self.profile.clone()
    }

    pub fn passed(&self) -> Option<PassedProfile> {
        self.passed.clone()
    }
}
