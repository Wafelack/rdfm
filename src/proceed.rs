use file_diff::diff;
use fs_extra::{dir::{remove},dir, file};

use crate::{Result, setup::setup, error, DtmError};
use std::{env, fs, path::Path};

fn advancement(index: usize, len: usize) -> String {
    let mut toret = "[".to_string();
    let tow = (index + 1/len) * 20;
    for _ in 0..tow {
        toret.push('=');
    }
    for _ in tow..20 {
        toret.push(' ');
    }
    toret.push_str(">] ");
    toret.push_str(&format!("{}/{}", index + 1, len));
    toret
}

pub fn proceed() -> Result<()> {
    let dotfiles_path = format!("{}/.dotfiles/dotfiles.dtm", env::var("HOME")?);

    setup()?;

    let content = fs::read_to_string(dotfiles_path)?;
    let len = content.lines().collect::<Vec<_>>().len();

    for (index, line) in content.lines().enumerate() {
        print!("\r{}", advancement(index, len));

        if line.starts_with("#") {
            continue;
        }
        let splited = line.split("->").collect::<Vec<_>>();
    
        if splited.len() != 2 {
            return Err(
                error!((format!("Invalid line format in ~/.dotfiles/dotfiles.dtm at line {} ({}).", index + 1, line)))
            )
        }
        println!("{};{}", splited[0], splited[1]);

        if diff(splited[0], splited[1]) {
            continue;
        }

        let dest = Path::new(splited[1]);
        let src = Path::new(splited[0]);

        if dest.exists() {
            if dest.is_dir() {
                remove(dest)?;
            } else {
                fs::remove_file(dest)?;
            }
        }

        if src.exists() {
            if src.is_dir() {
                fs::create_dir_all(dest)?;
                dir::copy(src, dest, &dir::CopyOptions::new())?;
                
            } else {
                fs::create_dir_all(dest.parent().unwrap())?;
                file::copy(src, dest, &file::CopyOptions::new())?;
            }
        } else {
            return Err(
                error!((format!("No filesystem element named `{}`", dest.to_str().unwrap())))
            )
        }
        
    }
    println!();

    Ok(())
}