use crate::cmd::hash_object;
use crate::cmd::write_tree;

pub fn main(message: &String) -> () {
    let hash = write_tree::write_tree(&false);
    let mut content = String::new();

    content.push_str(&format!("tree {}\n", hash));
    content.push_str(&format!("parent {}\n", read_head()));
    content.push_str(message);

    let hash = hash_object::store_content_as_object(&content).unwrap();
    write_head(&hash);
}

pub fn read_head() -> String {
    let content = gyat::get_file_content(gyat::HEAD).unwrap();
    if content.is_empty() {
        return "first".to_string();
    }
    content
}

fn write_head(hash: &String) -> () {
    hash_object::write_content_to_file(hash, gyat::HEAD).unwrap();
}
