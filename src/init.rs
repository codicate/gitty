use std::io::Result;
use std::{env, fs};

pub fn main() {
    match init() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn init() -> Result<()> {
    let dir_path = ".gyat";

    fs::create_dir(dir_path)?;
    fs::create_dir(".gyat/objects")?;

    let cwd = env::current_dir()?;
    println!(
        "Initialized empty gyat repository in {}\\.gyat",
        cwd.display()
    );
    Ok(())
}
