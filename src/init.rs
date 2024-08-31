use std::fs;

pub fn main() {
    match init() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn init() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = ".gyat";

    fs::create_dir(dir_path)?;
    fs::create_dir(".gyat/objects")?;

    Ok(())
}
