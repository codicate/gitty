use super::hash_object;
use std::{fs, io::Result};

pub fn main(tagname: &String, refname: &String) {
    let hash = get_oid(refname).unwrap();
    write_ref(tagname, &hash, gitty::TAGPATH);
    println!("Tagging commit {} with name {}", hash, tagname);
}

pub fn get_ref(name: &str) -> Result<String> {
    let hash = gitty::get_file_content(gitty::HEADPATH, name);
    if hash.is_ok() {
        return Ok(hash.unwrap());
    }
    gitty::get_file_content(gitty::TAGPATH, name)
}

pub fn write_ref(name: &str, hash: &String, path: &str) -> () {
    let path = gitty::concat_path(path, name);
    hash_object::write_content_to_file(hash, path).unwrap();
}

pub fn get_head() -> Result<String> {
    get_ref("HEAD")
}

pub fn get_head_commit() -> Result<String> {
    let refname = get_head()?;
    let hash = get_ref(&refname)?;
    Ok(hash)
}

pub fn write_head(refname: &String) -> () {
    write_ref("HEAD", refname, gitty::HEADPATH);
}

pub fn get_oid(name: &str) -> Result<String> {
    let oid = get_ref(name).unwrap_or(name.to_string());
    let path = gitty::concat_path(gitty::OBJPATH, &oid);
    if fs::metadata(path).is_ok() {
        Ok(oid)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Object not found",
        ))
    }
}
