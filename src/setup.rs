use crate::*;
use std::{fs, path::Path};

pub fn setup() -> Result<()> {

    let path = format!("{}/.config/.dotfiles", env!("HOME"));

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(())
}
