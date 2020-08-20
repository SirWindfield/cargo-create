use cargo_create::{
    args::Args,
    cli::{Cli, CliRunner},
};
use clap::{
    derive::{FromArgMatches, IntoApp},
    AppSettings,
};

struct CargoCreateCli;

impl Cli for CargoCreateCli {
    fn args(&self) -> Args {
        // `clap` programs used as cargo subcommands need the first two args removed
        // before parsing.
        let args = std::env::args_os().skip(2);
        // The binary name has to be cargo here.
        let app = Args::into_app();
        // Since the binary name is stripped away earlier, we need to tell `clap` to not
        // assume that the name exists.
        let matches = app
            .bin_name("cargo create")
            .setting(AppSettings::NoBinaryName)
            .get_matches_from(args);

        Args::from_arg_matches(&matches)
    }
}

fn main() {
    CliRunner::run(CargoCreateCli);
}
