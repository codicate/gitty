use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

pub fn main(hash: &str) -> Result<()> {
    let mut path = PathBuf::from(".gyat/objects");
    path.push(hash);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
