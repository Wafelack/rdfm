use crate::{Result, setup::get_dotfiles_folder};
use git2::Repository;
use std::{env, fs, path::Path};

pub fn pull(repo: &str) -> Result<()> {
    let dotfiles_folder = &get_dotfiles_folder()?;

    if Path::new(dotfiles_folder).exists() {
        fs::remove_dir_all(dotfiles_folder)?;
        fs::create_dir_all(dotfiles_folder)?;
    }

    Repository::clone(repo, dotfiles_folder)?;

    println!("Successfully pulled `{}` to `{}`", repo, dotfiles_folder);

    Ok(())
}
