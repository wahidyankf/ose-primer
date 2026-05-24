use clap::{Parser, Subcommand};

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
/// Phase 2 skeleton: intentionally empty. The eleven namespaces from the Go CLI
/// (agents, contracts, docs, doctor, env, git, java, repo-governance,
/// spec-coverage, test-coverage, workflows) are added in later phases.
#[derive(Subcommand, Debug)]
pub enum Commands {}

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
        return dispatch(cmd, output_format);
    }

    if !cli.say.is_empty() {
        println!("{}", cli.say);
        return 0;
    }

    print_help_and_exit()
}

fn dispatch(cmd: &Commands, _output_format: OutputFormat) -> i32 {
    // No namespaces are registered in the Phase 2 skeleton, so `Commands` has no
    // variants and this match is exhaustive over zero cases. Later phases add the
    // per-namespace dispatch arms.
    match *cmd {}
}

fn print_help_and_exit() -> i32 {
    let mut cmd = <Cli as clap::CommandFactory>::command();
    cmd.print_help().ok();
    println!();
    0
}
