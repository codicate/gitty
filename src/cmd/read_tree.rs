use crate::cmd::cat_file;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};
pub fn main(hash: &str) -> Result<()> {
    let path = PathBuf::from(".gyat/objects").join(hash);
    if !Path::new(&path).exists() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid hash provided"));
    }

    let ignored_files = gyat::get_ignored_file_list();
    delete_cwd(".", &ignored_files)?;
    restore_cwd(".", hash)?;

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

fn restore_cwd<P: AsRef<Path>>(path: P, hash: &str) -> Result<()> {
    let objpath = PathBuf::from(".gyat/objects").join(hash);
    let file = fs::File::open(objpath)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_>>().unwrap_or_default();
    let lines = parse_tree_file(lines);

    for (filetype, hash, filename) in lines {
        let path = PathBuf::from(path.as_ref()).join(filename);
        if filetype == "tree" {
            fs::create_dir(path.clone())?;
            restore_cwd(path, &hash)?;
        } else {
            let contents = cat_file::cat_file(&hash)?;
            let mut file = fs::File::create(path)?;
            file.write_all(contents.as_bytes())?;
            file.flush()?;
        }
    }

    Ok(())
}

fn parse_tree_file(input: Vec<String>) -> Vec<(String, String, String)> {
    input
        .into_iter()
        .map(|s| {
            let mut parts = s.split_whitespace();
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            )
        })
        .collect()
}
