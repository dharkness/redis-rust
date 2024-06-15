use std::io;
use std::io::{Read, Write};
use std::str::from_utf8;

use mio::net::TcpStream;
use mio::{Interest, Registry, Token};
use mio::event::Source;

use crate::{interrupted, would_block};
use crate::resp::{Command, Parser};

pub struct Client {
    token: Token,
    stream: TcpStream,
    outgoing: Vec<u8>,
    parser: Parser,
}

impl Client {
    pub fn new(token: Token, stream: TcpStream) -> Self {
        Self {
            token,
            stream,
            outgoing: Vec::with_capacity(4 * 1024),
            parser: Parser::new(),
        }
    }

    pub fn start(&mut self, registry: &Registry) -> io::Result<()> {
        self.stream.register(registry, self.token, Interest::READABLE)
    }

    pub fn receive(&mut self, registry: &Registry) -> io::Result<bool> {
        let mut buf = vec![0; 1024];
        let mut bytes_read = 0;

        loop {
            println!("reading up to {} bytes", buf.len() - bytes_read);
            match self.stream.read(&mut buf[bytes_read..]) {
                Ok(0) => {
                    println!("connection closed");

                    self.stream.shutdown(std::net::Shutdown::Both)?;
                    self.stream.deregister(registry)?;
                    return Ok(true);
                }
                Ok(n) => {
                    println!("read {} bytes", n);

                    bytes_read += n;
                    if bytes_read == buf.len() {
                        buf.resize(buf.len() + 1024, 0);
                    }
                }
                Err(ref err) if would_block(err) => {
                    println!("read {}", from_utf8(&buf[..bytes_read]).unwrap());

                    self.parser.add(from_utf8(&buf[..bytes_read]).unwrap());

                    return Ok(false);
                }
                Err(ref err) if interrupted(err) => {
                    println!("interrupted");
                }
                Err(err) => return Err(err),
            };
        }
    }

    pub fn try_parse_command(&mut self) -> Result<Option<Command>, String> {
        self.parser.try_parse_command()
    }

    pub fn write(&mut self, data: &[u8], registry: &Registry) -> io::Result<()> {
        self.outgoing.extend_from_slice(data);
        self.stream.reregister(registry, self.token, Interest::READABLE | Interest::WRITABLE)
    }

    pub fn send(&mut self, registry: &Registry) -> io::Result<bool> {
        let mut bytes_sent = 0;
        let mut bytes_left = self.outgoing.len();

        while bytes_left > 0 {
            println!("writing up to {} bytes", bytes_left);
            match self.stream.write(&self.outgoing[bytes_sent..self.outgoing.len()]) {
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
        self.stream.reregister(registry, self.token, Interest::READABLE)?;
        Ok(false)
    }
}
