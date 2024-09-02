use crate::cmd::hash_object;
use std::collections::HashSet;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

pub fn main(print: &bool) -> Result<()> {
    let ignored_files = gyat::get_ignored_file_list();
    let cwd = PathBuf::from(".");
    let hash = navigate_folders_recursively(&cwd, &ignored_files, print)?;
    println!("{}", hash);
    Ok(())
}

fn navigate_folders_recursively(
    dir: &Path,
    ignored_files: &HashSet<String>,
    print: &bool,
) -> Result<String> {
    let children = fs::read_dir(dir)?;
    let mut tree_content = String::new();

    for child in children {
        let path = child?.path();
        let path_string: String = path.to_str().unwrap()[2..].to_string(); // remove "./"

        if ignored_files.contains(&path_string) {
            continue;
        }

        if *print {
            println!("{}", path_string);
        }

        let hash = if path.is_dir() {
            navigate_folders_recursively(&path, ignored_files, print)?
        } else {
            hash_object::hash_object_file(&path_string, &true)?
        };

        let content_type = if path.is_dir() { "tree" } else { "blob" };
        let file_name = path.file_name().unwrap().to_str().unwrap();
        tree_content.push_str(&format!("{} {} {}\n", content_type, hash, file_name));
    }

    let hash = hash_object::hash_object(&tree_content, &true)?;
    Ok(hash)
}
