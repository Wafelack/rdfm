use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
};

use fs::File;

use crate::{setup::setup, Result};

pub fn add(src: &str, dest: &str) -> Result<()> {
    let dotfiles_path = format!("{}/.dotfiles/dotfiles.rdfm", env::var("HOME")?);

    setup()?;

    let mut dotfiles = OpenOptions::new().append(true).open(dotfiles_path)?;

    dotfiles.write_all(
        format!(
            "{}->{}\r\n",
            src,
            format!("{}/.dotfiles/{}", env::var("HOME")?, dest)
        )
        .as_bytes(),
    )?;
    println!("Successfully added `{}` to dotfiles as `{}`", src, dest);

    Ok(())
}

pub fn remove(value: &str) -> Result<()> {
    let dotfiles_path = format!("{}/.dotfiles/dotfiles.rdfm", env::var("HOME")?);

    setup()?;

    let mut content = fs::read_to_string(&dotfiles_path)?;
    let mut dotfiles = File::create(&dotfiles_path)?;

    content = content
        .lines()
        .filter(|l| !l.contains(value))
        .collect::<Vec<&str>>().join("\r\n");

    dotfiles.write_all(content.as_bytes())?;

    Ok(())
}
