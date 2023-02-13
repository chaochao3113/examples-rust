mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        pub constants: T,
    }

    impl <T> ClosedBox<T> {
        pub fn new(constants: T) -> ClosedBox<T> {
            ClosedBox { constants }
        }
    }
}

fn main() {
    let open_box = my::OpenBox{ contents: "public information" };
    println!("The open box contains: {}", open_box.contents);

    let _closed_box = my::ClosedBox::new("classified information");
}
