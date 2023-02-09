fn main() {
    let optional = Some(7);

    match optional {
        Some(x) => println!("{}", x),
        _ => todo!(),
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(x) = number {
        println!("number is {}", x);
    }

    if let Some(x) = letter {
        println!("letter is {}", x);
    } else {
        println!("can`t match");
    }

    let i_like_letters = false;

    if let Some(x) = emoticon {
        println!("emoticon is {}", x);
    } else if i_like_letters {
        println!("i like letters");
    } else {
        println!("i dont`t like letters and can`t match");
    }

    let a = Foo::Bar;
    let b = Foo::Bar;
    let c = Foo::Qux(10);

    if let Foo::Bar = a {
        println!("a is foo bar")
    }

    if let Foo::Bar = b {
        println!("b is foo bar")
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value)
    }

    let a = Foo::Bar;

    // 变量匹配 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}