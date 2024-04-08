use std::ffi::OsString;
use std::fmt;

use clap::{Parser, ValueEnum};
use nextsv::{Answer, Error, ForceLevel, TypeHierarchy, VersionCalculator};
use proc_exit::{Code, ExitResult};

#[derive(ValueEnum, Debug, Clone)]
enum ForceOptions {
    Major,
    Minor,
    Patch,
    First,
}

impl fmt::Display for ForceOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceOptions::Major => write!(f, "major"),
            ForceOptions::Minor => write!(f, "minor"),
            ForceOptions::Patch => write!(f, "patch"),
            ForceOptions::First => write!(f, "first"),
        }
    }
}
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    /// Force the calculation of the version number
    #[arg(short, long, value_enum)]
    force: Option<ForceLevel>,
    /// Prefix string to identify version number tags
    #[arg(short, long, value_parser, default_value = "v")]
    prefix: String,
    /// Report the level of the version number change
    #[arg(long)]
    level: bool,
    /// Report the version number
    #[arg(long)]
    number: bool,
    /// Require changes to these file before building release
    #[arg(short, long)]
    require: Vec<OsString>,
    /// Level at which required files should be enforced
    #[clap(short, long, default_value = "feature")]
    enforce_level: TypeHierarchy,
    /// Check level meets minimum for setting
    ///
    /// This option can be used to check the calculated level
    /// meets a minimum before applying an update. The program
    /// exits with an error if the threshold is not met.
    #[clap(short, long)]
    check: Option<TypeHierarchy>,
    /// add output to environment variable
    #[clap(long, default_value = "NEXTSV_LEVEL")]
    set_env: Option<String>,
}

fn main() {
    let result = run();
    proc_exit::exit(result);
}

fn run() -> ExitResult {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level_filter());
    builder.init();

    match (args.number, args.level) {
        (false, false) => log::info!("Calculating the next version level"),
        (false, true) => log::info!("Calculating the next version level"),
        (true, false) => log::info!("Calculating the next version number"),
        (true, true) => log::info!("Calculating the next version number and level"),
    };

    let latest_version = VersionCalculator::new(&args.prefix)?;

    log::trace!("require: {:#?}", args.require);

    // Encapsulate the list of required files in an option
    let files = if args.require.is_empty() {
        Option::None
    } else {
        Option::Some(args.require)
    };

    let resp = calculate(latest_version, args.force, files, args.enforce_level)?;

    set_environment_variable(args.set_env, resp.bump_level.to_string().into());
    check_level(args.check, resp.change_level())?;
    log::debug!("not checking so print the output");
    print_output(args.number, args.level, resp);

    Code::SUCCESS.ok()
}

fn check_level(threshold: Option<TypeHierarchy>, change_level: TypeHierarchy) -> Result<(), Error> {
    if let Some(minimum_level) = threshold {
        log::debug!("level expected is {:?}", &minimum_level);
        log::debug!("level reported is {:?}", &change_level);
        if change_level >= minimum_level {
            log::info!("the minimum level is met");
            return Err(Error::MinimumChangeLevelMet);
        } else {
            log::info!("the minimum level is not met");
            return Err(Error::MinimumChangeLevelNotMet);
        };
    }
    Ok(())
}

fn set_environment_variable(env_variable: Option<String>, value: OsString) {
    if let Some(key) = env_variable {
        std::env::set_var(key, value)
    }
}

fn calculate(
    mut latest_version: VersionCalculator,
    force: Option<ForceLevel>,
    files: Option<Vec<OsString>>,
    enforce_level: TypeHierarchy,
) -> Result<Answer, Error> {
    if let Some(f) = &force {
        log::debug!("Force option set to {}", f);
    };
    latest_version = latest_version.walk_commits()?;
    if let Some(f) = files {
        latest_version.has_required(f, enforce_level)?;
    }
    let mut answer = if let Some(svc) = force {
        latest_version.set_force(Some(svc)).next_version()
    } else {
        latest_version.next_version()
    };

    answer.change_level = latest_version.top_level();

    Ok(answer)
}

pub fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}

/// Print the output from the calculation
///
fn print_output(number: bool, level: bool, response: Answer) {
    match (number, level) {
        (false, false) => println!("{}", response.bump_level),
        (false, true) => println!("{}", response.bump_level),
        (true, false) => println!("{}", response.version_number),
        (true, true) => println!("{}\n{}", response.version_number, response.bump_level),
    }
}
