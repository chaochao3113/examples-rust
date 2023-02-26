fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I`m throwing that snake in a fire."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaa!!!");
    }

    println!("I love {}s", inside);
}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

fn main() {
    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    if let Some(next_birth) = next_birthday(Some(10)) {
        println!("{}", next_birth);
    };

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}