use crate::cmd::tag;
use std::{fs, io::Result};

pub fn main() {
    let branches = get_branches().unwrap();
    let cur_branch = tag::get_head().unwrap();

    for branch in branches {
        print!("{}", branch);
        if branch == cur_branch {
            print!(" (current)");
        }
        println!();
    }
}

fn get_branches() -> Result<Vec<String>> {
    let branches = fs::read_dir(gitty::HEADPATH)?;
    let mut branch_list = Vec::new();

    for branch in branches {
        let branch_name = branch?.file_name().into_string().unwrap();
        if branch_name != "HEAD" {
            branch_list.push(branch_name);
        }
    }

    Ok(branch_list)
}
