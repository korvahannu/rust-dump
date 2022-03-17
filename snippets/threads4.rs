use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        // num becomes a mutable reference to m
        // mutex is yet another smart pointer
        // After borrowing the value we must lock it to prevent other threads from using it
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // Multiple ownerships
    // Arc is kind of like RefCell and Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
