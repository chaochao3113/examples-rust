fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    let Person { name, ref age } = person;

    println!("The person`s age is {}", age);

    println!("The person`s name is {}", name);

    // println!("The person struct is {:?}", person);

    println!("{}", person.age);
}