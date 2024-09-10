use super::{log::read_commit, read_tree, tag};

pub fn main(name: &String, new_branch: &bool) {
    if *new_branch {
        create_new_branch(name);
    } else {
        move_head(name);
    }
}

fn move_head(name: &String) -> () {
    let hash = tag::get_oid(name);
    match hash {
        Ok(hash) => {
            tag::write_head(&hash);
            let (tree_hash, _, _) = read_commit(&hash);
            read_tree::main(&tree_hash);
            println!("Moved HEAD to: {}", name);
        }
        Err(_) => println!("fatal: reference '{}' cannot be found", name),
    }
}

fn create_new_branch(name: &String) {
    println!("Creating a new branch: {}", name);
}
