use super::tag;

pub fn main(refname: &str, _graph: &bool) -> () {
    let hash = match refname {
        "HEAD" => {
            let refname = tag::get_head().unwrap_or_default();
            println!("On branch {}", refname);
            tag::get_head_commit()
        }
        _ => tag::get_oid(refname),
    };

    match hash {
        Ok(hash) => traverse_commit_tree(&hash),
        Err(_) => println!("fatal: your current branch does not have any commits yet"),
    }
}

fn traverse_commit_tree(hash: &String) -> () {
    let parent_hash = print_commit(hash);
    if !parent_hash.is_empty() {
        println!();
        traverse_commit_tree(&parent_hash);
    }
}

fn print_commit(hash: &String) -> String {
    let (_, parent_hash, message) = read_commit(hash);
    println!("commit {}", hash);
    println!("message: {}", message);
    return parent_hash;
}

pub fn read_commit(hash: &String) -> (String, String, String) {
    let content = gyat::get_object_content(hash).unwrap();
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
        .unwrap_or_default()
        .to_string()
}
