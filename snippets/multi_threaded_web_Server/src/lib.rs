use std::thread;
// Remember: mpsc::channel will return a tuple of a sender and a receiver
// You can have multiple senders but only a single instance of a receiver
// Arc provides a way for us to have a references-counting pointer
// ==> Shared ownership of value of tpye T
// Arc does not allow mutation by default, so we wrap it inside
// Mutex, which provides a lock-system to make accessing it thread-safe
use std::sync::{mpsc, Arc, Mutex};

// ThreadPool struct, for the outsider you only have
// ThreadPool::new(usize) and ThreadPool.execute
// Workers contain a worker id and an Option<JoinHandle<()>
// We store the sending end of our mpsc inside the struct
// Sender is of type Sender<Message>, see Message-enum below
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

// The Message -enum was implemented so that our threads inside threadpool
// can shutdown gracefully
// Message::NewJob contains a box that holds a closure that is consumed upon use
// the closure implements Send -trait so we can move it between threads
// and a lifetime annotation of 'static
// We need 'static annotation, because we need to tell rust that this closure
// may outlive the function calling it
enum Message {
    NewJob(Box<dyn FnOnce() + Send + 'static>),
    Terminate
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // Panic if the size of pool is less than 0
        // Panicing here is appropriate, since running a threadpool with 0 threads
        // makes no sense
        assert!(size > 0);

        // Get the receiving and sending end of our mpsc
        let (sender, receiver) = mpsc::channel();

        // Wrap the receiver inside Arc(Mutex())
        let receiver = Arc::new(Mutex::new(receiver));

        // Vec::with_capacity(7) vs Vec::new(7)
        // with_capacity allocates the memory at the moment of calling
        // Making it safer in some instances
        let mut workers = Vec::with_capacity(7);

        for id in 0..size {
            // Iterate through 0..size and add x amount of workers to Vec<Worker>
            // Remember: We can use Arc::clone to have shared ownership
            // This is why we had to wrap receiver into arc and mutex
            // Simply using receiver.clone() without the wraps wouldn't work
            // Because Rust by default allows only multiple senders and a single receiver
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Return the threadpool
        ThreadPool {
            workers,
            sender
        }
    }

    // F is a closure that inherits FnOnce and send traits and has 'static lifetime
    // The parent function calls this and gives a closure as a parameter
    // That closure is Boxed and sent to the listeners by the sender
    // Which clone of receiver gets the job? We may never know
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

        // Send the terminate message for all workers
        // By sending workers.len() amount of terminate Messages
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // The reason we iterate here again instead of doing this in the loop above
        // Is because we can not quarantee that the terminating message has been received
        // before the code blow would be executed
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // Since we have sent a terminate -message, all threads should start closing gracefully
            // We take the thread out of each worker with .take() and leave None in replacement
            // and wait for them to close via thread.join().unwrap()
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

    }
}

// Worker struct contains an unique id
// and an Option<JoinHandle<()>>
// We need to wrap the thread handle inside option
// because otherwise it would be blank while we gracefully shutdown
// which rust does not allow
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {

            // First we use reciever.lock() to return a Result<T, E>
            // Then we unrwap it to retrieve the MutexGuard
            // Then we use .recv() to turn the MutexGuard into another result
            // Then we finally use unwrap to access the Message inside
            let message = receiver.lock().unwrap().recv().unwrap();

            // After we recieve a message, we compare it with match
            // Terminate signal will break the loop and shutdown the thread
            // NewJob will take the job inside the closure and execute it
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