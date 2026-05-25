use clap::{Parser, Subcommand};

use crate::commands::{
    agents, contracts, docs, doctor, env, git, java, repo_governance, speccoverage, testcoverage,
    workflows,
};
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
    /// Agent configuration commands.
    #[command(name = "agents", subcommand)]
    Agents(AgentsCommands),
    /// Repository governance validation commands.
    #[command(name = "repo-governance", subcommand)]
    RepoGovernance(RepoGovernanceCommands),
    /// Workflow definition commands.
    #[command(name = "workflows", subcommand)]
    Workflows(WorkflowsCommands),
    /// Git workflow commands.
    #[command(name = "git", subcommand)]
    Git(GitCommands),
    /// Contract codegen post-processing commands.
    #[command(name = "contracts", subcommand)]
    Contracts(ContractsCommands),
    /// Java validation commands.
    #[command(name = "java", subcommand)]
    Java(JavaCommands),
    /// Environment file backup and restore commands.
    #[command(name = "env", subcommand)]
    Env(EnvCommands),
    /// Check required tool versions are installed and correct.
    #[command(name = "doctor")]
    Doctor(doctor::DoctorArgs),
}

#[derive(Subcommand, Debug)]
pub enum EnvCommands {
    /// Create .env files from .env.example templates.
    Init(env::EnvInitArgs),
    /// Back up .env files from the repository.
    Backup(env::EnvBackupArgs),
    /// Restore .env files from a backup.
    Restore(env::EnvRestoreArgs),
}

#[derive(Subcommand, Debug)]
pub enum GitCommands {
    /// Run all pre-commit checks (config, lint, format, docs).
    #[command(name = "pre-commit")]
    PreCommit,
}

#[derive(Subcommand, Debug)]
pub enum ContractsCommands {
    /// Remove unused and same-package imports from generated Java files.
    #[command(name = "java-clean-imports")]
    JavaCleanImports(contracts::JavaCleanImportsArgs),
    /// Create Dart package scaffolding for generated contracts.
    #[command(name = "dart-scaffold")]
    DartScaffold(contracts::DartScaffoldArgs),
}

#[derive(Subcommand, Debug)]
pub enum JavaCommands {
    /// Validate Java packages have required null-safety annotations.
    #[command(name = "validate-annotations")]
    ValidateAnnotations(java::ValidateAnnotationsArgs),
}

#[derive(Subcommand, Debug)]
pub enum RepoGovernanceCommands {
    /// Scan governance markdown files for forbidden vendor-specific terms.
    #[command(name = "vendor-audit")]
    VendorAudit(repo_governance::VendorAuditArgs),
}

#[derive(Subcommand, Debug)]
pub enum WorkflowsCommands {
    /// Validate workflow filename suffixes and frontmatter name consistency.
    #[command(name = "validate-naming")]
    ValidateNaming,
}

#[derive(Subcommand, Debug)]
pub enum AgentsCommands {
    /// Sync Claude Code agents to OpenCode format.
    Sync(agents::SyncArgs),
    /// Validate Claude Code agent and skill format in .claude/ directory.
    #[command(name = "validate-claude")]
    ValidateClaude(agents::ValidateClaudeArgs),
    /// Validate that .claude/ and .opencode/ are in sync.
    #[command(name = "validate-sync")]
    ValidateSync,
    /// Validate agent filename suffixes and frontmatter name consistency.
    #[command(name = "validate-naming")]
    ValidateNaming,
    /// Emit the generated vendor binding files.
    #[command(name = "emit-bindings")]
    EmitBindings(agents::EmitBindingsArgs),
    /// Validate the generated vendor binding files against their source of truth.
    #[command(name = "validate-bindings")]
    ValidateBindings,
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
        Commands::TestCoverage(tc) => dispatch_test_coverage(tc, output_format, verbose, quiet),
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
        Commands::Agents(ac) => dispatch_agents(ac, output_format, verbose, quiet),
        Commands::RepoGovernance(rg) => match rg {
            RepoGovernanceCommands::VendorAudit(args) => (
                repo_governance::run_vendor_audit(args, output_format),
                repo_governance::VENDOR_AUDIT_USAGE,
            ),
        },
        Commands::Workflows(wf) => match wf {
            WorkflowsCommands::ValidateNaming => (
                workflows::run_validate_naming(output_format, verbose, quiet),
                workflows::VALIDATE_NAMING_USAGE,
            ),
        },
        Commands::Git(gc) => match gc {
            GitCommands::PreCommit => (git::run_pre_commit(), git::PRE_COMMIT_USAGE),
        },
        Commands::Contracts(cc) => match cc {
            ContractsCommands::JavaCleanImports(args) => (
                contracts::run_java_clean_imports(args, output_format, verbose, quiet),
                contracts::JAVA_CLEAN_IMPORTS_USAGE,
            ),
            ContractsCommands::DartScaffold(args) => (
                contracts::run_dart_scaffold(args, output_format, verbose, quiet),
                contracts::DART_SCAFFOLD_USAGE,
            ),
        },
        Commands::Java(jc) => match jc {
            JavaCommands::ValidateAnnotations(args) => (
                java::run_validate_annotations(args, output_format, verbose, quiet),
                java::VALIDATE_ANNOTATIONS_USAGE,
            ),
        },
        Commands::Env(ec) => dispatch_env(ec, output_format, verbose, quiet),
        Commands::Doctor(args) => (
            doctor::run_doctor(args, output_format, verbose, quiet),
            doctor::DOCTOR_USAGE,
        ),
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

fn dispatch_test_coverage(
    tc: &TestCoverageCommands,
    output_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> (anyhow::Result<()>, &'static str) {
    match tc {
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
    }
}

fn dispatch_agents(
    ac: &AgentsCommands,
    output_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> (anyhow::Result<()>, &'static str) {
    match ac {
        AgentsCommands::Sync(args) => (
            agents::run_sync(args, output_format, verbose, quiet),
            agents::SYNC_USAGE,
        ),
        AgentsCommands::ValidateClaude(args) => (
            agents::run_validate_claude(args, output_format, verbose, quiet),
            agents::VALIDATE_CLAUDE_USAGE,
        ),
        AgentsCommands::ValidateSync => (
            agents::run_validate_sync(output_format, verbose, quiet),
            agents::VALIDATE_SYNC_USAGE,
        ),
        AgentsCommands::ValidateNaming => (
            agents::run_validate_naming(output_format, verbose, quiet),
            agents::VALIDATE_NAMING_USAGE,
        ),
        AgentsCommands::EmitBindings(args) => {
            (agents::run_emit_bindings(args), agents::EMIT_BINDINGS_USAGE)
        }
        AgentsCommands::ValidateBindings => (
            agents::run_validate_bindings(),
            agents::VALIDATE_BINDINGS_USAGE,
        ),
    }
}

fn dispatch_env(
    ec: &EnvCommands,
    output_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> (anyhow::Result<()>, &'static str) {
    match ec {
        EnvCommands::Init(args) => (env::run_env_init(args), env::ENV_INIT_USAGE),
        EnvCommands::Backup(args) => (
            env::run_env_backup(args, output_format, verbose, quiet),
            env::ENV_BACKUP_USAGE,
        ),
        EnvCommands::Restore(args) => (
            env::run_env_restore(args, output_format, verbose, quiet),
            env::ENV_RESTORE_USAGE,
        ),
    }
}

fn print_help_and_exit() -> i32 {
    let mut cmd = <Cli as clap::CommandFactory>::command();
    cmd.print_help().ok();
    println!();
    0
}
