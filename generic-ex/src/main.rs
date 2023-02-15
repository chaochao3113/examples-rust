#[derive(Debug)]
struct A;

#[derive(Debug)]
struct Single(A);

#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);

    println!("{:?}", _s);
    let _char: SingleGen<char> = SingleGen('a');

    println!("{:?}", _char);
    let _t = SingleGen(A);
    println!("{:?}", _t);
    let _i32 = SingleGen(6);
    println!("{:?}", _i32);
    let _char = SingleGen('a');
    println!("{:?}", _char);
}
