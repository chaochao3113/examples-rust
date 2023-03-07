use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn`t open {}: {:?}", display, e),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("couldn`t read {}: {:?}", display, e),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}
