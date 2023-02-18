use std::fmt::{Display, Debug};

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct S<T: Display>(T);

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: T) -> f64 {
    t.area()
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}

impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    // let s = S(vec![1]);
    let rectangle = Rectangle {
        length: 1.0,
        height: 2.0
    };

    let _triangle = Triangle {
        length: 2.0,
        height: 3.0
    };
    print_debug(&rectangle);
    println!("{}", area(rectangle));


    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}