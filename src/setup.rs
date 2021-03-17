use crate::{get_dotfiles_path, Result};
use std::{io::Write, fs::{self, File}, path::Path};

pub fn setup() -> Result<()> {
    let path = get_dotfiles_path();
    let cfg_path = format!("{}/dotfiles.rdfm", &path);

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }

    if !Path::new(&cfg_path).exists() {
        File::create(&cfg_path)?.write_all(b"# Write here the files that will be linked to your dotfiles folder\n# Refer to rdfm.conf(5) for help.")?;
    }

    ok!("Successfully initialized dotfiles folder in ", path, ".");

    Ok(())
}
