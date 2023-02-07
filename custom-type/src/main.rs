#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    (rect.top_left.y - rect.bottom_right.y) * (rect.bottom_right.x - rect.top_left.x)
}

fn square(point: Point, length: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..point },
        bottom_right: Point {
            x: point.x + length,
            y: point.y - length,
        },
    }
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y: i64}
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("press the key: {}", c),
        WebEvent::Paste(s) => println!("pasted {}", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}



fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };

    println!("{}", rect_area(rect));

    println!("{:#?}", square(Point { x: 0.0, y: 10.0 }, 10.0));

    let pressed = WebEvent::KeyPress('x');

    let pasted = WebEvent::Paste("my text".to_owned());

    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;

    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;

    let work = Civilian;

    match status {
        Poor => println!("the poor have no money"),
        Rich => println!("the rich have lots of mony"),
    };

    match work {
        Civilian => println!("Civilians works!"),
        Soldier => println!("Soldiers fight!"),
    };

    println!("zero is {}", Number::Zero as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
