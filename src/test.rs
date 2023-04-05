#[cfg(test)]
mod test {
    use std::fs::DirEntry;
    use std::path::{PathBuf};
    use crate::{get_filename, filename_symbol, FILENAME_LAST_SYMBOL, FILENAME_SYMBOL};

    #[test]
    // Flaky!! Relies on current dir contents + read_dir ordering
    fn extract_filename() {
        let entry = dir_entry();
        let file_name = get_filename(&entry);
        assert_eq!(file_name, "Cargo.toml");
    }

    #[test]
    fn create_last_filename_symbol() {
        let mut peekable = [0; 0].iter().peekable();
        assert_eq!(peekable.peek(), None);
        assert_eq!(filename_symbol(&mut peekable).symbol, FILENAME_LAST_SYMBOL)
    }

    #[test]
    fn create_filename_symbol() {
        let mut peekable = [0; 1].iter().peekable();
        assert_eq!(peekable.peek(), Some(&&0));
        assert_eq!(filename_symbol(&mut peekable).symbol, FILENAME_SYMBOL)
    }

    fn dir_entry() -> DirEntry {
        let path = PathBuf::from(".");
        let mut read_dir = path.read_dir().unwrap();
        read_dir.next().unwrap().unwrap()
    }
}