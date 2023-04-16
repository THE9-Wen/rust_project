fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_param(11);
    another_function_with_params(13, 14);

    let x = five();
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    // dont add ;
    5
}

fn another_function() {
    println!("Another Function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_with_params(x: i32, y: i32) {
    println!("The values are: x = {}, y = {}", x, y);
}