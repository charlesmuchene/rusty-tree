use std::{
    fmt::{self, Formatter},
    fs::{ReadDir},
    io,
    env,
};
use std::fs::DirEntry;

use crate::RustyError::IOError;

pub fn run() -> Result {
    let current_path = env::current_dir()?;
    let read_dir = current_path.read_dir()?;
    list(read_dir)?;
    Ok(())
}

fn list(read_dir: ReadDir) -> Result {
    for entry in read_dir.filter(|e| !is_hidden(e)) {
        let entry = entry?;
        let file_type = entry.file_type()?;
        println!("* {:?}", entry.file_name());
        if file_type.is_dir() {
            list(entry.path().read_dir()?)?;
        }
    }
    Ok(())
}

fn is_hidden(entry: &std::result::Result<DirEntry, io::Error>) -> bool {
    if let Ok(entry) = entry {
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    } else {
        false
    }
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