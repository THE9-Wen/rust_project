use std::fmt::Display;

fn main() {
    let s1 = String::from("Hello world!");
    let result;
    {
        let s2 = String::from("abc");
        result = witch_string_is_longer(s1.as_str(), s2.as_str());
        println!("The longer string is: {}", result);
        witch_string_is_longer_with_print(s1.as_str(), s2.as_str(), "abc");
    }
    // println!("The longer string is: {}", result);
}

fn witch_string_is_longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn witch_string_is_longer_with_print<'a, T>(
    s1: &'a str,
    s2: &'a str,
    ann: T
) -> &'a str
where T: Display
{
    println!("{}", ann);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
