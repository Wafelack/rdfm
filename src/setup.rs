use fs::File;

use crate::{Result};
use std::{fs, path::Path};
use std::env;


pub fn setup() -> Result<()> {

    let home = env::var("HOME")?;
    let folder = format!("{}/.dotfiles", home);

    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)?;
    }

    if !Path::new(&format!("{}/dotfiles.dtm", folder)).exists() {
        File::create(&format!("{}/dotfiles.dtm", folder))?;
    }
    
    Ok(())
}