use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Result;
use std::path::{Path, PathBuf};

pub fn main(hash: &str) -> Result<()> {
    let mut path = PathBuf::from(".gyat/objects");
    path.push(hash);
    let file = File::open(path).expect("Invalid hash provided");

    let ignored_files = gyat::get_ignored_file_list();
    delete_cwd(".", &ignored_files)?;

    Ok(())
}

fn delete_cwd<P: AsRef<Path>>(path: P, ignored_files: &HashSet<String>) -> Result<()> {
    for child in fs::read_dir(&path)? {
        let path = child?.path();
        let path_string: String = path.to_str().unwrap()[2..].to_string(); // remove "./"

        if ignored_files.contains(&path_string) {
            continue;
        }

        if path.is_dir() {
            delete_cwd(&path, ignored_files)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    fs::remove_dir(path)?;
    Ok(())
}
