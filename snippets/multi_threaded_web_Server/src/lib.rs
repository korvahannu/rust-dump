use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

// Job is an alias for a type of a box that holds a thread
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // Panic if the size of pool is less than 0
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(7);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    // F is an object that inherits FnOnce and send traits
    // We need Send to transfer the closure from one thread to another
    // and 'static because we do not know how long the thread will take to execute
    pub fn execute<F: FnOnce() + Send + 'static>
    (&self, f: F) {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// For graceful shutdown we implement Drop for ThreadPool
impl Drop for ThreadPool {
    fn drop (&mut self) {
        
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got job; executing", id);

                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }

        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}