use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Error opening file: {}", err);
    //     },
    // };
}
