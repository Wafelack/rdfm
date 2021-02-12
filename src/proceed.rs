use fs::{File};
use fs_extra::{dir::{remove},dir, file};
use sha2::{Digest, Sha256};

use crate::{Result, setup::setup, error, DtmError};
use std::{env, ffi::OsStr, fs, io::Read, path::Path};

fn same<T>(a: T, b: T) -> Result<bool>
where T: AsRef<Path> + AsRef<OsStr> {

    let a_path = Path::new(&a);
    let b_path = Path::new(&b);

    if !a_path.exists() || !b_path.exists() {
        return Ok(false);
    }

    if a_path.is_dir() {
        if b_path.is_dir() {

            let mut a_entries = vec![];

            for entry in fs::read_dir(a_path)? {
                let entry = entry?;
                a_entries.push(entry.path());
            }

            let mut b_entries = vec![];

            for entry in fs::read_dir(b_path)? {
                let entry = entry?;
                b_entries.push(entry.path());
            }

            if b_entries.len() != a_entries.len() {
                return Ok(false);
            }

            for i in 0..a_entries.len() {
                if !same(&a_entries[i], &b_entries[i])? {
                    return Ok(false);
                }
            }


            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        if b_path.is_dir() {
            Ok(false)
        } else{

            let mut a_bytes = vec![];
            let mut b_bytes = vec![];

            let mut a_file = File::open(a_path)?;
            let mut b_file = File::open(b_path)?;

            a_file.read_to_end(&mut a_bytes)?;
            b_file.read_to_end(&mut b_bytes)?;

            let mut a_hasher = Sha256::new();
            let mut b_hasher = Sha256::new();

            a_hasher.update(&a_bytes);
            b_hasher.update(&b_bytes);

            let a_result = a_hasher.finalize();
            let b_result = b_hasher.finalize();

            Ok(a_result[..] == b_result[..])
        }
    }
}

pub fn proceed() -> Result<()> {
    let dotfiles_path = format!("{}/.dotfiles/dotfiles.dtm", env::var("HOME")?);

    setup()?;

    let content = fs::read_to_string(dotfiles_path)?;
    let len = content.lines().filter(|l| !l.starts_with("#")).collect::<Vec<_>>().len();

    println!("\x1b[0;32mLinking\x1b[0m {} files...", len);
    for (index, line) in content.lines().enumerate() {
        
        if line.starts_with("#") {
            continue;
        }
        let splited = line.split("->").collect::<Vec<_>>();
    
        if splited.len() != 2 {
            return Err(
                error!((format!("Invalid line format in ~/.dotfiles/dotfiles.dtm at line {} ({}).", index + 1, line)))
            )
        }

        if same(splited[0], splited[1])? {
            println!("`{}`->`{}`... \x1b[1;33mnot updated\x1b[0m", splited[0].replace(&env::var("HOME")?, "~"), splited[1].replace(&env::var("HOME")?, "~"));
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
        println!("`{}`->`{}`... \x1b[0;31mnok\x1b[0m", splited[0].replace(&env::var("HOME")?, "~"), splited[1].replace(&env::var("HOME")?, "~"));
        
    }
    println!("\x1b[0;32mSuccessfully\x1b[0m linked {} files.", len);

    Ok(())
}