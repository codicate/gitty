use super::hash_object;

pub fn main(name: &String, hash: &String) {
    println!("Tagging commit {} with name {}", hash, name);
}

pub fn get_tag(name: &str) -> String {
    gyat::get_file_content(gyat::TAGPATH, name).unwrap_or_default()
}

pub fn write_tag(name: &str, hash: &String) -> () {
    let path = gyat::concat_path(gyat::TAGPATH, name);
    hash_object::write_content_to_file(hash, path).unwrap();
}

pub fn read_head() -> String {
    let content = get_tag("HEAD");
    if content.is_empty() {
        return "first".to_string();
    }
    content
}

pub fn write_head(hash: &String) -> () {
    write_tag("HEAD", hash);
}
