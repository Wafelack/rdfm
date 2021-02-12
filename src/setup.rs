use fs::File;

use crate::Result;
use std::env;
use std::{fs, io::Write, path::Path};

pub fn setup() -> Result<()> {
    let home = env::var("HOME")?;
    let folder = format!("{}/.dotfiles", home);

    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)?;
    }

    if !Path::new(&format!("{}/dotfiles.rdfm", folder)).exists() {
        let mut f = File::create(&format!("{}/dotfiles.rdfm", folder))?;
        f.write_all(
            "# This file is created by rdfm and is not intended for manual editing.\r\n".as_bytes(),
        )?;
    }

    Ok(())
}
