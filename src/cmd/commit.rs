use crate::cmd::checkout;
use crate::cmd::hash_object;
use crate::cmd::tag;
use crate::cmd::write_tree;

pub fn main(message: &String) -> () {
    let hash = write_tree::write_tree(&false);
    let mut content = String::new();

    content.push_str(&format!("tree {}\n", hash));
    let parent = tag::get_head_commit().unwrap_or_default();
    content.push_str(&format!("parent {}\n", parent));
    content.push_str(message);

    let hash = hash_object::store_content_as_object(&content).unwrap();
    println!("commit: {}", hash);

    if parent.is_empty() {
        checkout::create_main_branch(&hash);
    } else {
        checkout::update_branch(&hash);
    }
}
