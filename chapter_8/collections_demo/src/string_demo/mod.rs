pub fn string_init() {
    let mut s = String::new();
    // let data = "initial contents";

    s.push_str("Hello");

    for c in s.chars() {
        println!("{}", c);
    }

    for b in s.bytes() {
        println!("{}", b);
    }
}
