use std::collections::HashMap;

pub fn hash_map_init() {
    let mut map = HashMap::new();
    let s = "hello";
    let string = String::from("value");
    map.insert(s, string);
    println!("{}", s);
    // println!("{}", string);
}
