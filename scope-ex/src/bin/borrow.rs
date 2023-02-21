fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("{}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("{}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        eat_box_i32(boxed_i32);

        // borrow_i32(_ref_to_i32);
    }

    // eat_box_i32(boxed_i32);

    let immutabook = Book {
        author: "DDD",
        title: "DDD",
        year: 1979,
    };

    let mut mutabook = immutabook;

    // borrow_book(&immutabook);

    // borrow_book(&mutabook);

    new_edition(&mut  mutabook);

    new_edition(&mut mutabook);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &mut Book) {
    println!("{}-{}", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("{}-{}", book.title, book.year);
}
