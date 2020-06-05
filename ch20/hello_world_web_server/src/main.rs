use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // 7878 is the default http port, also it is how you spell rust on a 10 digit keypad
    // our TcpListner will establish the connection string and wait for incoming requests
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // process the incoming requests here
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// filesystem module
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    // stack buffer to pull data from the stream
    let mut buffer = [0; 512];

    // read bytes from the string and put them in the buffer
    stream.read(&mut buffer).unwrap();

    // read file as string
    let contents = fs::read_to_string("hello.html").unwrap();

    // format the string onto the HTTP response
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    // we can send bytes downstream on the connection with .write(&[u8])
    stream.write(response.as_bytes()).unwrap(); // unwrap because .write could return an error
    // flush() will wait for all bytes to be written before continuing in the program
    stream.flush().unwrap();
}
