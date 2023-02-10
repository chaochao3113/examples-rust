fn main() {
    use std::mem;

    let color = String::from("green");

    // borrow &color
    let print = || println!("`color`: {}", color);

    print();

    //allow many immutalbe variables exist
    let _reborrow = &color; // borrow &color
    print(); // borrow &color

    let _color_move = color;

    // err after moved
    // print();

    let mut count = 0;

    // borrow &mut count
    let mut inc = || {
        count+=1;
        println!("count: {}", count);
    };

    inc();

    //not allow a mutable variable exist with many immutable or mutable variables
    // let _borrow = &count;
    inc();

    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };


    consume();

    // consume();


    let haystack = vec![1, 2, 3];

    // borrow  &haystack
    let contains = | needle | {
        haystack.contains(needle)
    };


    println!("{}", contains(&1));
    println!("{}", contains(&5));

    println!("{:?}", haystack);



    let haystack = vec![1, 2, 3];

    // force move to closure
    let contains = move | needle | {
        haystack.contains(needle)
    };

    println!("{}", contains(&1));
    println!("{}", contains(&5));
    
    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}
