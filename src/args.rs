use clap::Clap;
use log::LevelFilter;

#[derive(Clap, Debug)]
#[clap(about, author, version)]
#[cfg_attr(feature = "colors", clap(setting(clap::AppSettings::ColoredHelp)))]
// INVESTIGATE: is their an easier way to declare the conflicts between the
// arguments?
pub struct Args {
    /// The git branch to checkout.
    ///
    /// If not specified the default remote branch is used.
    #[clap(conflicts_with("config-path"), conflicts_with("profile"), long, short)]
    pub branch: Option<String>,
    /// Prints the path to the user configuration file.
    #[clap(
        conflicts_with("features"),
        conflicts_with("force"),
        conflicts_with("git"),
        conflicts_with("interactive"),
        conflicts_with("name"),
        conflicts_with("parameters"),
        conflicts_with("profile"),
        long,
        short
    )]
    pub config_path: bool,
    /// The list of features to use.
    ///
    /// The items are space delimited.
    #[clap(long, short)]
    pub features: Option<Vec<String>>,
    /// If true overwrites the target folder even if it is non-empty.
    #[clap(conflicts_with("config-path"), long)]
    pub force: bool,
    /// The git repository to use as a template.
    ///
    /// A branch can optionally be set by appending an `@<branch name>`.
    #[clap(
        conflicts_with("profile"),
        long,
        required_unless("config-path"),
        required_unless("profile"),
        short
    )]
    pub git: Option<String>,
    /// Whether to start the questioning process.
    #[clap(long, short)]
    pub interactive: bool,
    /// The name of the project to generate.
    #[clap(long, required_unless("config-path"), short)]
    pub name: Option<String>,
    /// Catch-all for custom variables.
    #[clap(allow_hyphen_values(true), multiple(true), last(true))]
    pub parameters: Vec<String>,
    /// The profile to use
    #[clap(
        conflicts_with("git"),
        long,
        required_unless("config-path"),
        required_unless("git"),
        short
    )]
    pub profile: Option<String>,
    /// The verbosity of the logs.
    #[clap(long, short, parse(from_occurrences))]
    pub verbosity: u8,
}

impl Args {
    /// Returns the log filter for the given verbosity level.
    pub fn log_filter_from_verbose(&self) -> LevelFilter {
        match self.verbosity {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            // Everything finer than trace will be set to trace.
            5..=std::u8::MAX => LevelFilter::Trace,
        }
    }
}
