mod add;
mod proceed;
mod pull;
mod setup;
use add::{add, remove};
use proceed::{proceed, rev_proceed};
use pull::pull;
use setup::*;
mod errors;
pub use errors::{RdfmError, Result};

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() < 1 {
        return Err(error!("Invalid arguments. Usage: rdfm <COMMAND> [OPTIONS]"));
    }

    match args[0].as_str() {
        "setup" => setup(),
        "help" | "-h" | "--help" => help(),
        "version" | "-v" | "--version" => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        "add" => {
            if args.len() == 3 {
                add(&args[1], Some(&args[2]))
            } else if args.len() == 2 {
                add(&args[1], None)
            } else {
                Err(error!("Invalid arguments. Usage: rdfm <COMMAND> [OPTIONS]"))
            }
        }
        "remove" => {
            if args.len() == 2 {
                remove(&args[1])
            } else {
                Err(error!("Invalid arguments. Usage: rdfm <COMMAND> [OPTIONS]"))
            }
        }
        "pull" => {
            if args.len() == 2 {
                pull(&args[1])
            } else {
                Err(error!("Invalid arguments. Usage: rdfm <COMMAND> [OPTIONS]"))
            }
        }
        "rev-proceed" => rev_proceed(),
        "proceed" => proceed(),
        _ => Err(error!("Invalid command, type `rdfm help` for help")),
    }
}

fn help() -> Result<()> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_AUTHORS"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));

    // Usage
    println!("\nUSAGE:");
    println!("\n{} <COMMAND> [OPTIONS]", env!("CARGO_PKG_NAME"));

    // Flags
    println!("\nFLAGS:");
    println!("\t-h, --help   \tDisplays this message.");
    println!("\t-v, --version\tDisplays version information.");

    // Commands
    println!("\nCOMMANDS:");
    println!("\thelp           \tDisplays this message.");
    println!("\tversion        \tDisplays version information.");
    println!(
        "\tsetup          \tSetups rdfm (creating ~/.dotfiles and ~/.dotfiles/dotfiles.rdfm)."
    );
    println!("\tadd $src $dest \tAdds $dest pointing to $src to the dotfiles.");
    println!("\tremove $entry  \tRemoves all lines containing $entry.");
    println!("\tproceed        \tLinks the files into ~/.dotfiles/.");
    println!("\trev-proceed    \tReversely links the files from ~/.dotfiles/ to their real path.");
    println!("\tpull $repo_link\tPulls the specified repo to ~/.dotfiles/.");

    println!();
    Ok(())
}
