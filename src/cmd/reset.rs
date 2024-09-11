use super::{log, read_tree};

pub fn main(hash: &String) -> () {
    let (tree_hash, _, _) = log::read_commit(&hash);
    read_tree::main(&tree_hash);
    println!("You are in 'detached HEAD' state. You can look around, make experimental changes and commit them, and you can discard any commits you make in this state without impacting any branches by switching back to a branch.");
}
