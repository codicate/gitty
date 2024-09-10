use super::hash_object;
use std::{fs, io::Result};

pub fn main(name: &String, hash: &String) {
    println!("Tagging commit {} with name {}", hash, name);
    write_tag(name, hash);
}

pub fn get_tag(name: &str) -> Result<String> {
    gyat::get_file_content(gyat::TAGPATH, name)
}

pub fn write_tag(name: &str, hash: &String) -> () {
    let path = gyat::concat_path(gyat::TAGPATH, name);
    hash_object::write_content_to_file(hash, path).unwrap();
}

pub fn get_head() -> Result<String> {
    get_tag("HEAD")
}

pub fn write_head(hash: &String) -> () {
    write_tag("HEAD", hash);
}

pub fn get_oid(name: &str) -> Result<String> {
    let oid = get_tag(name).unwrap_or(name.to_string());
    let path = gyat::concat_path(gyat::OBJPATH, &oid);
    if fs::metadata(path).is_ok() {
        Ok(oid)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Object not found",
        ))
    }
}
