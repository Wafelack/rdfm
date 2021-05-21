mod get_files;
mod lib;
mod link;
mod setup;
use lib::Result;
use link::link;
use setup::setup;

use clap::{App, Arg, SubCommand};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A Rusty DotFiles Manager")
        .help_message("Print help information.")
        .version_message("Print version information.")
        .arg(
            Arg::with_name("folder")
                .short("f")
                .long("folder")
                .takes_value(true)
                .help("The dotfiles folder to use instead of $HOME/.config/.dotfiles."),
        )
        .subcommand(SubCommand::with_name("setup").about("Setup the dotfiles folder."))
        .subcommand(
            SubCommand::with_name("link")
                .arg(
                    Arg::with_name("dry_run")
                        .long("dry-run")
                        .short("dr")
                        .help("Don't link, just tell what will be done."),
                )
                .about("Link the files to the dotfiles folder."),
        )
        .subcommand(
            SubCommand::with_name("install")
                .arg(
                    Arg::with_name("dry_run")
                        .long("dry-run")
                        .short("dr")
                        .help("Don't install, just tell what will be done."),
                )
                .about("Install the dotfiles to their original folders."),
        )
        .get_matches();

    let folder = matches.value_of("folder");
    if let Some(_) = matches.subcommand_matches("setup") {
        setup(folder)?;
    } else if let Some(m) = matches.subcommand_matches("link") {
        link(false, m.is_present("dry_run"), folder)?;
    } else if let Some(m) = matches.subcommand_matches("install") {
        link(true, m.is_present("dry_run"), folder)?;
    }

    Ok(())
}
