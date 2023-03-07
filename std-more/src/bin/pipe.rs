use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

fn main() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(e) => panic!("couldn`t spawn wc: {:?}", e),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(e) => panic!("couldn`t write to wc stdin: {:?}", e),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(e) => panic!("couldn`t read wc stdout: {:?}", e),
        Ok(_) => println!("wc responded with:\n{}", s),
    }
}