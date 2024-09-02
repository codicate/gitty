use crate::cmd::init;
use std::fs;

pub fn main() -> std::io::Result<()> {
    // Recursively delete the directory and its contents
    fs::remove_dir_all(gyat::DIRPATH)?;
    init::main()?;

    Ok(())
}
