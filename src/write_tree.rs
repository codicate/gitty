use crate::hash_object;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::path::PathBuf;

pub fn main(print: &bool) {
    match write_tree(print) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn write_tree(print: &bool) -> Result<()> {
    let default_ignored_files: Vec<String> = [".gyat", ".git"].map(str::to_string).to_vec();
    let ignorefile = ".gyatignore";
    let mut lines = read_lines(ignorefile)?;
    lines.iter_mut().for_each(|x| {
        if x.ends_with('/') {
            x.pop();
        }
    });

    lines.extend(default_ignored_files);
    let ignored_files: HashSet<_> = lines.into_iter().collect();

    let cwd = PathBuf::from(".");
    navigate_folders_recursively(&cwd, &ignored_files, print)?;
    Ok(())
}

fn read_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn navigate_folders_recursively(
    dir: &Path,
    ignored_files: &HashSet<String>,
    print: &bool,
) -> Result<()> {
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let path = entry?.path();
        let path_string: String = path.to_str().unwrap()[2..].to_string();

        if ignored_files.contains(&path_string) {
            continue;
        }

        if *print {
            println!("{}", path_string);
        }

        if path.is_dir() {
            navigate_folders_recursively(&path, ignored_files, print)?;
        } else {
            hash_object::main(&path_string, &true, &false)
        }
    }
    Ok(())
}
