use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Result, Write};

pub fn main(path: &String, write: &bool) {
    match hash_object(path, write) {
        Ok(hash) => println!("{}", hash),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn hash_object(path: &String, write: &bool) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut hasher = Sha1::new();
    hasher.update(&contents);
    let result = hasher.finalize();

    // Convert hash digest in the form of GenericArray to hex string
    let hash: String = result.iter().map(|byte| format!("{:02x}", byte)).collect();

    if *write {
        write_to_file(&hash, &contents)?;
    }

    Ok(hash)
}

fn write_to_file(hash: &String, contents: &String) -> Result<()> {
    let path = ".gyat/objects";
    let mut file = File::create(format!("{0}/{1}", path, hash))?;
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}
