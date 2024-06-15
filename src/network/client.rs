use std::io;
use std::io::{Read, Write};
use std::str::from_utf8;

use mio::{Interest, Registry, Token};
use mio::event::Source;
use mio::net::TcpStream;

use crate::parse::Parser;
use crate::storage::Store;

use super::{interrupted, would_block};

pub struct Client {
    token: Token,
    stream: TcpStream,
    incoming_end: usize,
    incoming: Vec<u8>,
    outgoing: Vec<u8>,
}

impl Client {
    pub fn new(token: Token, stream: TcpStream) -> Self {
        Self {
            token,
            stream,
            incoming: vec![0; 1024],
            incoming_end: 0,
            outgoing: Vec::with_capacity(4 * 1024),
        }
    }

    pub fn start(&mut self, registry: &Registry) -> io::Result<()> {
        self.stream
            .register(registry, self.token, Interest::READABLE)
    }

    pub fn receive(&mut self, registry: &Registry) -> io::Result<bool> {
        loop {
            println!(
                "reading up to {} bytes",
                self.incoming.len() - self.incoming_end
            );
            match self.stream.read(&mut self.incoming[self.incoming_end..]) {
                Ok(0) => {
                    println!("connection closed");

                    self.stream.shutdown(std::net::Shutdown::Both)?;
                    self.stream.deregister(registry)?;
                    return Ok(true);
                }
                Ok(n) => {
                    println!("read {} bytes", n);

                    self.incoming_end += n;
                    if self.incoming_end == self.incoming.len() {
                        self.incoming.resize(self.incoming.len() + 1024, 0);
                    }
                }
                Err(ref err) if would_block(err) => {
                    println!(
                        "read {}",
                        from_utf8(&self.incoming[..self.incoming_end]).unwrap()
                    );

                    return Ok(false);
                }
                Err(ref err) if interrupted(err) => {
                    println!("interrupted");
                }
                Err(err) => return Err(err),
            };
        }
    }

    pub fn run_commands(
        &mut self,
        parser: &Parser,
        store: &mut Store,
        registry: &Registry,
    ) -> io::Result<()> {
        let mut index = 0;

        while index < self.incoming_end {
            match parser.try_next_input(&self.incoming[index..self.incoming_end]) {
                Ok(Some((input, len))) => {
                    index += len;
                    match parser.try_parse_command(input) {
                        Ok(command) => {
                            command.apply(store, self, registry)?;
                        }
                        Err(err) => {
                            self.write_simple_error(&err, registry)?;
                            break;
                        }
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(err) => {
                    self.write_simple_error(&err, registry)?;
                    break;
                }
            }
        }

        self.incoming.copy_within(index..self.incoming_end, 0);
        self.incoming_end -= index;

        Ok(())
    }

    pub fn write(&mut self, data: &[u8], registry: &Registry) -> io::Result<()> {
        self.outgoing.extend_from_slice(data);
        self.stream.reregister(
            registry,
            self.token,
            Interest::READABLE | Interest::WRITABLE,
        )
    }

    pub fn write_string(&mut self, s: String, registry: &Registry) -> io::Result<()> {
        self.write(s.as_bytes(), registry)
    }

    pub fn write_null(&mut self, registry: &Registry) -> io::Result<()> {
        self.write(b"_\r\n", registry)
    }

    pub fn write_ok(&mut self, registry: &Registry) -> io::Result<()> {
        self.write(b"+OK\r\n", registry)
    }

    pub fn write_simple_error(&mut self, error: &str, registry: &Registry) -> io::Result<()> {
        self.write_string(format!("-{}\r\n", error), registry)
    }

    pub fn write_bulk_error(&mut self, error: &str, registry: &Registry) -> io::Result<()> {
        self.write_string(format!("!{}\r\n{}\r\n", error.len(), error), registry)
    }

    pub fn write_simple_string(&mut self, value: &str, registry: &Registry) -> io::Result<()> {
        self.write_string(format!("+{}\r\n", value), registry)
    }

    pub fn write_bulk_string(&mut self, value: &str, registry: &Registry) -> io::Result<()> {
        self.write_string(format!("${}\r\n{}\r\n", value.len(), value), registry)
    }

    pub fn write_integer(&mut self, value: i64, registry: &Registry) -> io::Result<()> {
        self.write_string(format!(":{}\r\n", value), registry)
    }

    pub fn write_array(&mut self, values: &[&str], registry: &Registry) -> io::Result<()> {
        self.write_string(format!("*{}\r\n", values.len()), registry)?;
        for value in values {
            self.write_bulk_string(value, registry)?;
        }
        Ok(())
    }

    pub fn write_empty_array(&mut self, registry: &Registry) -> io::Result<()> {
        self.write(b"*0\r\n", registry)
    }

    pub fn send(&mut self, registry: &Registry) -> io::Result<bool> {
        let mut bytes_sent = 0;
        let bytes_total = self.outgoing.len();
        let mut bytes_left = bytes_total;

        while bytes_left > 0 {
            println!("writing up to {} bytes", bytes_left);
            match self.stream.write(&self.outgoing[bytes_sent..bytes_total]) {
                Ok(0) => {
                    println!("connection closed");

                    self.stream.deregister(registry)?;
                    return Ok(true);
                }
                Ok(n) => {
                    println!("wrote {} bytes", n);

                    bytes_sent += n;
                    bytes_left -= n;
                }
                Err(ref err) if would_block(err) => {
                    println!("wrote {}", from_utf8(&self.outgoing[..bytes_sent]).unwrap());
                    if bytes_left > 0 {
                        println!("did not write {} bytes", bytes_left);
                        self.outgoing.copy_within(bytes_sent..bytes_total, 0);
                        self.outgoing.truncate(bytes_left);
                    }

                    return Ok(false);
                }
                Err(ref err) if interrupted(err) => {
                    println!("interrupted");
                }
                Err(err) => return Err(err),
            };
        }

        println!("wrote {}", from_utf8(&self.outgoing).unwrap());
        self.outgoing.truncate(0);

        // done writing
        self.stream.flush()?;
        self.stream
            .reregister(registry, self.token, Interest::READABLE)?;
        Ok(false)
    }
}
