use crate::cmd::checkout;
use crate::cmd::hash_object;
use crate::cmd::write_tree;

pub fn main(message: &String) -> () {
    let hash = write_tree::write_tree(&false);
    let mut content = String::new();

    content.push_str(&format!("tree {}\n", hash));
    content.push_str(&format!("parent {}\n", checkout::read_head()));
    content.push_str(message);

    let hash = hash_object::store_content_as_object(&content).unwrap();
    checkout::write_head(&hash);
}
