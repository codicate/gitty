use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};
use std::path::PathBuf;

pub const CWD: &str = "./playground/";
pub const DIRPATH: &str = "playground/.gyat";
pub const OBJPATH: &str = "playground/.gyat/objects";
pub const TAGPATH: &str = "playground/.gyat/refs/tags";
pub const HEADPATH: &str = "playground/.gyat/refs/heads";
pub const IGNOREPATH: &str = "playground/.gyatignore";

pub fn concat_path(a: &str, b: &str) -> String {
    if a.ends_with('/') {
        format!("{}{}", a, b)
    } else {
        format!("{}/{}", a, b)
    }
}

pub fn strip_path(path: &PathBuf) -> String {
    path.to_str()
        .map(|s| s.trim_start_matches("./").to_string())
        .unwrap_or_default()
}

pub fn get_ignored_file_list() -> HashSet<String> {
    let default_ignored_files: Vec<String> = [".gyat", ".git"].map(str::to_string).to_vec();

    let mut lines = read_ignorefile(IGNOREPATH);
    lines.extend(default_ignored_files);
    return lines.into_iter().collect();
}

fn read_ignorefile(ignorefile: &str) -> Vec<String> {
    let file = match File::open(ignorefile) {
        Ok(file) => file,
        Err(_) => {
            println!("It is recommended to create a .gyatignore file in the root directory");
            return Vec::new();
        }
    };

    let reader = BufReader::new(file);
    let mut lines: Vec<_> = reader.lines().collect::<Result<_>>().unwrap_or_default();

    lines.iter_mut().for_each(|x| {
        if x.ends_with('/') {
            x.pop();
        }
    });
    return lines;
}

pub fn get_file_content_by_path(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_file_content(path: &str, name: &str) -> Result<String> {
    let path = concat_path(path, name);
    get_file_content_by_path(&path)
}

pub fn get_object_content(hash: &str) -> Result<String> {
    get_file_content(OBJPATH, hash)
}
