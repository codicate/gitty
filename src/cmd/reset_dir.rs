use crate::cmd::init;
use std::fs;

pub fn main() -> () {
    // Recursively delete the directory and its contents
    fs::remove_dir_all(gyat::DIRPATH).unwrap();
    init::main();
}
