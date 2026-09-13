use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "commslang",
    version,
    about = "Communication Systems Programming Language",
    long_about = "CommsLang — a programming language and simulation environment for communication systems."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new CommsLang project
    New {
        /// Name of the project
        name: String,
    },

    /// Run a CommsLang program
    Run {
        /// CommsLang source file
        file: Option<PathBuf>,
    },

    /// Check a CommsLang program without running it
    Check {
        /// CommsLang source file
        file: Option<PathBuf>,
    },

    /// Build a CommsLang program and generate IR
    Build {
        /// CommsLang source file
        file: Option<PathBuf>,
    },

    /// Show the CommsLang version
    Version,
}
