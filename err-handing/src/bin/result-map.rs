use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    first.parse::<i32>().and_then(|first_number| {
        second.parse::<i32>().map(|second_number| {
            first_number * second_number
        })
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("err: {}", e),
    }
}

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply2(first: &str, second: &str) -> AliasedResult<i32> {
    first.parse::<i32>().and_then(|first_number| {
        second.parse::<i32>().map(|second_number| {
            first_number * second_number
        })
    })
}

fn multiply3(first: &str, second: &str) -> AliasedResult<i32> {
    let first_number = match first.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let second_number = match second.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn multiply4(first: &str, second: &str) -> AliasedResult<i32> {
    let first_number = first.parse::<i32>()?;

    let second_number = second.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);

    print(multiply2("10", "2"));
    print(multiply2("t", "2"));

    print(multiply3("10", "2"));
    print(multiply3("t", "2"));

    print(multiply4("10", "2"));
    print(multiply4("t", "2"));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    
    println!("The first doubled is {}", double_first(numbers));
    
    println!("The first doubled is {}", double_first(empty));
    // 错误1：输入 vector 为空
    
    println!("The first doubled is {}", double_first(strings));
    // 错误2：此元素不能解析成数字
}