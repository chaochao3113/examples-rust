fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 
    where F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // 只打印  借用即可满足 所以闭包是Fn
        println!("I said {}", greeting);

        // 改变了farewell， 所以需要闭包是FnMut
        farewell.push_str("!!!");

        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用drop  所以需要闭包是FnOnce
        mem::drop(farewell);
    };


    apply(diary);

    let double = |x| 2*x;

    println!("3 doubled: {}", apply_to_3(double));
}
