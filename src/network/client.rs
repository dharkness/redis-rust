use std::collections::HashSet;
use std::hash::Hasher;
use std::io;
use std::io::{Read, Write};
use std::str::from_utf8;

use mio::{Interest, Registry, Token};
use mio::event::Source;
use mio::net::TcpStream;

use crate::parse::Parser;
use crate::storage::{Store, Value};

use super::{interrupted, Response, would_block};
use super::error::Error;

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
                        Ok(command) => match command.apply(store) {
                            Ok(response) => {
                                self.write_response(&response)?;
                            }
                            Err(error) => {
                                self.write_error(&error)?;
                                break;
                            }
                        },
                        Err(error) => {
                            self.write_error(&error)?;
                            break;
                        }
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(error) => {
                    self.write_error(&error)?;
                    break;
                }
            }
        }

        self.incoming.copy_within(index..self.incoming_end, 0);
        self.incoming_end -= index;

        self.stream.reregister(
            registry,
            self.token,
            Interest::READABLE | Interest::WRITABLE,
        )?;

        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.outgoing.extend_from_slice(data);
        Ok(())
    }

    pub fn write_response(&mut self, response: &Response) -> io::Result<()> {
        match response {
            Response::Null => self.write(b"_\r\n"),
            Response::Ok => self.write(b"+OK\r\n"),
            Response::Zero => self.write(b":0\r\n"),
            Response::One => self.write(b":1\r\n"),
            Response::False => self.write(b"#f\r\n"),
            Response::True => self.write(b"#t\r\n"),
            Response::EmptyBulkString => self.write(b"$0\r\n\r\n"),
            Response::EmptyList => self.write(b"*0\r\n"),
            Response::EmptyMap => self.write(b"%0\r\n"),
            Response::EmptySet => self.write(b"~0\r\n"),
            Response::I64(value) => self.write_integer(*value),
            Response::Usize(value) => self.write_usize(*value),
            Response::Raw(data) => self.write(data),
            Response::SimpleString(s) => self.write_simple_string(s),
            Response::BulkString(s) => self.write_bulk_string(s),
            Response::List(list) => self.write_list(list),
            Response::Set(set) => self.write_set(set),
            Response::Value(value) => self.write_value(value),
            Response::ValueRef(value) => self.write_value(value),
            Response::ValueList(list) => {
                self.write_string(format!("*{}\r\n", list.len()))?;
                for value in list {
                    self.write_value(value)?;
                }
                Ok(())
            }
        }
    }

    pub fn write_value(&mut self, value: &Value) -> io::Result<()> {
        match value {
            Value::List(values) => self.write_list(values),
            Value::Integer(i) => self.write_integer(*i),
            Value::Set(members) => self.write_set(members),
            Value::String(s) => self.write_bulk_string(s),
        }
    }

    pub fn write_error(&mut self, error: &Error) -> io::Result<()> {
        match error {
            Error::Raw(data) => self.write(data),
            Error::String(s) => self.write_simple_string(s),
            Error::Protocol => self.write(b"-broken protocol\r\n"),
            Error::UnknownCommand(command) => {
                self.write_string(format!("-unknown command '{}'\r\n", command))
            }
            Error::UnknownOption(command, option) => self.write_string(format!(
                "-unknown option '{}' for command '{}'\r\n",
                option, command
            )),
            Error::DuplicateOption(command, option) => self.write_string(format!(
                "-duplicate option '{}' for command '{}'\r\n",
                option, command
            )),
            Error::MissingArgument(command, arg) => self.write_string(format!(
                "-missing argument '{}' for command '{}'\r\n",
                arg, command
            )),
            Error::Syntax => self.write(b"-syntax error\r\n"),
            Error::Integer => self.write(b"-value is not an integer or out of range\r\n"),
            Error::ExpireTime => self.write(b"-invalid expire time\r\n"),
            Error::KeyNotFound => self.write(b"-no such key\r\n"),
            Error::WrongType => self
                .write(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n"),
        }
    }

    pub fn write_string(&mut self, s: String) -> io::Result<()> {
        self.write(s.as_bytes())
    }

    pub fn write_simple_string(&mut self, value: &str) -> io::Result<()> {
        self.write_string(format!("+{}\r\n", value))
    }

    pub fn write_bulk_string(&mut self, value: &str) -> io::Result<()> {
        self.write_string(format!("${}\r\n{}\r\n", value.len(), value))
    }

    pub fn write_integer(&mut self, value: i64) -> io::Result<()> {
        self.write_string(format!(":{}\r\n", value))
    }

    pub fn write_usize(&mut self, value: usize) -> io::Result<()> {
        self.write_string(format!(":{}\r\n", value))
    }

    pub fn write_list(&mut self, values: &[String]) -> io::Result<()> {
        self.write_string(format!("*{}\r\n", values.len()))?;
        for value in values {
            self.write_bulk_string(value)?;
        }
        Ok(())
    }

    pub fn write_set(&mut self, members: &HashSet<String>) -> io::Result<()> {
        self.write_string(format!("~{}\r\n", members.len()))?;
        for member in members {
            self.write_bulk_string(member)?;
        }
        Ok(())
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
