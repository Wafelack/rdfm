use crate::Result;
use fs::File;
use std::env;
use std::{fs, io::Write, path::Path};

pub fn setup() -> Result<()> {
    let folder = get_dotfiles_folder()?;

    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)?;
    }

    if !Path::new(&format!("{}/dotfiles.rdfm", folder)).exists() {
        let mut f = File::create(&format!("{}/dotfiles.rdfm", folder))?;
        f.write_all(
            "# This file is created by rdfm and is not intended for manual editing.\r\n".as_bytes(),
        )?;
    }

    Ok(())
}

pub fn get_dotfiles_folder() -> Result<String> {
    if env::var("XDG_CONFIG_HOME").is_ok() {
        println!("Using $XDG_CONFIG_HOME instead of $HOME/.dotfiles/ for dotfiles initialization.");
        Ok(format!("{}/.dotfiles", env::var("XDG_CONFIG_HOME")?))
    } else {
        Ok(format!("{}/.dotfiles", env::var("HOME")?))
    }
}
