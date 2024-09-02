use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

pub fn main(hash: &str) -> Result<()> {
    let contents = cat_file(hash)?;
    println!("{}", contents);
    Ok(())
}

pub fn cat_file(hash: &str) -> Result<String> {
    let path = PathBuf::from(".gyat/objects").join(hash);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
