use std::ffi::OsString;
use std::fmt;

use clap::{Parser, ValueEnum};
use nextsv::{Answer, Error, ForceLevel, Semantic, TypeHierarchy, VersionCalculator};
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
    force: Option<ForceOptions>,
    /// Prefix string to identify version number tags
    #[arg(short, long, value_parser, default_value = "v")]
    prefix: String,
    /// Report the level of the version number change
    #[arg(long)]
    level: bool,
    /// Report the version number
    #[arg(long)]
    number: bool,
    /// Set a pre-release string (optional)
    /// [example values: alpha, beta, rc]
    #[arg(long, default_value = None)]
    pre_release: Option<String>,
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

    let latest_version = VersionCalculator::new(&args.prefix, args.pre_release)?;

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
    force: Option<ForceOptions>,
    files: Option<Vec<OsString>>,
    enforce_level: TypeHierarchy,
) -> Result<Answer, Error> {
    if let Some(f) = &force {
        log::debug!("Force option set to {}", f);
    };

    let pre_release = latest_version.get_pre_release();
    let has_existing_pre_release: bool = has_existing_pre_release(latest_version.name());
    if has_existing_pre_release
        && pre_release.is_some()
        && latest_version.name().pre_release().unwrap().suffix() == pre_release.clone().unwrap()
    {
        // increment existing pre-release only
        let new_version = latest_version.name().increment_pre_release().clone();
        let answer = Answer::new(nextsv::Level::PreRelease, new_version, None);
        return Ok(answer);
    }

    latest_version = latest_version.walk_commits()?;
    if let Some(f) = files {
        latest_version.has_required(f, enforce_level)?;
    }

    let mut answer = if let Some(svc) = force {
        match svc {
            ForceOptions::Major => latest_version.force(ForceLevel::Major).next_version(),
            ForceOptions::Minor => latest_version.force(ForceLevel::Minor).next_version(),
            ForceOptions::Patch => latest_version.force(ForceLevel::Patch).next_version(),
            ForceOptions::First => latest_version.promote_first()?,
        }
    } else {
        if pre_release.is_none() && has_existing_pre_release {
            // just promote pre-release
            let new_version = latest_version.name().unset_pre_release().clone();
            return Ok(Answer::new(nextsv::Level::Release, new_version, None));
        }
        let mut answer = latest_version.next_version();
        let mut next_version = answer.version_number.clone();
        if pre_release.is_some() {
            if has_existing_pre_release {
                next_version = latest_version.name();
            }
            next_version = next_version
                .first_pre_release(&pre_release.unwrap().to_string())
                .clone();
            answer = Answer::new(nextsv::Level::PreRelease, next_version, None);
        }
        answer
    };

    answer.change_level = latest_version.top_level();

    Ok(answer)
}

/// Reports if version is a Pre-Release
///
fn has_existing_pre_release(version: Semantic) -> bool {
    match version.pre_release() {
        Some(_) => {
            log::info!("Is a pre-release: {}", version);
            true
        }
        None => {
            log::info!("Is not a pre-release: {}", version);
            false
        }
    }
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
