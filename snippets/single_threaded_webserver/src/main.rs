use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    // Create a new TcpListener
    // instead of ::new it is ::bind because in networking
    // connecting to a port to listen is known as "binding to a port"
    // Returns Results<T, E>
    let listener = TcpListener::bind(ADDRESS).unwrap();

    // ITerate over connection attempts
    for stream in listener.incoming() {
        // Program will panic if any stream errors
        let stream = stream.unwrap();

        handle_connection(stream);

        println!("{}");


    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create a buffer of 1024 bytes in size
    // It should be enough to hold the data of basic request
    // This is basically an array of 0's with a length of 1024
    // If we wanted to handle reqests of an arbitary size, buffer
    // management would not be so easy
    let mut buffer = [0; 1024];

    // stream.read reads bytes from tcpstream and puts them inside buffer
    stream.read(&mut buffer).unwrap();

    // Because we read raw bytes in the buffer, transform get to bytes
    // By adding a b before the string
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "response.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // String::from_utf8_lossy takes a &[u8] and procudes a string rom it
    // Lossy part indicates that this function will replace invalid utf8-sequences
    // with �
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Get the HTML file from project root
    let contents = fs::read_to_string(filename).unwrap();

    // format response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Write method takes a &[u8] and sends those bytes directly down the connection
    // On a real app you might want to check for errors
    stream.write(response.as_bytes()).unwrap();

    // flush will wait and prevent the program from continuing
    // untill all the bytes are written to the connection
    stream.flush().unwrap();
}
