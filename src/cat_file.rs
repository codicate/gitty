use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

pub fn main(hash: &str) {
    match cat_file(hash) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cat_file(hash: &str) -> Result<String> {
    let mut path = PathBuf::from(".gyat/objects");
    path.push(hash);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
