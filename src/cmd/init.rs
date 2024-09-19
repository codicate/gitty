use std::path::Path;
use std::{env, fs};

pub fn main() -> () {
    let path = Path::new(gitty::DIRPATH);
    if path.is_dir() {
        panic!("This repository has already been initialized");
    }

    fs::create_dir_all(gitty::OBJPATH).unwrap();
    fs::create_dir_all(gitty::TAGPATH).unwrap();
    fs::create_dir_all(gitty::HEADPATH).unwrap();

    let cwd = env::current_dir().unwrap();
    println!(
        "Initialized empty gitty repository in {}\\.gitty",
        cwd.display()
    );
}
