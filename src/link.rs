use crate::{
    get_files, lib::copy_dir, lib::get_dotfiles_path, lib::is_even, non_fatal_error, ok, warn,
    Result,
};
use std::{fs, path::Path};

pub fn link(reverse: bool, dry_run: bool, folder: Option<&str>) -> Result<()> {
    let couples = get_files::get_files(folder)?;

    for (from, to) in couples {
        let mut to = format!("{}/{}", get_dotfiles_path(folder), to).replace("//", "/");
        let mut from = from;
        let temp = to.clone();

        if reverse {
            to = from.clone();
            from = temp;
        }

        if is_even(&from, &to)? {
            warn!(from, " and ", to, " won't be updated.")
        } else {
            if !dry_run {
                if Path::new(&to).is_dir() {
                    fs::remove_dir_all(&to)?;
                } else if Path::new(&to).is_file() {
                    fs::remove_file(&to)?;
                }
            }

            if Path::new(&from).is_dir() {
                if !dry_run {
                    copy_dir(&from, &to)?;
                }
            } else if Path::new(&from).is_file() {
                if !dry_run {
                    fs::copy(&from, &to)?;
                }
            } else {
                non_fatal_error!(from, " does not exist. Skipping entry.");
                continue;
            }

            ok!(
                "Sucessfully",
                {
                    if reverse {
                        " installed "
                    } else {
                        " linked "
                    }
                },
                from,
                " to ",
                to,
                "."
            );
        }
    }

    Ok(())
}
