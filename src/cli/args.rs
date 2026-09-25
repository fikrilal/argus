use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Argus — The All-Seeing Pre-Flight QA Swarm
#[derive(Debug, Parser)]
#[command(
    name = "argus",
    author = "ahmad fikril <fikrildev@gmail.com>",
    version,
    about = "The all-seeing pre-flight QA swarm. Catches edge cases, state invariant leaks, and spec gaps before human QA.",
    long_about = "Argus unleashes a multi-agent adversarial swarm on your git diff—cross-referencing specification documents, auditing state reversibility, checking regional regulations, and fuzzing inputs before human QA ever sees the code."
)]
pub struct Cli {
    /// Enable verbose diagnostic output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Custom path to the .argus configuration file
    #[arg(short, long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run an adversarial QA audit on your code changes
    Audit(AuditArgs),

    /// Initialize Argus configuration and starter personas in the repository
    Init(InitArgs),

    /// Inspect and list available agent personas
    Personas(PersonasArgs),

    /// Resume an active agent session interactively via Pi
    Resume(ResumeArgs),
}

/// Arguments for `argus audit`
#[derive(Debug, Args)]
pub struct AuditArgs {
    /// The squad of personas to deploy (e.g., forms, state, sync, spec, all)
    #[arg(short, long, default_value = "all")]
    pub squad: String,

    /// Target specific feature directory or file to audit
    #[arg(short = 'p', long, value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// Path to the active System Functional Design (SFD) specification file in Markdown format
    #[arg(long, value_name = "FILE")]
    pub sfd: Option<PathBuf>,

    /// The base Git branch or commit to diff against (e.g. origin/main, origin/development)
    #[arg(short, long, conflicts_with_all = ["staged", "full"])]
    pub base: Option<String>,

    /// Audit only staged changes instead of the entire working tree
    #[arg(long, conflicts_with_all = ["base", "full"])]
    pub staged: bool,

    /// Audit the entire codebase across all source files instead of only diff changes
    #[arg(long, conflicts_with_all = ["base", "staged"])]
    pub full: bool,

    /// Maximum number of subagents to execute concurrently
    #[arg(short = 'j', long, default_value = "4")]
    pub concurrency: usize,

    /// Maximum execution timeout in seconds allowed per subagent
    #[arg(long, default_value = "300")]
    pub timeout: u64,
}

/// Arguments for `argus init`
#[derive(Debug, Args)]
pub struct InitArgs {
    /// Overwrite existing .argus files if they already exist
    #[arg(short, long)]
    pub force: bool,

    /// Target repository path to initialize
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    pub target_dir: PathBuf,
}

/// Arguments for `argus personas`
#[derive(Debug, Args)]
pub struct PersonasArgs {
    #[command(subcommand)]
    pub action: PersonasSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PersonasSubcommand {
    /// List all available personas, their squads, and their origin (built-in or repo-override)
    List {
        /// Filter personas by squad name
        #[arg(short, long)]
        squad: Option<String>,
    },

    /// Display the complete prompt and metadata for a specific persona
    Show {
        /// Name of the persona to display (e.g. stock-ledger-auditor)
        name: String,
    },
}

/// Arguments for `argus resume`
#[derive(Debug, Args)]
pub struct ResumeArgs {
    /// Specific persona session to resume directly (e.g. stock-ledger-auditor)
    #[arg(short, long)]
    pub persona: Option<String>,

    /// Filter sessions by Git branch name
    #[arg(short, long)]
    pub branch: Option<String>,
}
