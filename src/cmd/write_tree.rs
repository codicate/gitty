use crate::cmd::hash_object;
// use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn main(print: &bool) -> () {
    let hash = write_tree(print);
    println!("{}", hash);
}

pub fn write_tree(print: &bool) -> String {
    let ignored_files = gitty::get_ignored_file_list();
    let hash = navigate_folders_recursively(gitty::CWD, &ignored_files, print).unwrap();
    hash
}

fn navigate_folders_recursively<P: AsRef<Path>>(
    path: P,
    ignored_files: &HashSet<String>,
    print: &bool,
) -> Result<String> {
    let children = fs::read_dir(path)?;
    let mut tree_content = String::new();

    for child in children {
        let path = child?.path();
        let path_string = gitty::strip_path(&path);

        if ignored_files.contains(&path_string) {
            continue;
        }

        if *print {
            println!("{}", path_string);
        }

        let hash = if path.is_dir() {
            navigate_folders_recursively(&path, ignored_files, print).unwrap()
        } else {
            hash_object::store_file_as_object(&path_string).unwrap()
        };

        let content_type = if path.is_dir() { "tree" } else { "blob" };
        let file_name = path.file_name().unwrap().to_str().unwrap();
        tree_content.push_str(&format!("{} {} {}\n", content_type, hash, file_name));
    }

    let hash = hash_object::store_content_as_object(&tree_content).unwrap();
    Ok(hash)
}
