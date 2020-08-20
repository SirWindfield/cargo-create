use cargo_create::{
    args::Args,
    cli::{Cli, CliRunner},
};
use clap::Clap;

struct JenCli;

impl Cli for JenCli {
    fn args(&self) -> Args {
        Args::parse()
    }
}

fn main() {
    CliRunner::run(JenCli);
}
