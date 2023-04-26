use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let hi = String::from("hi");
    //     tx.send(hi).unwrap();
    // });
    //
    // let receive = rx.recv().unwrap();
    // println!("{receive}");

    thread::spawn(move || {
       let msg1 = String::from("msg1");
       let msg2 = String::from("msg2");
       let msg3 = String::from("msg3");
       let msg4 = String::from("msg4");

        let vec = vec![msg1, msg2, msg3, msg4];

        for msg in vec {
            thread::sleep(Duration::from_secs(1));
            tx.send(msg).unwrap();
        }
    });

    for msg in rx {
        println!("{msg}");
    }
}
