use crate::*;
use std::{fs, path::Path};

pub fn get_files() -> Result<Vec<(String, String)>> {
    let path = get_dotfiles_path();

    if !Path::new(&path).exists() {
        return Err(error("Dotfiles folder is not initialized."));
    }

    let file_path = format!("{}/dotfiles.rdfm", &path);

    if !Path::new(&file_path).exists() {
        return Err(error("No `dotfiles.rdfm` in the dotfiles foldr."));
    }

    let content = fs::read_to_string(file_path)?;

    let mut to_ret = vec![];

    for (idx, line) in content.lines().enumerate() {
        let splited = line.split('=').collect::<Vec<_>>();

        if splited.len() != 2 {
            warn!("Invalid syntax at line ", idx, ".");
            continue;
        }

        let from = splited[0].trim().to_string();
        let to = splited[1].trim().to_string();

        to_ret.push((from, to));
    }

    Ok(to_ret)
}
