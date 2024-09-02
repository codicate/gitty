use std::{fs::File, io::Result, path::PathBuf};

pub fn main(hash: &str) -> Result<()> {
    let mut path = PathBuf::from(".gyat/objects");
    path.push(hash);
    let file = File::open(path).expect("Invalid hash provided");
    let ignored_files = gyat::get_ignored_file_list();

    Ok(())
}
