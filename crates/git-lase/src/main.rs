use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-lase", bin_name = "git lase")]
#[command(about = "A git multiplexer - sync repos across multiple providers")]
#[command(version)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  /// Initialize git-lase config
  Init,

  /// Manage git providers
  #[command(subcommand)]
  Provider(ProviderCommand),

  /// Manage tracked repos
  #[command(subcommand)]
  Repo(RepoCommand),

  /// Push all tracked repos to all their remotes
  Push,

  /// Pull all tracked repos from all their remotes
  Pull,

}

#[derive(Subcommand)]
enum ProviderCommand {
  /// Add a git provider
  Add {
    /// Provider name (e.g. github, gitlab, gitea)
    name: String,
    /// Base URL (e.g. https://github.com, https://gitlab.com)
    url: String,
    /// Username or organization on this provider
    user: String,
    /// Auth token
    #[arg(long)]
    token: Option<String>,
  },

  /// List configured providers
  List,

  /// Remove a provider
  Remove {
    /// Provider name to remove
    name: String,
  },
}

#[derive(Subcommand)]
enum RepoCommand {
  /// Add a repo to track
  Add {
    /// Path to the local git repo (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,
    /// Only track against specific providers (defaults to all)
    #[arg(long, value_delimiter = ',')]
    providers: Vec<String>,
  },

  /// Remove a tracked repo
  Rm {
    /// Path to the local git repo (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,
  },

  /// List all tracked repos and their remotes
  List,

  /// Show git status of all tracked repos
  Status,
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Command::Init => todo!(),
    Command::Provider(cmd) => match cmd {
      ProviderCommand::Add { .. } => todo!(),
      ProviderCommand::List => todo!(),
      ProviderCommand::Remove { .. } => todo!(),
    },
    Command::Repo(cmd) => match cmd {
      RepoCommand::Add { .. } => todo!(),
      RepoCommand::Rm { .. } => todo!(),
      RepoCommand::List => todo!(),
      RepoCommand::Status => todo!(),
    },
    Command::Push => todo!(),
    Command::Pull => todo!(),
  }
}
