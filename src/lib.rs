use std::env;

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
            eprint!("[\x1b[1;33mWARN\x1b[0m]");

            $(
                eprint!("{}", $msg);
             )*

            eprintln!();
        }
    }
}

mod setup;
mod get_files;
