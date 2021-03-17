use crate::*;
use std::{fs, path::Path};

pub fn setup() -> Result<()> {
    let path = get_dotfiles_path();

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(())
}
