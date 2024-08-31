use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::{env, fs};

pub fn main() {
    match init() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn init() -> Result<()> {
    let path = Path::new(".gyat");
    if path.is_dir() {
        return Err(Error::new(
            ErrorKind::Other,
            "This repository has already been initialized",
        ));
    }

    fs::create_dir(path)?;
    fs::create_dir(".gyat/objects")?;

    let cwd = env::current_dir()?;
    println!(
        "Initialized empty gyat repository in {}\\.gyat",
        cwd.display()
    );
    Ok(())
}
