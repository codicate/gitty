use std::path::Path;
use std::{env, fs};

pub fn main() -> () {
    let path = Path::new(gyat::DIRPATH);
    if path.is_dir() {
        panic!("This repository has already been initialized");
    }

    fs::create_dir_all(gyat::OBJPATH).unwrap();
    fs::create_dir_all(gyat::TAGPATH).unwrap();

    let cwd = env::current_dir().unwrap();
    println!(
        "Initialized empty gyat repository in {}\\.gyat",
        cwd.display()
    );
}
