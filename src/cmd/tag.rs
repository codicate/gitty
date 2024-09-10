use super::hash_object;
use std::{fs, io::Result};

pub fn main(tagname: &String, refname: &String) {
    let hash = get_oid(refname).unwrap();
    write_ref(tagname, &hash, gyat::TAGPATH);
    println!("Tagging commit {} with name {}", hash, tagname);
}

pub fn get_ref(name: &str) -> Result<String> {
    let hash = gyat::get_file_content(gyat::HEADPATH, name);
    if hash.is_ok() {
        return Ok(hash.unwrap());
    }
    gyat::get_file_content(gyat::TAGPATH, name)
}

pub fn write_ref(name: &str, hash: &String, path: &str) -> () {
    let path = gyat::concat_path(path, name);
    hash_object::write_content_to_file(hash, path).unwrap();
}

pub fn get_head() -> Result<String> {
    get_ref("HEAD")
}

pub fn write_head(hash: &String) -> () {
    write_ref("HEAD", hash, gyat::TAGPATH);
}

pub fn get_oid(name: &str) -> Result<String> {
    let oid = get_ref(name).unwrap_or(name.to_string());
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
