use super::{log::read_commit, read_tree};

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
    let (tree_hash, _, _) = read_commit(hash);
    read_tree::main(&tree_hash);
}

fn create_new_branch(branchname: &String) {}

fn switch_to_branch(branchname: &String) {}
