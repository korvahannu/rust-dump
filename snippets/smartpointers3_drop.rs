use std::mem::drop;

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // Any code that you want to run before removing data from memory will be put here
        // This is a destructor
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    let x = CustomSmartPointer {
        data: String::from("My new data 1")
    };
    let y = CustomSmartPointer {
        data: String::from("My new data 2")
    };
    let z = CustomSmartPointer {
        data: String::from("My new data 3")
    };

    drop(z); // Lets drop z just for memes

    // Note how variables are dropped in the reverse order of their creation
    println!("Created data.");
}
