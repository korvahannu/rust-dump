use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc = multiple producers, single reciever
    // tx means transmitter, rx reciever
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // send method returns Result<T, E>
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv blocks main threads execution and waits
    // let recieved = rx.recv().unwrap();

    // This for loop is active until the other thread closes
    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
