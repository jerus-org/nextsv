use std::ffi::OsString;

use clap::Parser;
use nextsv::{CalculatorConfig, ForceBump, Hierarchy};
use proc_exit::{Code, ExitResult};

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
    /// Do not report version bump
    #[arg(short = 'b', long)]
    no_bump: bool,
    /// Report the version number
    #[arg(short = 'n', long)]
    number: bool,
    /// Files that require changes before making a release
    ///
    /// The level at which the required files are enforced
    /// can be set with the `enforce` option.
    #[arg(short, long)]
    require: Vec<OsString>,
    /// Bump level at which required files list should be enforced
    ///
    /// Should be used in conjunction with the `require` option.
    #[clap(short, long, default_value = "feature")]
    enforce: Hierarchy,
    /// Check level meets minimum for setting
    ///
    /// This option can be used to check the calculated level
    /// meets a minimum before applying an update. Bump is reported
    /// as "none" if the required level is not met.
    #[clap(short, long)]
    check: Option<Hierarchy>,
    /// add output to environment variable
    #[clap(long, default_value = "NEXTSV_BUMP")]
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

    match (args.number, args.no_bump) {
        (false, false) => log::info!("Calculating the next version level"),
        (false, true) => log::info!("Calculating the next version level"),
        (true, false) => log::info!("Calculating the next version number"),
        (true, true) => log::info!("Calculating the next version number and level"),
    };

    let mut calculator_config = CalculatorConfig::new();
    calculator_config = calculator_config.set_prefix(&args.prefix);
    log::trace!("require: {:#?}", args.require);
    calculator_config = calculator_config.set_bump_report(!args.no_bump);
    calculator_config = calculator_config.set_version_report(args.number);
    if let Some(force) = args.force {
        calculator_config = calculator_config.set_force_bump(force);
    };
    if !args.require.is_empty() {
        calculator_config = calculator_config.add_required_files(args.require);
        calculator_config = calculator_config.set_required_enforcement(args.enforce);
    };
    if let Some(check_level) = args.check {
        calculator_config = calculator_config.set_reporting_threshold(check_level);
    }
    let calculator = calculator_config.build()?;

    // Set the environment variable if required
    if let Some(key) = args.set_env {
        std::env::set_var::<OsString, OsString>(key.into(), calculator.bump().into())
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
