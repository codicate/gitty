use crate::cmd::tag;

use super::{log::read_commit, read_tree};

pub fn main(name: &String, new_branch: &bool) -> () {
    if *new_branch {
        create_new_branch(name);
    } else {
        move_head(name);
    }
}

fn move_head(name: &String) -> () {
    let hash = tag::get_oid(name).expect(&format!("fatal: reference '{}' cannot be found", name));
    let (tree_hash, _, _) = read_commit(&hash);

    read_tree::main(&tree_hash);
    tag::write_head(name);
    println!("Switched to branch '{}'", name);
}

pub fn create_main_branch(hash: &String) {
    tag::write_ref("main", hash, gitty::HEADPATH);
    tag::write_head(&"main".to_string());
}

fn create_new_branch(name: &String) {
    let hash = tag::get_head_commit();
    match hash {
        Ok(hash) => {
            tag::write_ref(name, &hash, gitty::HEADPATH);
            tag::write_head(name);
            println!("Created and switched to a new branch: {}", name);
        }
        Err(_) => println!("fatal: your current branch does not have any commits yet"),
    }
}

pub fn update_branch(hash: &String) {
    let refname = tag::get_head().unwrap();
    tag::write_ref(&refname, hash, gitty::HEADPATH);
}
