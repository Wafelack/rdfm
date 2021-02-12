use fs::File;

use crate::{Result};
use std::{fs, io::Write, path::Path};
use std::env;


pub fn setup() -> Result<()> {

    let home = env::var("HOME")?;
    let folder = format!("{}/.dotfiles", home);

    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)?;
    }

    if !Path::new(&format!("{}/dotfiles.dtm", folder)).exists() {
        let mut f = File::create(&format!("{}/dotfiles.dtm", folder))?;
        f.write_all("# This file is created by dtm and is not intended for manual editing.\r\n".as_bytes())?;
    }
    
    Ok(())
}