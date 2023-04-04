use std::{
    fmt::{self, Formatter},
    fs::{ReadDir},
    io,
    env
};

use crate::RustyError::IOError;

pub fn run() -> Result {
    let current_path = env::current_dir()?;
    let read_dir = current_path.read_dir()?;
    list(read_dir)?;
    Ok(())
}

fn list(read_dir: ReadDir) -> Result {
    for entry in read_dir {
        let entry = entry?;
        let file_type = entry.file_type()?;
        println!("* {:?}", entry.file_name());
        if file_type.is_dir() {
            list(entry.path().read_dir()?)?;
        }
    }
    Ok(())
}

type Result<T = ()> = std::result::Result<T, RustyError>;

pub enum RustyError {
    IOError(io::Error),
}

impl fmt::Display for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IOError(e) => f.write_fmt(format_args!("{e}")),
        }
    }
}

impl From<io::Error> for RustyError {
    fn from(value: io::Error) -> Self {
        IOError(value)
    }
}