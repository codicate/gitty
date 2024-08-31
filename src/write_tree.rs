use crate::hash_object;
use std::io::Result;
use std::path::Path;
use std::{env, fs};

pub fn main() {
    match write_tree() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn write_tree() -> Result<()> {
    let cwd = env::current_dir().unwrap();
    navigate_folders_recursively(&cwd);
    Ok(())
}

fn navigate_folders_recursively(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    println!("{}", path.display());
                    navigate_folders_recursively(&path);
                } else {
                    let path_string = path.to_str().unwrap().to_string();
                    // hash_object::main(&path_string, &true)
                }
            }
        }
    }
}
