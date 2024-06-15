use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use mio::{Events, Interest, Poll, Token};
use mio::event::Event;
use mio::net::TcpListener;

use crate::parse::Parser;
use crate::store::Store;

use super::{Client, interrupted};

const SERVER: Token = Token(0);

pub struct Server {
    parser: Parser,
    store: Store,
    poll: Poll,
    last_token: Token,
    clients: HashMap<Token, Client>,
}

impl Server {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            parser: Parser::new(),
            store: Store::new(),
            poll: Poll::new()?,
            last_token: SERVER,
            clients: HashMap::new(),
        })
    }

    pub fn start(&mut self, addr: SocketAddr) -> io::Result<()> {
        let mut listener = TcpListener::bind(addr)?;
        let mut events = Events::with_capacity(128);

        self.poll
            .registry()
            .register(&mut listener, SERVER, Interest::READABLE)?;

        println!("listening for connections");

        loop {
            if let Err(err) = self.poll.poll(&mut events, None) {
                if interrupted(&err) {
                    continue;
                }
                return Err(err);
            }

            for event in events.iter() {
                match event.token() {
                    SERVER => loop {
                        let (stream, address) = match listener.accept() {
                            Ok((stream, address)) => (stream, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // no more incoming connections
                                break;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        };

                        println!("accepted connection from: {}", address);

                        let token = self.next_token();
                        let mut client = Client::new(token, stream);

                        client.start(self.poll.registry())?;
                        self.clients.insert(token, client);
                    },
                    token => {
                        let done = self.handle_event(token, event)?;

                        if done {
                            self.clients.remove(&token);
                        }
                    }
                }
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.last_token.0 += 1;
        Token(self.last_token.0)
    }

    /// Returns `true` if the connection was closed.
    fn handle_event(&mut self, token: Token, event: &Event) -> io::Result<bool> {
        let registry = self.poll.registry();

        if let Some(client) = self.clients.get_mut(&token) {
            if event.is_writable() {
                println!("writable");
                match client.send(registry) {
                    Ok(true) => return Ok(true),
                    Ok(false) => (),
                    Err(err) => return Err(err),
                };
            }

            if event.is_readable() {
                println!("readable");
                match client.receive(registry) {
                    Ok(true) => return Ok(true),
                    Ok(false) => {
                        self.store.expire_items();
                        match client.run_commands(&self.parser, &mut self.store, registry) {
                            Ok(()) => return Ok(false),
                            Err(err) => {
                                println!("error: {}", err);
                                return Ok(false);
                            }
                        }
                    }
                    Err(err) => return Err(err),
                };
            }

            Ok(false)
        } else {
            // ignore event for unknown token
            Ok(false)
        }
    }
}
