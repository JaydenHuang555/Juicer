use std::process::ExitCode;

use clap::Parser;

use crate::options::Options;

pub mod config;
pub mod execute;
pub mod juicer;
pub mod options;
pub mod paths;
pub mod profile;
pub mod sync;
pub mod util;

fn main() -> ExitCode {
    let options = Options::parse();

    if let Some(passed) = options.passed() {
        if let Err(missing) = passed.validate() {
            eprintln!("Passed profile is incomplete!!!");
            let mut built = String::from("Params missing: ");
            for i in 0..missing.len() {
                built.push('[');
                built.push_str(missing[i]);
                built.push(']');
            }
            return ExitCode::from(14);
        }
    }

    juicer::run(&options)
}
