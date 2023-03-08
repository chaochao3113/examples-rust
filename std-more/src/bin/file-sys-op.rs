use std::{fs, fs::File, fs::OpenOptions, io::{self, Read, Write}, os::unix, path::Path};

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`mkdir a`");
    match fs::create_dir("a") {
        Err(e) => println!("! {:?}", e.kind()),
        Ok(_) => {},
    }

    println!("`echo hello > a/b.txt`");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|e| println!("! {:?}", e.kind()));

    println!("`mkdir -p a/c/d`");
    fs::create_dir_all("a/c/d").unwrap_or_else(|e| println!("! {:?}", e.kind()));

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|e| println!("! {:?}", e.kind()));

    println!("`ln -s ../b.txt a/c/b.txt`");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|e| println!("! {:?}", e.kind()));
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(e) => println!("! {:?}", e.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    match fs::read_dir("a") {
        Err(e) => println!("! {:?}", e.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        }
    }

    println!("`rm a/c/e.txt`");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|e| println!("! {:?}", e.kind()));

    println!("`rmdir a/c/d`");
    fs::remove_dir("a/c/d").unwrap_or_else(|e| println!("! {:?}", e.kind()));
}
