#![allow(dead_code, unused_variables)]
use std::thread;
use std::time::Duration;

fn main() { 
    // simple_threads();
    // simple_threads2();
    simple_threads3();
}

fn simple_threads() { // There is no quarantee on the order in which threads are run
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..10 {
        println!("Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1000));
    } // If the main for -loop ends before the one spawned, the spawned thread is also closed
}

fn simple_threads2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle is a JoinHandle which is an owned value
    // When we call join method on it, this program will wait for its thread to finish
    handle.join().unwrap();
}

fn simple_threads3() {

    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}