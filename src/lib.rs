use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

pub struct Error(String);

pub fn error<T: ToString>(err: T) -> Error {
    Error(err.to_string())
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self(format!("{}", e))
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn get_dotfiles_path() -> String {
    format!("{}/.config/.dotfiles/", env::var("HOME").unwrap())
}

#[macro_export]
macro_rules! warn {
    ($($msg:tt),*) => {
        {
            eprint!("[\x1b[1;33m WARN \x1b[0m]");

            $(
                eprint!("{}", $msg);
             )*

            eprintln!();
        }
    }
}
#[macro_export]
macro_rules! ok {
    ($($msg:tt),*) => {
        {
            eprint!("[\x1b[0;32m  OK  \x1b[0m]");

            $(
                eprint!("{}", $msg);
             )*

            eprintln!();
        }
    }
}

pub fn is_even<T: AsRef<Path>, U: AsRef<Path>>(from: T, to: U) -> Result<bool> {
    if from.as_ref().is_dir() && to.as_ref().is_dir() {
        let from_files = fs::read_dir(from)?
            .filter(|e| !e.is_err())
            .map(|e| e.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<_>>();

        let to_files = fs::read_dir(to)?
            .filter(|e| !e.is_err())
            .map(|e| e.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<_>>();

        if from_files.len() != to_files.len() {
            Ok(false)
        } else {
            for i in 0..from_files.len() {
                if !is_even(&from_files[i], &to_files[i])? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
    } else if from.as_ref().is_file() && to.as_ref().is_file() {
        let mut from_buffer = vec![];
        let mut to_buffer = vec![];

        File::open(from)?.read_to_end(&mut from_buffer)?;
        File::open(to)?.read_to_end(&mut to_buffer)?;

        Ok(from_buffer == to_buffer)
    } else {
        Ok(false)
    }
}

fn readr<T: AsRef<Path>>(dir: T) -> Result<Vec<String>> {
    let mut elements = vec![];

    for entry in fs::read_dir(dir.as_ref())? {
        let path = entry?.path();

        if path.is_dir() {
            elements.push(path.to_str().unwrap().to_string());
            elements.extend(readr(path.to_str().unwrap())?);
        } else {
            elements.push(path.to_str().unwrap().to_string());
        }
    }

    Ok(elements)
}

pub fn copy_dir<T: AsRef<Path> + std::fmt::Display, U: AsRef<Path> + std::fmt::Display>(from: T, to: U) -> Result<()> {
    if !from.as_ref().exists() {
        return Err(error(format!("{} not found.", from)));
    } else if !from.as_ref().is_dir() {
        return Err(error(format!("{} is not a directory.", from)));
    } else if to.as_ref().exists() {
        return Err(error(format!("Cannot copy {} into {}: Destination file exists.", from, to)));
    } else {

        
        let current = env::current_dir()?;
        env::set_current_dir(&from)?;
        let content = readr(".")?;

        env::set_current_dir(&current)?;
        fs::create_dir_all(&to)?;
        env::set_current_dir(to)?;

        for element in content {
            let stringified = format!("{}/{}/{}", &current.to_str().unwrap(), from, &element);

            if Path::new(&stringified).is_dir() {
                fs::create_dir_all(&element)?;
            } else {
                let mut buffer = vec![];
                File::open(stringified)?.read_to_end(&mut buffer)?;
                File::create(&element)?.write_all(&buffer)?;
            }
        }
        
        env::set_current_dir(&current)?;

        Ok(())
    }
}

mod get_files;
mod link;
mod setup;
