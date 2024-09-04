use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;

pub const CWD: &str = "./playground/";
pub const DIRPATH: &str = "playground/.gyat";
pub const DIROBJPATH: &str = "playground/.gyat/objects";
pub const IGNOREPATH: &str = "playground/.gyatignore";

pub fn concat_path(base_path: &str, folder_name: &str) -> String {
    if base_path.ends_with('/') {
        format!("{}{}", base_path, folder_name)
    } else {
        format!("{}/{}", base_path, folder_name)
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
