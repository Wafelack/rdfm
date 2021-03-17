use rdfm_lib::{
    Result,
    setup::setup,
    link::link,
};
use clap::{App, SubCommand};


fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about("manage dotfiles")
                        .subcommand(SubCommand::with_name("setup")
                                    .about("Setups the dotfiles folder."))
                        .subcommand(SubCommand::with_name("link")
                                    .about("Links the files to the dotfiles folder."))
                        .get_matches();


    if let Some(_) = matches.subcommand_matches("setup") {
        setup()?;
    } else if let Some(_) = matches.subcommand_matches("link") {
        link()?;
    }

    Ok(())
}
