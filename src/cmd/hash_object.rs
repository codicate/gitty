use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub fn main(path: &str, print: &bool) -> () {
    let hash = store_file_as_object(path).unwrap();

    if *print {
        println!("{}", hash)
    }
}

pub fn hash_content(content: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let result = hasher.finalize();

    // Convert hash digest in the form of GenericArray to hex string
    let hash: String = result.iter().map(|byte| format!("{:02x}", byte)).collect();
    hash
}

pub fn write_content_to_file<S: AsRef<Path>>(content: &String, path: S) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn store_content_as_object(content: &String) -> Result<String> {
    let hash = hash_content(content);
    let path = gyat::concat_path(gyat::DIROBJPATH, &hash);
    write_content_to_file(content, &path).expect("Please run gyat init first");
    Ok(hash)
}

pub fn store_file_as_object(path: &str) -> Result<String> {
    let content = gyat::get_file_content(path).unwrap();
    store_content_as_object(&content)
}
