pub fn main(hash: &str) -> () {
    let contents = gitty::get_object_content(hash).unwrap();
    println!("{}", contents);
}
