use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Result, Write};

pub fn main(path: &str, write: &bool, print: &bool) -> Result<()> {
    let hash = hash_object_file(path, write)?;

    if *print {
        println!("{}", hash)
    }

    Ok(())
}

pub fn hash_object_file(path: &str, write: &bool) -> Result<String> {
    let mut file = File::open(path).expect(&format!("Failed to open {} in hash_object_file", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let hash = hash_object(&contents, write)?;
    Ok(hash)
}

pub fn hash_object(contents: &String, write: &bool) -> Result<String> {
    let mut hasher = Sha1::new();
    hasher.update(&contents);
    let result = hasher.finalize();

    // Convert hash digest in the form of GenericArray to hex string
    let hash: String = result.iter().map(|byte| format!("{:02x}", byte)).collect();

    if *write {
        let path = gyat::concat_path(gyat::DIROBJPATH, &hash);
        let mut file = File::create(path).expect("Please run gyat init first");
        file.write_all(contents.as_bytes())?;
        file.flush()?;
    }

    Ok(hash)
}
