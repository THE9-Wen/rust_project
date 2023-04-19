pub fn vec_init() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v1 = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let third = &v1[2];
    println!("The third element is {}", third);
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("The third element is not found"),
    }

    // panic
    // let does_not_exist = &v1[100];
    // None
    // let does_not_exist = v1.get(100);

    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v1 {
        *i += 1;
    }

    for i in &v1 {
        println!("{}", i);
    }

    println!("The Last element is {}", match v1.pop() {
        Some(value  ) => value,
        None => 0,
    });
}
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

impl SpreadsheetCell {
    pub(crate) fn display(&self) -> String {
        match self {
            SpreadsheetCell::Int(i) => i.to_string(),
            SpreadsheetCell::Float(f) => f.to_string(),
            SpreadsheetCell::Text(t) => t.to_string(),
        }
    }
}

pub fn different_type_in_vec() -> Vec<SpreadsheetCell> {
    vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.0),
        SpreadsheetCell::Text(String::from("Hello")),
    ]
}
