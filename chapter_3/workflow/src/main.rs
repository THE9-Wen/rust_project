fn main() {
    // let number = if_function();
    // println!("The value of number is: {}", number);
    //
    // loop_function();
    // for_function();
    // while_function();
    //
    // let fah = cel_to_fah(32.0);
    // println!("32.0 cel equals to {} fah", fah);
    // let cel = fah_to_cel(108.0);
    // println!("108.0 fah equals to {} cel", cel);
    fibonacci(10);
}

fn if_function() -> i32 {
    let number = 3;
    if number < 5 {
        println!("number < 5");
    } else {
        println!("number >= 5");
    }

    let condition = true;
    // must be same type in different branch
    let number = if condition {
        5
    } else {
        6
    };
    number
}

fn loop_function() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break;
        }
        println!("Loop for {} times.", counter);
    }
}

fn for_function() {
    // for i in 0..6 {
    //     println!("{}.", i);
    // }

    let a = [2, 4, 6, 8];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn while_function() {
    let mut number = 3;
    while number != 0 {
        number -= 1;
        println!("{}!", number);
    }
}

fn cel_to_fah(cel: f64) -> f64 {
    1.8 * cel + 32.0
}

fn fah_to_cel(fah: f64) -> f64 {
    (fah - 32.0) / 1.8
}

fn fibonacci(x: i32) {
    let mut a = 1;
    let mut b = 0;
    let mut temp = b;
    for i in 0..x {
        println!("Index {}: {}", i, a);
        temp = a;
        a = a + b;
        b = temp;
    }
}
