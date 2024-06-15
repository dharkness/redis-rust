use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut reader = BufReader::new(_stream.try_clone().expect("clone failed"));

                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(len) => {
                            if len == 0 {
                                break;
                            }
                            println!("received: {}", line.trim());
                            if len >= 4 && &line[..4] == "PING" {
                                _stream.write_all(b"+PONG\r\n").expect("write failed");
                            }
                        }
                        Err(e) => {
                            println!("error reading: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
