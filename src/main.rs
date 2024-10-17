use std::ffi::OsString;

use clap::{Parser, Subcommand};
use nextsv::{CalculatorConfig, ForceBump, Hierarchy};
use proc_exit::{Code, ExitResult};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    /// Force the calculation of the version number
    #[command(subcommand)]
    command: Commands,
    /// Do not report version bump
    #[arg(short = 'b', long)]
    no_bump: bool,
    /// Report the version number
    #[arg(short = 'n', long)]
    number: bool,

    /// Check level meets minimum for setting
    ///
    /// This option can be used to check the calculated level
    /// meets a minimum before applying an update. Bump is reported
    /// as "none" if the required level is not met.
    #[clap(short, long)]
    check: Option<Hierarchy>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "calculate", about = "Calculate the next version number")]
    Calculate(Calculate),
    #[clap(name = "force", about = "Force the bump level")]
    Force(Force),
    #[clap(
        name = "require",
        about = "Require the listed files to be updated before making a release with the specified change level"
    )]
    Require(Require),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Calculate {
    /// Prefix string to identify version number tags
    #[arg(short, long, value_parser, default_value = "v")]
    prefix: String,
    /// Filter to commits in the specified sub directory only
    #[arg(short, long)]
    subdir: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Force {
    /// Force the calculation to the bump value
    #[command(subcommand)]
    bump: ForceBump,
    /// First flag to set first version and pre-release in the same transaction
    #[arg(short, long)]
    first: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Require {
    #[command(subcommand)]
    enforce: Hierarchy,
    #[arg(short, long)]
    files: Vec<OsString>,
    /// Prefix string to identify version number tags
    #[arg(short, long, value_parser, default_value = "v")]
    prefix: String,
    /// Filter to commits in the specified sub directory only
    #[arg(short, long)]
    subdir: Option<String>,
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
    calculator_config = calculator_config.set_bump_report(!args.no_bump);
    calculator_config = calculator_config.set_version_report(args.number);

    if let Some(check_level) = args.check {
        calculator_config = calculator_config.set_reporting_threshold(check_level);
    };

    match args.command {
        Commands::Force(args) => {
            calculator_config = calculator_config.set_force_bump(args.bump);
            if args.first {
                log::debug!("Setting first version and pre-release in the same transaction");
                calculator_config = calculator_config.set_first_version();
            };
        }
        Commands::Calculate(args) => {
            calculator_config = calculator_config.set_prefix(&args.prefix);
        }
        Commands::Require(args) => {
            calculator_config = calculator_config.set_prefix(&args.prefix);
            calculator_config = calculator_config.add_required_files(args.files);
            calculator_config = calculator_config.set_required_enforcement(args.enforce);
        }
    };

    let calculator = calculator_config.build()?;

    println!("{}", calculator.report());

    Code::SUCCESS.ok()
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
