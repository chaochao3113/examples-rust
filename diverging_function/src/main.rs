use std::process::exit;

fn foo() -> ! {
    panic!("This call never returns.");
}

fn some_fn() {
    ()
}

fn main() {
    let _a = some_fn();
    println!("this fn returns.");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9(excluding): {}", sum_odd_numbers(9));

    let _x = foo();
    println!("never returns.");
}
