use std::{
    fmt::{self, Formatter},
    fs::{ReadDir, DirEntry},
    io,
    env,
};

use crate::RustyError::IOError;

pub fn run() -> Result {
    let current_path = env::current_dir()?;
    let read_dir = current_path.read_dir()?;
    println!(".");
    list(read_dir, String::new())?;
    Ok(())
}

fn list(read_dir: ReadDir, prefix: String) -> Result {
    let mut peekable = read_dir
        .filter(|entry| !is_hidden(entry))
        .peekable();

    while let Some(entry) = peekable.next() {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or("Unknown");

        let (separator, symbol) = match peekable.peek() {
            Some(_) => ("│  ", "├──"),
            None => ("  ", "└──")
        };

        println!("{}{} {}", prefix, symbol, file_name);

        if file_type.is_dir() {
            list(entry.path().read_dir()?, format!("{}{}", prefix, separator))?;
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