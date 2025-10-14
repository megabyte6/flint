use std::{fmt::Display, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Create a new server
    New {
        /// The name of the new server
        server: PathBuf,

        /// The type of server software to use
        #[arg(default_value_t = ServerTypes::Vanilla)]
        kind: ServerTypes,

        /// The Minecraft version to install
        #[arg(default_value_t = String::from("latest"))]
        mc_version: String,
    },

    /// Update existing server
    Update {
        /// The name of the server to update
        server: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum ServerTypes {
    /// A vanilla server by Mojang
    Vanilla,
}

impl Display for ServerTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerTypes::Vanilla => write!(f, "vanilla"),
        }
    }
}
