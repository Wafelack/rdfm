mod pull;
mod proceed;
mod add;
mod setup;
use add::{add, remove};
use proceed::proceed;
use setup::*;
use pull::pull;

use std::{env, fmt::{Debug, Formatter, Result as fmtRes}};

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() < 1 {
        return Err(
            error!("Invalid arguments. Usage: dtm <COMMAND> [OPTIONS]")
        )
    }

    match args[0].as_str() {
        "setup" => setup(),
        "help" | "-h" | "--help" => help(),
        "version" | "-v" | "--version" => {
            println!("{} {}", env!("CARGO_PKG_NAME"),env!("CARGO_PKG_VERSION")); 
            Ok(())
        },
        "add" => if args.len() == 3 {
            add(&args[1], &args[2])
        } else {
            Err(
                error!("Invalid arguments. Usage: dtm <COMMAND> [OPTIONS]")
            )
        }
        "remove" => if args.len() == 2 {
            remove(&args[1])
        } else {
            Err(
                error!("Invalid arguments. Usage: dtm <COMMAND> [OPTIONS]")
            )
        }
        "pull" => if args.len() == 2 {
            pull(&args[1])
        } else {
            Err(
                error!("Invalid arguments. Usage: dtm <COMMAND> [OPTIONS]")
            )
        }
        "proceed" => proceed(),
        _ => Err(
            error!("Invalid command, type `dtm help` for help")
        )
    }

}

fn help() -> Result<()> {
    
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_AUTHORS"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    
    // Usage
    println!("\nUSAGE:");
    println!("\n{} <COMMAND> [OPTIONS]",env!("CARGO_PKG_NAME"));

    // Flags
    println!("\nFLAGS:");
    println!("\t-h, --help   \tDisplays this message.");
    println!("\t-v, --version\tDisplays version information.");

    // Commands
    println!("\nCOMMANDS:");
    println!("\thelp           \tDisplays this message.");
    println!("\tversion        \tDisplays version information.");
    println!("\tsetup          \tSetups dtm (creating ~/.dotfiles and ~/.dotfiles/dotfiles.dtm).");
    println!("\tadd $src $dest \tAdds $dest pointing to $src to the dotfiles.");
    println!("\tremove $entry  \tRemoves all lines containing $entry.");
    println!("\tproceed        \tLinks the files into ~/.dotfiles/.");
    println!("\tpull $repo_link\tPulls the specified repo to ~/.dotfiles/.");

    println!();
    Ok(())
}

pub struct DtmError {
    pub message: String,
}

impl Debug for DtmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtRes {
        write!(f, "{}.", self.message)
    }
}

impl From<env::VarError> for DtmError {
    fn from(_: env::VarError) -> Self {
        error!("environment variable `HOME` not found.") 
        // I hardcode the variable name because it is the only time
        // I'll use env::var, I'll adjust if necesary.
    }
}
impl From<fs_extra::error::Error> for DtmError {
    fn from(e: fs_extra::error::Error) -> Self {
        error!(e)
    }
}

impl From<std::io::Error> for DtmError {
    fn from(e: std::io::Error) -> Self {
        error!(e) 
        // I hardcode the variable name because it is the only time
        // I'll use env::var, I'll adjust if necesary.
    }
}

impl From<git2::Error> for DtmError {
    fn from(e: git2::Error) -> Self {
        error!(e)
    }
}

#[macro_export]
macro_rules! error {
    ($msg:tt) => {
        DtmError {
            message: $msg.to_string(),
        }
    };
}

pub type Result<T> = std::result::Result<T, DtmError>;