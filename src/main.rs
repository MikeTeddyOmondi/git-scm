// Import our repository and other modules
mod git;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use git::repository::Repository;

#[derive(Parser)]
#[command(name = "git-scm")]
#[command(about = "Git-SCM - A Git-like version control tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Stage files for commit
    Add {
        /// Files or directories to stage
        paths: Vec<PathBuf>,
    },

    /// Commit staged changes
    // Commit {
    //     /// Commit message
    //     #[arg(short, long)]
    //     message: String,
    //     /// Author name
    //     #[arg(short, long, default_value = "Anonymous")]
    //     author: String,
    // },
    Commit(Commit),

    /// Create a new branch
    Branch {
        /// Name of the new branch
        name: String,
    },

    /// Switch to a different branch
    Checkout {
        /// Name of the branch to switch to
        branch: String,
    },

    /// Show differences between branches
    Diff {
        /// Source branch
        #[arg(default_value = "main")]
        branch1: String,

        /// Target branch
        #[arg(default_value = "")]
        branch2: Option<String>,
    },

    /// Merge branches
    Merge {
        /// Base branch name
        base: String,

        /// Source branch
        source: String,

        /// Target branch (defaults to current branch)
        #[arg(default_value = "")]
        target: Option<String>,
    },

    /// Show commit history
    Log,
}

#[derive(Debug, Args)]
pub struct Commit {
    #[clap(subcommand)]
    pub command: CommitCommands,
}

#[derive(Debug, Subcommand)]
pub enum CommitCommands {
    /// Add Commit Message
    Message(CommitCommandMessage),
    /// Commit History
    History,
}

#[derive(Debug, Args)]
pub struct CommitCommandMessage {
    /// Commit message
    #[arg(short, long)]
    message: String,
    /// Author name
    #[arg(short, long, default_value = "Anonymous")]
    author: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize the repository once, or create a dummy repository for `Init` command
    let mut repo = match &cli.command {
        Commands::Init { .. } => None, // No repository exists yet; Init will create one
        _ => Some(Repository::init(".").context("Failed to open or initialize repository")?),
    };

    match &cli.command {
        Commands::Init { path } => {
            let _repo = Repository::init(path).context("Failed to initialize repository")?;
            println!("Initialized empty Git repository in {}", path.display());
            Ok(())
        }

        Commands::Add { paths } => {
            // let mut repo = Repository::init(".")?;
            let repo= repo.as_mut().expect("Repository must be initialized");
            for path in paths {
                repo.add(path)
                    .with_context(|| format!("Failed to add {}", path.display()))?;
            }
            println!("Files staged successfully");
            Ok(())
        }

        Commands::Commit(Commit { command }) => match command {
            CommitCommands::Message(CommitCommandMessage { message, author }) => {
                // let mut repo = Repository::init(".")?;
                let repo= repo.as_mut().expect("Repository must be initialized");
                let commit_id = repo
                    .commit(message.clone(), author.clone())
                    .context("Failed to create commit")?;
                println!("Commit created: {}", commit_id);
                Ok(())
            }
            CommitCommands::History => {
                // let repo = Repository::init(".")?;
                let repo= repo.as_mut().expect("Repository must be initialized");
                repo.log();
                Ok(())
            }
        },

        Commands::Branch { name } => {
            // let mut repo = Repository::init(".")?;
            let repo= repo.as_mut().expect("Repository must be initialized");
            repo.create_branch(name.clone())
                .context("Failed to create branch")?;
            println!("Created branch: {}", name);
            Ok(())
        }

        Commands::Checkout { branch } => {
            // let mut repo = Repository::init(".")?;
            let repo= repo.as_mut().expect("Repository must be initialized");
            repo.switch_branch(&branch)
                .context("Failed to switch branch")?;
            println!("Switched to branch: {}", branch);
            Ok(())
        }

        Commands::Diff { branch1, branch2 } => {
            // let repo = Repository::init(".")?;
            let repo= repo.as_mut().expect("Repository must be initialized");

            // If no second branch specified, use current branch
            let branch2 = branch2
                .clone()
                .unwrap_or_else(|| repo.current_branch().to_string());

            let diffs = repo
                .diff(branch1, &branch2)
                .context("Failed to generate diff")?;

            if diffs.is_empty() {
                println!("No differences found between branches");
            } else {
                println!("Differences between {} and {}:", branch1, branch2);
                for diff in diffs {
                    println!("{}", git::diff::Differ::render_diff(&diff));
                }
            }
            Ok(())
        }

        Commands::Merge {
            base,
            source,
            target,
        } => {
            // let mut repo = Repository::init(".")?;
            let repo= repo.as_mut().expect("Repository must be initialized");

            // If no target specified, use current branch
            let target = target
                .clone()
                .unwrap_or_else(|| repo.current_branch().to_string());

            match repo.merge(&base, &source, &target) {
                Ok(_) => {
                    println!(
                        "Successfully merged {} and {} into {}",
                        source, target, base
                    );
                    Ok(())
                }
                Err(merge_conflicts) => {
                    println!("Merge conflicts detected:");
                    println!("{:?}", merge_conflicts);
                    Err(anyhow::anyhow!("Merge failed due to conflicts"))
                }
            }
        }

        Commands::Log => {
            // let repo = Repository::init(".")?;
            let repo= repo.as_mut().expect("Repository must be initialized");

            // Implement a proper log display
            println!("Commit History:");
            for (branch_name, branch) in repo.branches() {
                println!("Branch: {}", branch_name);
                for commit_id in branch.commits() {
                    // TODO: fetch and display more commit details
                    println!("  Commit: {}", commit_id);
                }
            }
            Ok(())
        }
    }
}
