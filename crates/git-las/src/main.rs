mod cli;
mod cmd;
mod git;
mod gitlas;
mod logger;
mod types;

use clap::Parser;

use cli::{Cli, Command, RemoteCommand, RepoCommand, RepoRemoteCommand, RepoSetCommand};

fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::Init => cmd::init::run()?,
    Command::Remote(cmd) => match cmd {
      RemoteCommand::Add {
        name,
        url,
        user,
        token,
      } => cmd::remote::add::run(name, url, user, token)?,
      RemoteCommand::Ls => cmd::remote::ls::run()?,
      RemoteCommand::Rm { name } => cmd::remote::rm::run(name)?,
    },
    Command::Repo(cmd) => match cmd {
      RepoCommand::Add { path } => cmd::repo::add::run(path)?,
      RepoCommand::Rm => cmd::repo::rm::run()?,
      RepoCommand::Ls => cmd::repo::ls::run()?,
      RepoCommand::Status => cmd::repo::status::run()?,
      RepoCommand::Remote(cmd) => match cmd {
        RepoRemoteCommand::Add {
          remote,
          repo,
          user,
          primary,
        } => cmd::repo::remote::add::run(remote, repo, user, primary)?,
        RepoRemoteCommand::Rm { remote } => cmd::repo::remote::rm::run(remote)?,
        RepoRemoteCommand::Ls => cmd::repo::remote::ls::run()?,
      },
      RepoCommand::Set(cmd) => match cmd {
        RepoSetCommand::Primary { remote, repo, user } => {
          cmd::repo::set::primary::run(remote, repo, user)?
        }
      },
    },
    Command::Push => cmd::push::run()?,
    Command::Pull => cmd::pull::run()?,
  }

  Ok(())
}
