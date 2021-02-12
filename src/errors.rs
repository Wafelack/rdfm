use std::{
    env,
    fmt::{Debug, Formatter, Result as fmtRes},
};

#[macro_export]
macro_rules! error {
    ($msg:tt) => {
        RdfmError {
            message: $msg.to_string(),
        }
    };
}

pub struct RdfmError {
    pub message: String,
}

impl Debug for RdfmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtRes {
        write!(f, "{}.", self.message)
    }
}

impl From<env::VarError> for RdfmError {
    fn from(_: env::VarError) -> Self {
        error!("environment variable `HOME` not found.")
        // I hardcode the variable name because it is the only time
        // I'll use env::var, I'll adjust if necesary.
    }
}
impl From<fs_extra::error::Error> for RdfmError {
    fn from(e: fs_extra::error::Error) -> Self {
        error!(e)
    }
}

impl From<std::io::Error> for RdfmError {
    fn from(e: std::io::Error) -> Self {
        error!(e)
        // I hardcode the variable name because it is the only time
        // I'll use env::var, I'll adjust if necesary.
    }
}

impl From<git2::Error> for RdfmError {
    fn from(e: git2::Error) -> Self {
        error!(e)
    }
}

pub type Result<T> = std::result::Result<T, RdfmError>;
