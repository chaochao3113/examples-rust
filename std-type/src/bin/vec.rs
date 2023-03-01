fn main() {
    let ci: Vec<i32> = (0..10).collect();
    println!("{:?}", ci);

    let mut xs = vec![1i32, 2, 3];
    println!("{:?}", xs);

    xs.push(4);

    // ci.push(0);

    println!("{}", xs.len());

    println!("{}", xs[1]);

    println!("{:?}", xs.pop());

    // println!("{}", xs[3]);

    for x in xs.iter() {
        println!("{}", x);
    }

    for x in xs.iter_mut() {
        *x *=3; 
    }

    println!("{:?}", xs);
}