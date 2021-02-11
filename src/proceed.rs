use fs_extra::{dir::{remove},dir, file};

use crate::{Result, setup::setup, error, DtmError};
use std::{env, fs, path::Path};

pub fn proceed() -> Result<()> {
    let dotfiles_path = format!("{}/.dotfiles/dotfiles.dtm", env::var("HOME")?);

    setup()?;

    let content = fs::read_to_string(dotfiles_path)?;

    for (index, line) in content.lines().enumerate() {
        let splited = line.split("->").collect::<Vec<_>>();
    
        if splited.len() != 2 {
            return Err(
                error!((format!("Invalid line format in ~/.dotfiles/dotfiles.dtm at line {} ({}).", index + 1, line)))
            )
        }

        let dest = Path::new(splited[1]);
        let src = Path::new(splited[0]);
        if dest.exists() {
            if dest.is_dir() {
                remove(dest)?;
            } else {
                fs::remove_file(dest)?;
            }
        }

        if src.exists() {
            if src.is_dir() {
                fs::create_dir_all(dest)?;
                dir::copy(src, dest, &dir::CopyOptions::new())?;
            } else {
                file::copy(src, dest, &file::CopyOptions::new())?;
            }
        } else {
            return Err(
                error!((format!("No filesystem element named `{}`", dest.to_str().unwrap())))
            )
        }
        
    }

    Ok(())
}