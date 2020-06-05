use std::net::TcpListener;

fn main() {
    // 7878 is the default http port, also it is how you spell rust on a 10 digit keypad
    // our TcpListner will establish the connection string and wait for incoming requests
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // process the incoming requests here
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
