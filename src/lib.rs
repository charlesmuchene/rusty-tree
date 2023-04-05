use std::{
    env,
    fmt::{self, Formatter},
    fs::{DirEntry, ReadDir},
    io,
    iter::Peekable
};

use crate::RustyError::IOError;

const FILENAME_SYMBOL: &str = "├── ";
const FILENAME_LAST_SYMBOL: &str = "└── ";
const FILENAME_SEPARATOR: &str = "│  ";
const FILENAME_LAST_SEPARATOR: &str = "  ";
const CURRENT_DIR: &str = ".";

/// Entry point to listing logic
///
/// Returns a `Result` which is an alias for `std::result::Result<(), RustyError`
pub fn run() -> Result {
    let current_path = env::current_dir()?;
    let read_dir = current_path.read_dir()?;
    println!("{}", CURRENT_DIR);
    list(read_dir, String::new())?;
    Ok(())
}

/// List contents of `ReadDir` using the given `prefix`
fn list(read_dir: ReadDir, prefix: String) -> Result {
    let mut peekable = read_dir
        .filter(|entry| !is_hidden(entry))
        .peekable();

    while let Some(entry) = peekable.next() {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_name = get_filename(&entry);
        let symbol = filename_symbol(&mut peekable);

        println!("{}{}{}", prefix, symbol.symbol, file_name);

        if file_type.is_dir() {
            let read_dir = entry.path().read_dir()?;
            let prefix = format!("{}{}", prefix, symbol.separator);
            list(read_dir, prefix)?;
        }
    }
    Ok(())
}

/// Extract file name from `DirEntry`: defaults to "Unknown"
fn get_filename(entry: &DirEntry) -> String {
    entry.file_name().to_str().unwrap_or("Unknown").to_string()
}

/// Determine the separator and symbol to use for printing filename based on the status of the
/// given `Peekable`.
fn filename_symbol<T: Iterator>(peekable: &mut Peekable<T>) -> FilenameSymbol {
    match peekable.peek() {
        Some(_) => FilenameSymbol { separator: FILENAME_SEPARATOR, symbol: FILENAME_SYMBOL },
        None => FilenameSymbol { separator: FILENAME_LAST_SEPARATOR, symbol: FILENAME_LAST_SYMBOL }
    }
}

/// Determines if the entry pointed to by `entry` is hidden
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

// FIXME Shouldn't shadow `std::result::Result`
type Result<T = ()> = std::result::Result<T, RustyError>;

/// `FilenameSymbol` groups a prefix to a filename:
/// * symbol -> a group of characters printed before filename
/// * separator -> a group of characters forming the prefix of child files
struct FilenameSymbol {
    symbol: &'static str,
    separator: &'static str,
}

/// Custom error
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