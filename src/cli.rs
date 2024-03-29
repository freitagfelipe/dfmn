use crate::commands::{self, Command};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Add(commands::Add),
    List(commands::List),
    Update(commands::Update),
    Remove(commands::Remove),
    Remote(commands::Remote),
    Reset(commands::Reset),
    Clone(commands::Clone),
    Sync(commands::Sync),
}

/// dfmn - dotfiles Manager
#[derive(Parser)]
#[command(version)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

impl Commands {
    pub fn invoke(self) {
        match self {
            Self::Add(cmd) => cmd.call(),
            Self::List(cmd) => cmd.call(),
            Self::Update(cmd) => cmd.call(),
            Self::Remove(cmd) => cmd.call(),
            Self::Remote(cmd) => cmd.call(),
            Self::Reset(cmd) => cmd.call(),
            Self::Clone(cmd) => cmd.call(),
            Self::Sync(cmd) => cmd.call(),
        }
    }
}

pub fn parse() -> CLI {
    CLI::parse()
}
