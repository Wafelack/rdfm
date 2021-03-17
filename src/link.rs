use std::{path::Path, fs};
use crate::{get_files, Result, get_dotfiles_path, ok, warn, is_even, copy_dir};

pub fn link() -> Result<()> {
    let couples = get_files::get_files()?;

    for (from, to) in couples {

        let to = format!("{}/{}", get_dotfiles_path(), to);

        if is_even(&from, &to)? {
            ok!(from, " and ", to, " won't be updated.")
        } else {

            if Path::new(&to).is_dir() {
                fs::remove_dir_all(&to)?;
            } else if Path::new(&to).is_file() {
                fs::remove_file(&to)?;
            }

            if Path::new(&from).is_dir() {
                copy_dir(&from, &to)?;    
            } else if Path::new(&from).is_file() {
                fs::copy(&from, &to)?;
            } else {
                warn!(from, " does not exist. Skipping entry.");
                continue;
            }

            ok!("Sucessfully linked ", from, " to ", to, ".");
        }
    }  

    Ok(())
}