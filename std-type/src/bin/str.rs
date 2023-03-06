fn main() {
    let pangram: &'static str = "xxxx   xxxxx  xxxx ";
    println!("{}", pangram);

    for word in pangram.split_whitespace().rev() {
        println!("{}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(",");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    let be = "I`m writing \x52\x75\x73\x74!";
    println!("{}", be);

    let unicode_code = "\u{211D}";
    let char_name = "\"Double-struck capital R\"";

    println!("{}, {}", unicode_code, char_name);

    let long_str = "String literals
                can span multiple lines.
                The linebreak and indentation here ->\
                <- can be escaped too!";

    println!("{}", long_str);
}
