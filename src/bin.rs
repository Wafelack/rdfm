use rdfm_lib::{
    Result,
    setup::setup,
    link::link,
};
use clap::{App, Arg, SubCommand};


fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about("manage dotfiles")
                        .subcommand(SubCommand::with_name("setup")
                                    .about("Setups the dotfiles folder."))
                        .subcommand(SubCommand::with_name("link")
                                    .arg(Arg::with_name("dry_run")
                                         .long("dry-run")
                                         .short("dr")
                                         .help("Doesn't do anything, just gives an overview."))

                                    .about("Links the files to the dotfiles folder."))
                        .subcommand(SubCommand::with_name("install")
                                    .arg(Arg::with_name("dry_run")
                                         .long("dry-run")
                                         .short("dr")
                                         .help("Doesn't do anything, just gives an overview."))
                                    .about("Install the dotfiles to their original folders."))
                        .get_matches();


    if let Some(_) = matches.subcommand_matches("setup") {
        setup()?;
    } else if let Some(m) = matches.subcommand_matches("link") {
        link(false, m.is_present("dry_run"))?;
    } else if let Some(m) = matches.subcommand_matches("install") {
        link(true, m.is_present("dry_run"))?;
    }

    Ok(())
}
