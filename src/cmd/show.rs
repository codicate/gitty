use crate::cmd::read_tree;
use colored::Colorize;
use gitty::get_object_content;
use std::process::Command;
use std::{collections::HashMap, path::PathBuf};

use super::log;

pub fn main(hash: &String) -> () {
    let (cur_tree_hash, parent_hash, message) = log::read_commit(&hash);
    let parent_tree_hash = log::read_commit(&parent_hash).0;
    println!("commit {}", hash);
    println!("message: {}\n", message);

    compare_tree(&cur_tree_hash, &parent_tree_hash);
}

fn compare_tree(cur_tree_hash: &String, parent_tree_hash: &String) -> () {
    let mut cur_file_list: HashMap<String, String> = HashMap::new();
    let mut parent_file_list: HashMap<String, String> = HashMap::new();
    get_file_list(gitty::CWD, cur_tree_hash, &mut cur_file_list);
    get_file_list(gitty::CWD, parent_tree_hash, &mut parent_file_list);

    let mut deleted_files: Vec<&String> = Vec::new();
    let mut modified_files: Vec<(&String, &String, &String)> = Vec::new();
    let mut new_files: Vec<&String> = Vec::new();

    for (path, cur_hash) in &cur_file_list {
        if parent_file_list.contains_key(path) {
            let parent_hash = parent_file_list.get(path).unwrap();
            if parent_hash != cur_hash {
                modified_files.push((path, parent_hash, cur_hash));
            }
        } else {
            new_files.push(path);
        }
    }

    for (path, _) in &parent_file_list {
        if !cur_file_list.contains_key(path) {
            deleted_files.push(path);
        }
    }

    if !deleted_files.is_empty() {
        println!("Deleted files:");
        for file in deleted_files {
            let line = format!("- {}", file).red();
            println!("{}", line);
        }
        println!();
    }

    if !new_files.is_empty() {
        println!("New files:");
        for file in new_files {
            let line = format!("+ {}", file).green();
            println!("{}", line);
        }
        println!();
    }

    if !modified_files.is_empty() {
        for (file, hash1, hash2) in modified_files {
            let line = format!("> {}", file).yellow();
            println!("{}", line);
            print_file_diffs(hash1, hash2);
        }
    }
}

fn get_file_list(dir: &str, tree_hash: &String, file_list: &mut HashMap<String, String>) -> () {
    let tree_content = gitty::get_object_content(tree_hash).unwrap();
    let lines = read_tree::parse_tree_file(tree_content);

    for (filetype, hash, filename) in lines {
        let path = PathBuf::from(dir)
            .join(filename)
            .to_str()
            .unwrap()
            .to_string();

        if filetype == "tree" {
            get_file_list(&path, &hash, file_list);
        } else {
            file_list.insert(path, hash);
        }
    }
}

fn print_file_diffs(hash1: &String, hash2: &String) {
    let path1 = gitty::concat_path(gitty::OBJPATH, hash1);
    let path2 = gitty::concat_path(gitty::OBJPATH, hash2);

    let output = Command::new("diff")
        .arg("-u")
        .arg(path1)
        .arg(path2)
        .output()
        .expect("failed to execute diff command");

    let diff = String::from_utf8(output.stdout).unwrap();
    println!("{}", diff);
}
