use std::collections::HashSet;
use std::fs;
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

pub fn main(hash: &str) -> () {
    let path = gyat::concat_path(gyat::OBJPATH, hash);
    if !Path::new(&path).exists() {
        panic!("Invalid hash provided");
    }

    let ignored_files = gyat::get_ignored_file_list();
    delete_cwd(gyat::CWD, &ignored_files).unwrap();
    restore_cwd(gyat::CWD, hash).unwrap();
}

fn delete_cwd(dir: &str, ignored_files: &HashSet<String>) -> Result<()> {
    for child in fs::read_dir(&dir)? {
        let path = child?.path();
        let path_string = gyat::strip_path(&path);

        if ignored_files.contains(&path_string) {
            continue;
        }

        if path.is_dir() {
            delete_cwd(&path_string, ignored_files)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    if dir != gyat::CWD {
        fs::remove_dir(dir)?;
    }
    Ok(())
}

fn restore_cwd(dir: &str, hash: &str) -> Result<()> {
    let tree_content = gyat::get_object_content(hash)?;
    let lines = parse_tree_file(tree_content);

    for (filetype, hash, filename) in lines {
        let path = PathBuf::from(dir).join(filename);
        let path_string = gyat::strip_path(&path);

        if filetype == "tree" {
            fs::create_dir(path)?;
            restore_cwd(&path_string, &hash)?;
        } else {
            let contents = gyat::get_object_content(&hash)?;
            let mut file = fs::File::create(path)?;
            file.write_all(contents.as_bytes())?;
            file.flush()?;
        }
    }

    Ok(())
}

pub fn parse_tree_file(input: String) -> Vec<(String, String, String)> {
    input
        .lines()
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
