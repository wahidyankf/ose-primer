use clap::{Parser, Subcommand};

use crate::commands::{docs, speccoverage, testcoverage};
use crate::internal::cliout::OutputFormat;

#[derive(Parser, Debug)]
#[command(
    name = "rhino-cli",
    version = "0.1.0",
    about = "CLI tools for repository management",
    long_about = "Command-line tools for repository management and automation.",
    disable_help_flag = true
)]
pub struct Cli {
    #[arg(
        long,
        short = 'v',
        global = true,
        help = "verbose output with timestamps"
    )]
    pub verbose: bool,

    #[arg(long, short = 'q', global = true, help = "quiet mode (errors only)")]
    pub quiet: bool,

    #[arg(
        long,
        short = 'o',
        global = true,
        default_value = "text",
        help = "output format: text, json, markdown"
    )]
    pub output: String,

    #[arg(long = "no-color", global = true, help = "disable colored output")]
    pub no_color: bool,

    #[arg(
        long,
        global = true,
        default_value = "",
        help = "echo a message to stdout"
    )]
    pub say: String,

    #[arg(long, short = 'h', global = true, help = "Print help")]
    pub help: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Top-level command namespaces.
///
/// Phase 3 registers `test-coverage` and `spec-coverage`. The remaining
/// namespaces from the Go CLI (agents, contracts, docs, doctor, env, git, java,
/// repo-governance, workflows) are added in later phases.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Test coverage commands.
    #[command(name = "test-coverage", subcommand)]
    TestCoverage(TestCoverageCommands),
    /// BDD spec coverage commands.
    #[command(name = "spec-coverage", subcommand)]
    SpecCoverage(SpecCoverageCommands),
    /// Documentation validation commands.
    #[command(name = "docs", subcommand)]
    Docs(DocsCommands),
}

#[derive(Subcommand, Debug)]
pub enum DocsCommands {
    /// Validate markdown links in the repository.
    #[command(name = "validate-links")]
    ValidateLinks(docs::ValidateLinksArgs),
    /// Validate Mermaid flowchart diagrams in markdown files.
    #[command(name = "validate-mermaid")]
    ValidateMermaid(docs::ValidateMermaidArgs),
}

#[derive(Subcommand, Debug)]
pub enum TestCoverageCommands {
    /// Check test coverage against a threshold (line-coverage 3-state algorithm).
    Validate(testcoverage::ValidateArgs),
    /// Show coverage for changed lines only (diff coverage).
    Diff(testcoverage::DiffArgs),
    /// Merge multiple coverage files into one LCOV output.
    Merge(testcoverage::MergeArgs),
}

#[derive(Subcommand, Debug)]
pub enum SpecCoverageCommands {
    /// Validate that all BDD spec files have matching test implementations.
    Validate(speccoverage::ValidateArgs),
}

pub fn run() -> i32 {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            e.print().ok();
            return 2;
        }
    };

    let output_format = match OutputFormat::parse(&cli.output) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error: {err}");
            return 1;
        }
    };

    if cli.help {
        return print_help_and_exit();
    }

    if let Some(cmd) = &cli.command {
        return dispatch(cmd, output_format, cli.verbose, cli.quiet);
    }

    if !cli.say.is_empty() {
        println!("{}", cli.say);
        return 0;
    }

    print_help_and_exit()
}

fn dispatch(cmd: &Commands, output_format: OutputFormat, verbose: bool, quiet: bool) -> i32 {
    // Each arm returns the run result paired with the command's cobra-style usage
    // block, which is printed to stderr on error to mirror cobra's default
    // (SilenceUsage is unset on the Go commands; only SilenceErrors is set).
    let (result, usage) = match cmd {
        Commands::TestCoverage(tc) => match tc {
            TestCoverageCommands::Validate(args) => (
                testcoverage::run_validate(args, output_format, verbose, quiet),
                testcoverage::VALIDATE_USAGE,
            ),
            TestCoverageCommands::Diff(args) => (
                testcoverage::run_diff(args, output_format, verbose, quiet),
                testcoverage::DIFF_USAGE,
            ),
            TestCoverageCommands::Merge(args) => (
                testcoverage::run_merge(args, output_format, verbose, quiet),
                testcoverage::MERGE_USAGE,
            ),
        },
        Commands::SpecCoverage(sc) => match sc {
            SpecCoverageCommands::Validate(args) => (
                speccoverage::run_validate(args, output_format, verbose, quiet),
                speccoverage::VALIDATE_USAGE,
            ),
        },
        Commands::Docs(dc) => match dc {
            DocsCommands::ValidateLinks(args) => (
                docs::run_validate_links(args, output_format, verbose, quiet),
                docs::VALIDATE_LINKS_USAGE,
            ),
            DocsCommands::ValidateMermaid(args) => (
                docs::run_validate_mermaid(args, output_format, verbose, quiet),
                docs::VALIDATE_MERMAID_USAGE,
            ),
        },
    };
    match result {
        Ok(()) => 0,
        Err(e) => {
            // `{e:#}` renders the full anyhow context chain joined with ": ",
            // matching Go's `%w` error wrapping.
            eprint!("{usage}");
            eprintln!("Error: {e:#}");
            1
        }
    }
}

fn print_help_and_exit() -> i32 {
    let mut cmd = <Cli as clap::CommandFactory>::command();
    cmd.print_help().ok();
    println!();
    0
}
