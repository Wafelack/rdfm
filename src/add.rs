use fs::File;
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
};

use crate::{
    setup::{get_dotfiles_folder, setup},
    Result,
};

pub fn add(src: &str, dest: Option<&str>) -> Result<()> {
    let dotfiles_path = format!("{}/dotfiles.rdfm", get_dotfiles_folder()?);

    setup()?;

    let mut dotfiles = OpenOptions::new().append(true).open(dotfiles_path)?;

    let write_dest = if dest.is_some() {
        format!("{}{}", get_dotfiles_folder()?, dest.unwrap())
    } else {
        format!(
            "{}{}",
            get_dotfiles_folder()?,
            src.replace(&env::var("HOME")?, "")
        )
    };

    dotfiles.write_all(format!("{}->{}\r\n", src, write_dest).as_bytes())?;

    println!(
        "Successfully added `{}` to dotfiles as `{}`",
        src, write_dest
    );

    Ok(())
}

pub fn remove(value: &str) -> Result<()> {
    let dotfiles_path = format!("{}/dotfiles.rdfm", get_dotfiles_folder()?);

    setup()?;

    let mut content = fs::read_to_string(&dotfiles_path)?;
    let mut dotfiles = File::create(&dotfiles_path)?;

    content = content
        .lines()
        .filter(|l| !l.contains(value))
        .collect::<Vec<&str>>()
        .join("\r\n");

    dotfiles.write_all(content.as_bytes())?;

    Ok(())
}
