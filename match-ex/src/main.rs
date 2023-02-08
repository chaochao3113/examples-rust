fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain`t special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    let triple = (0, -2, 3);
    println!("Tell me about: {:?}", triple);
    match triple {
        (0, y, z) => println!("{:?}, {:?}", y, z),
        (1, ..) => println!("One"),
        _ => println!("none"),
    }

    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => println!("{}, {}, {}", r, g, b),
        Color::HSV(h, s, v) => println!("{}, {}, {}", h, s, v),
    }

    let reference = &4;

    match reference {
        &val => println!("{}", val),
    }

    match *reference {
        val => println!("{}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref val => println!("{}", val),
    }

    match mut_value {
        ref mut val => {
            *val += 10;
            println!("{}", val);
        }
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }


    let foo = Foo {
        x: (1, 1),
        y: 1,
    };

    let Foo { x: (a, b), y } = foo;

    println!("{}, {}, {}", a, b, y);

    let Foo{y: i, x: j} = foo;
    println!("{}, {:?}", i, j);

    let Foo{y, ..} = foo;

    println!("{}", y);


    let pair = (2, -2);

    match pair {
        (x, y) if x == y => println!("x == y"),
        (x, y) if x + y == 0 => println!("x + y = 0"),
        (x, _) if x % 2 == 0 => println!("x % 2 = 0"),
        _ => println!("No correlation"),
    }

    match age() {
        0 => println!("0"),
        n @ 1 ..=12 => println!("child {}", n),
        n @ 13 ..=19 => println!("teen {}", n),
        n => println!("old, {}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("42 is {}", n),
        Some(b) => println!("other is {}", b),
        None => todo!(),
    }
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(32)
}