use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// impl <T> PrintInOption for T where
//     Option<T>: Debug {
//     fn print_in_option(self) {
//         println!("{:?}", Some(self));
//     }
// }

impl <S> PrintInOption for S where S: Debug {
    fn print_in_option(self) {
        println!("{:?}", self);
    }
}

// #[derive(Debug)]
struct Text {
    a: i32,
    l: String,
    h: Vec<i32>
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();

    let t = Text {
        a: 1,
        h: vec![1, 2, 3],
        l: String::from("value")
    };

    // t.print_in_option();
}