use std::process;
use rusty_tree::{run};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1)
    }
}
