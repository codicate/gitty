use crate::cmd::read_tree;
use colored::Colorize;
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
    get_file_list(gyat::CWD, cur_tree_hash, &mut cur_file_list);
    get_file_list(gyat::CWD, parent_tree_hash, &mut parent_file_list);

    let mut deleted_files: Vec<String> = Vec::new();
    let mut modified_files: Vec<String> = Vec::new();
    let mut new_files: Vec<String> = Vec::new();

    for (path, hash) in &cur_file_list {
        if parent_file_list.contains_key(path) {
            if parent_file_list.get(path).unwrap() != hash {
                modified_files.push(path.clone());
            }
        } else {
            new_files.push(path.clone());
        }
    }

    for (path, _) in &parent_file_list {
        if !cur_file_list.contains_key(path) {
            deleted_files.push(path.clone());
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
        println!("Modified files:");
        for file in modified_files {
            let line = format!("- {}", file).yellow();
            println!("{}", line);
        }
        println!();
    }
}

fn get_file_list(dir: &str, tree_hash: &String, file_list: &mut HashMap<String, String>) -> () {
    let tree_content = gyat::get_object_content(tree_hash).unwrap();
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
