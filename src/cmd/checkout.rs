use crate::cmd::tag;

use super::{log::read_commit, read_tree};

pub fn main(name: &String, new_branch: &bool, detached: &bool) -> () {
    if *new_branch {
        create_new_branch(name);
    } else {
        move_head(name, detached);
    }
}

fn move_head(name: &String, detached: &bool) -> () {
    let hash = tag::get_oid(name).expect(&format!("fatal: reference '{}' cannot be found", name));
    let (tree_hash, _, _) = read_commit(&hash);
    read_tree::main(&tree_hash);

    if !*detached {
        tag::write_head(name);
        println!("Switched to branch '{}'", name);
    } else {
        println!("You are in 'detached HEAD' state. You can look around, make experimental changes and commit them, and you can discard any commits you make in this state without impacting any branches by switching back to a branch.");
    }
}

pub fn create_main_branch(hash: &String) {
    tag::write_ref("main", hash, gyat::HEADPATH);
    tag::write_head(&"main".to_string());
}

fn create_new_branch(name: &String) {
    let hash = tag::get_head_commit();
    match hash {
        Ok(hash) => {
            tag::write_ref(name, &hash, gyat::HEADPATH);
            tag::write_head(name);
            println!("Created and switched to a new branch: {}", name);
        }
        Err(_) => println!("fatal: your current branch does not have any commits yet"),
    }
}

pub fn update_branch(hash: &String) {
    let refname = tag::get_head().unwrap();
    tag::write_ref(&refname, hash, gyat::HEADPATH);
}
