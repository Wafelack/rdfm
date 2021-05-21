use crate::{Result, lib::get_dotfiles_path, ok};
use std::{io::Write, fs::{self, File}, path::Path};

pub fn setup(folder: Option<&str>) -> Result<()> {
    let path = get_dotfiles_path(folder);
    let cfg_path = format!("{}/dotfiles.rdfm", &path);

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }

    if !Path::new(&cfg_path).exists() {
        File::create(&cfg_path)?.write_all(b"# Write here the files that will be linked to your dotfiles folder\n# Syntax: `/path/to/source/file = relative/location/in/folder`.")?;
    }

    ok!("Successfully initialized dotfiles folder in ", path, ".");

    Ok(())
}
