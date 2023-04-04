use std::{fmt};
use std::fmt::{Formatter};
use std::fs::{DirEntry};
use std::io;
use std::path::{PathBuf};
use crate::RustyError::{EntryInitializationError, IOError};

const ROOT_PATH: &str = ".";
const UNKNOWN_FILE_NAME: &str = "Unknown";

pub fn run() -> Result {
    let path = PathBuf::from(ROOT_PATH);
    let entry = path.read_dir()?;
    for e in entry {
        let dir = Directory::new(e?)?;
        dir.traverse()
    }
    Ok(())
}

trait Entry {
    fn name(&self) -> String;
    fn traverse(&self) {
        // no-op
    }

    fn get_name(entry: &DirEntry) -> String {
        match entry.file_name().into_string() {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Error getting name for {:?}:{:?}", entry, e);
                UNKNOWN_FILE_NAME.to_string()
            }
        }
    }
}

struct Directory {
    entry: DirEntry,
}

impl Directory {
    fn new(entry: DirEntry) -> Result<impl Entry> {
        // let file_type = entry.file_type().expect("We have no idea");
        println!("{:?}", entry.file_name());
        // if file_type.is_dir() {
        //     Ok(Directory { entry })
        // } else {
        //     Err(EntryInitializationError)
        // }
        Ok(Directory { entry })
    }
}

impl Entry for Directory {
    fn name(&self) -> String {
        Directory::get_name(&self.entry)
    }

    fn traverse(&self) {
        for entry in self.entry.path().read_dir().expect("Check traverse") {
            // block(Entry::new(entry?))?
            println!("Traversing children {:?}", entry)
        }
    }
}

struct File {
    entry: DirEntry,
}

impl Entry for File {
    fn name(&self) -> String {
        File::get_name(&self.entry)
    }
}

type Result<T = ()> = std::result::Result<T, RustyError>;

pub enum RustyError {
    IOError(io::Error),
    EntryInitializationError,
}

impl fmt::Display for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IOError(e) => f.write_fmt(format_args!("{e}")),
            EntryInitializationError => f.write_str("Couldn't initialize an entry")
        }
    }
}

impl From<io::Error> for RustyError {
    fn from(value: io::Error) -> Self {
        IOError(value)
    }
}