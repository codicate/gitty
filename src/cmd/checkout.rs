use super::{hash_object, log::read_commit, read_tree};

pub fn main(branchname: &String, new_branch: &bool, hash: &bool) {
    if *hash {
        println!("Switching to commit hash: {}", branchname);
        switch_to_commit_hash(branchname);
    } else if *new_branch {
        println!("Creating a new branch: {}", branchname);
        create_new_branch(branchname);
    } else {
        println!("Switching to branch: {}", branchname);
        switch_to_branch(branchname);
    }
}

fn switch_to_commit_hash(hash: &String) {
    write_head(hash);
    let (tree_hash, _, _) = read_commit(hash);
    read_tree::main(&tree_hash);
}

fn create_new_branch(branchname: &String) {}

fn switch_to_branch(branchname: &String) {}

pub fn read_head() -> String {
    let content = gyat::get_file_content(gyat::HEAD).unwrap();
    if content.is_empty() {
        return "first".to_string();
    }
    content
}

pub fn write_head(hash: &String) -> () {
    hash_object::write_content_to_file(hash, gyat::HEAD).unwrap();
}
