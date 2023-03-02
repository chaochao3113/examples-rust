fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0{
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{}/{}failed", dividend, divisor),
        Some(q) => println!("{}/{}={}", dividend, divisor, q),
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // 绑定 `None` 到一个变量需要类型标注
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // 解包 `Some` 将取出被包装的值。
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // 解包 `None` 将会引发 `panic!`。
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}