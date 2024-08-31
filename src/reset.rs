use std::fs;
use std::io::Result;

pub fn main() {
    match reset() {
        Ok(_) => println!("Reset successful"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn reset() -> Result<()> {
    let dir_path = ".gyat";

    // Recursively delete the directory and its contents
    fs::remove_dir_all(dir_path)?;

    Ok(())
}
