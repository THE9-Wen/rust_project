fn main() {
    let s1 = String::from("Hello World!");
    // let s2 = s1;
    let s2 = s1.clone();

    // error when compile
    println!("The value of s1 is: {}", s1);
    println!("The value of s2 is: {}", s2);

    takes_ownership(s2);
    // error when compile
    // println!("s2 = {}", s2);

    let length = calculate_length(&s1);
    println!("length = {}", length);
    println!("s1 = {}", s1);

    // let r3 = &mut s1; // error
    let r1 = &s1; // ok
    let r2 = &s1; // ok

    let s = String::from("hello");
    let slice = &s[..2];
    println!("slice = {}", slice);

    let word = first_word(&s);
    println!("word = {}", word);
}

fn takes_ownership(some_string: String) {
    println!("s = {}", some_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
