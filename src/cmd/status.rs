use crate::cmd::tag;

pub fn main() {
    let cur_branch = tag::get_head();
    match cur_branch {
        Ok(cur_branch) => println!("Currently on branch '{}'", cur_branch),
        Err(_) => println!("You do not have any commits yet"),
    }
}
