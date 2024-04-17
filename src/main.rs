use std::ffi::OsString;
use std::fmt;

use clap::{Parser, ValueEnum};
use nextsv::{CalculatorConfig, ForceBump, Hierarchy};
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
    force: Option<ForceBump>,
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
    enforce_level: Hierarchy,
    /// Check level meets minimum for setting
    ///
    /// This option can be used to check the calculated level
    /// meets a minimum before applying an update. The program
    /// exits with an error if the threshold is not met.
    #[clap(short, long)]
    check: Option<Hierarchy>,
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

    let mut calculator_config = CalculatorConfig::new(&args.prefix);
    log::trace!("require: {:#?}", args.require);
    calculator_config.set_print_bump(args.level);
    calculator_config.set_print_version_number(args.number);
    if let Some(force) = args.force {
        calculator_config.set_force_level(force);
    };
    if !args.require.is_empty() {
        calculator_config.add_required_files(args.require);
        calculator_config.set_file_requirement_enforcement_level(args.enforce_level);
    };
    if let Some(check_level) = args.check {
        calculator_config.set_threshold(check_level);
    }
    let calculator = calculator_config.build_calculator()?;

    // Set the environment variable if required
    if let Some(key) = args.set_env {
        std::env::set_var::<OsString, OsString>(key.into(), calculator.bump_as_os_string())
    }

    println!("{}", calculator.report());

    Code::SUCCESS.ok()
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
