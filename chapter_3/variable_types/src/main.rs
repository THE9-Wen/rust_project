fn main() {
    // -128..=127
    let x1: i8 = 127;
    println!("The value of x1 is: {}", x1);
    // 0..=255
    let x2: u8= 255;
    println!("The value of x1 is: {}", x2);
    // -32768..=32767
    let x3: i16 = 32767;
    println!("The value of x3 is: {}", x3);
    // 0..=65535
    let x4: u16 = 65535;
    println!("The value of x4 is: {}", x4);
    // -2147483648..=2147483647
    // default type
    let x5: i32 = 2147483647;
    println!("The value of x5 is: {}", x5);
    // 0..=4294967295
    let x6: u32 = 4294967295;
    println!("The value of x6 is: {}", x6);
    // -9223372036854775808..=9223372036854775807
    let x7: i64 = 9223372036854775807;
    println!("The value of x7 is: {}", x7);
    // -65536..=65535
    let x8: u64 = u64::MAX;
    println!("The value of x8 is: {}", x8);

    // float default type
    let y1: f64 = 1.0;

    let sum = 1 + 1;
    let difference = 5 - 1;
    let product = 4 * 30;
    let quotient = 56.8 / 32.2;
    let remainder = 43 % 5;

    // bool type
    let t = true;
    let f: bool = false;

    // char
    let x = 'c';
    let z = 'Z';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

}
