fn multiply(first: &str, second: &str) -> i32 {
    let first_number = first.parse::<i32>().unwrap();
    let second_number = second.parse::<i32>().unwrap();

    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}