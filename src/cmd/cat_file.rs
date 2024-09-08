use std::fs::File;
use std::io::{Read, Result};

pub fn main(hash: &str) -> () {
    let contents = get_object_content(hash).unwrap();
    println!("{}", contents);
}

pub fn get_file_content(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_object_content(hash: &str) -> Result<String> {
    let path = gyat::concat_path(gyat::DIROBJPATH, hash);
    get_file_content(&path)
}
