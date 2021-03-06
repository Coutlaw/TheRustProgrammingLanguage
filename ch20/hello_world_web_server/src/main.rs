use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use thread_pool::ThreadPool;

fn main() {
    // 7878 is the default http port, also it is how you spell rust on a 10 digit keypad
    // our TcpListner will establish the connection string and wait for incoming requests
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // our custom thread pool impl
    let pool = ThreadPool::new(4);

    // process the incoming requests here
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // bad implementation here, we are creating limitless threads for every request incoming
        pool.execute(|| { handle_connection(stream) });
    }
}

// filesystem module
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    // stack buffer to pull data from the stream
    let mut buffer = [0; 512];

    // read bytes from the string and put them in the buffer
    stream.read(&mut buffer).unwrap();

    // b" " is to turn string into byte code
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    // read file as string
    let contents = fs::read_to_string(filename).unwrap();

    // format the string onto the HTTP response
    let response = format!("{}{}", status_line, contents);

    // we can send bytes downstream on the connection with .write(&[u8])
    stream.write(response.as_bytes()).unwrap(); // unwrap because .write could return an error
                                                // flush() will wait for all bytes to be written before continuing in the program
    stream.flush().unwrap();
}
