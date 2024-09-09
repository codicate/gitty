pub fn main(hash: &str) -> () {
    let contents = gyat::get_object_content(hash).unwrap();
    println!("{}", contents);
}
