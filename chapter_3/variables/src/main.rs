fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable
    x = 6;
    println!("The value of x is: {}", x);
}
