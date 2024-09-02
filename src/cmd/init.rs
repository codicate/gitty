use std::io::{Error, ErrorKind};
use std::path::Path;
use std::{env, fs};

pub fn main() -> std::io::Result<()> {
    let path = Path::new(gyat::DIRPATH);
    if path.is_dir() {
        return Err(Error::new(
            ErrorKind::Other,
            "This repository has already been initialized",
        ));
    }

    fs::create_dir(path)?;
    fs::create_dir(gyat::DIROBJPATH)?;

    let cwd = env::current_dir()?;
    println!(
        "Initialized empty gyat repository in {}\\.gyat",
        cwd.display()
    );
    Ok(())
}
