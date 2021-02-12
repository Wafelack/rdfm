use crate::Result;
use git2::Repository;
use std::{env, fs, path::Path};

pub fn pull(repo: &str) -> Result<()> {
    let dotfiles_folder = &format!("{}/.dotfiles/", env::var("HOME")?);

    if Path::new(dotfiles_folder).exists() {
        fs::remove_dir_all(dotfiles_folder)?;
        fs::create_dir_all(dotfiles_folder)?;
    }

    Repository::clone(repo, dotfiles_folder)?;

    println!("Successfully pulled `{}` to `{}`", repo, dotfiles_folder);

    Ok(())
}
