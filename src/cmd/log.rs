use super::{cat_file::get_object_content, commit::read_head};

pub fn main() {
    let hash = read_head();
    if hash.is_empty() {
        println!("fatal: your current branch 'main' does not have any commits yet");
        return;
    }
    traverse_commit_tree(&hash);
}

fn traverse_commit_tree(hash: &String) -> () {
    println!("commit {}", hash);
    let (_, parent_hash, message) = read_commit(hash);
    println!("message: {}\n", message);

    if parent_hash == "first" {
        return;
    }
    traverse_commit_tree(&parent_hash);
}

pub fn read_commit(hash: &String) -> (String, String, String) {
    let content = get_object_content(hash).unwrap();
    let tree_hash = get_hash_from_line(&content, 0);
    let parent_hash = get_hash_from_line(&content, 1);
    let message = content.lines().nth(2).unwrap().to_string();
    return (tree_hash, parent_hash, message);
}

fn get_hash_from_line(content: &String, n: usize) -> String {
    content
        .lines()
        .nth(n)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .to_string()
}
