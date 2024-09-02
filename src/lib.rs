use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn get_ignored_file_list() -> HashSet<String> {
    let default_ignored_files: Vec<String> = [".gyat", ".git"].map(str::to_string).to_vec();
    let ignorefile = ".gyatignore";

    let mut lines = read_ignorefile(ignorefile);
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
