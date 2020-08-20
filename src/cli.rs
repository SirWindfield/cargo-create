use crate::args::Args;
use crate::config::user::user_config_file_path;
use crate::start;

use std::process::exit;

/// The trait that the two binaries implement to abstract away some logic.
#[doc(hidden)]
pub trait Cli {
    /// Constructs an `Args` instance.
    fn args(&self) -> Args;
}

/// Runs a Cli instance.
#[doc(hidden)]
pub struct CliRunner;

impl CliRunner {
    pub fn run(cli: impl Cli) {
        let args = cli.args();

        let user_config_path =
            user_config_file_path().expect("failed to resolve user config file path");

        if args.config_path {
            println!("{}", user_config_path.display());
        } else {
            if let Err(e) = start(args, user_config_path) {
                eprintln!("An error occurred during execution: {:?}", e);
                exit(1);
            }
        }
    }
}
