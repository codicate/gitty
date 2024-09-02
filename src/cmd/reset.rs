use crate::cmd::init;
use std::fs;

pub fn main() -> std::io::Result<()> {
    let dir_path = ".gyat";

    // Recursively delete the directory and its contents
    fs::remove_dir_all(dir_path)?;
    init::main()?;

    Ok(())
}
