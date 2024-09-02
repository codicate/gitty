use std::fs::File;
use std::io::{Read, Result};

pub fn main(hash: &str) -> Result<()> {
    let contents = cat_file(hash)?;
    println!("{}", contents);
    Ok(())
}

pub fn cat_file(hash: &str) -> Result<String> {
    let path = gyat::concat_path(gyat::DIROBJPATH, hash);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
