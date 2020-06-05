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

fn handle_connection(mut stream: TcpStream) {
    // stack buffer to pull data from the stream
    let mut buffer = [0; 512];

    // read bytes from the string and put them in the buffer
    stream.read(&mut buffer).unwrap();

    // convert the bytes into a string from the buffer
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
