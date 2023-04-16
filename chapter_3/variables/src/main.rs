fn main() {
    let x = 5;
    // let mut x = 5;
    let x = x + 1;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable
    let x = x * 2;
    // x = 6;
    println!("The value of x is: {}", x);

    const NAME: &str = "Hello World!";
    println!("The value of NAME is: {}", NAME);
}
