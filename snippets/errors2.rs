#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { // We can change what main returns to Result<T,E>

    // Open a file and panic if it does not exist
    let f : File = match File::open("./src/test.txt") {
        Ok(file) => file, // returns a File handle
        Err(error) => panic!("Error: {:?}", error)
    };

    let file_string = "./src/does_it_exist.txt";

    let f : File = match File::open(file_string) { // File::open returns an enum Result<T, E>, lets match that!
        Ok(file) => file, // Return file handle
        Err(error) => { // Err contains type std::io::ErrorKind
            match error.kind() {
                ErrorKind::NotFound => match File::create(file_string)  {
                    Ok(fc) => fc,
                    Err(ec) => panic!("{}", ec),
                },
                other_error => {
                    panic!("Problem opening the file {:?}", other_error)
                }
            }
        }
    };

    println!("{}", read_file_test().unwrap());

    // unwrap returns either the panic! macro if no file is found Err
    // or returns the file handle on Ok(file)
    let f : File = File::open("exists.txt").unwrap();
    // Same as above, but panic! now has a message
    let f : File = File::open("exists.txt").expect("File not found");

    Ok(())
}

fn read_file_test() -> Result<String, io::Error> {
    let f = File::open("./src/test.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), //If file is not found exit function and return error
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_file_test2() -> Result<String, io::Error> {
    /*
        The ? works the same as match expect with less writing
        if File::open or read_to_string succeeds, the contents of Ok()
        is returned to in the statement. If the value is an Err()
        the Err will be returned from the whole function as if we had
        used return keyword => Error value gets propagated to the calling code
    */
    let mut f = File::open("./src/test.txt")?;
    let mut s =  String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_test3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("./src/test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_test4() -> Result<String, io::Error> {
    // Reading a file into string is common operation
    // So fs contains an associative function that does what it says
    fs::read_to_string("./src/test.txt")
}