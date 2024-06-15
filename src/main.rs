use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    println!("listening for connections");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    let mut buf = [0; 1024];

                    loop {
                        match stream.read(&mut buf).await {
                            Ok(len) => {
                                if len == 0 {
                                    break;
                                }
                                // if len >= 4 && &buf[..4] == b"PING" {
                                    stream.write_all(b"+PONG\r\n").await.expect("write failed");
                                // }
                            }
                            Err(e) => {
                                println!("error reading: {}", e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
