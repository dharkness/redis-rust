#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    dead_code
)]

mod client;
mod commands;
mod input;
mod parser;
mod resp;
mod store;

use std::collections::HashMap;
use std::io;

use mio::event::Event;
use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Registry, Token};

use client::Client;
use parser::Parser;
use store::Store;

const SERVER: Token = Token(0);

fn main() -> io::Result<()> {
    let parser = Parser::new();
    let mut store = Store::new();

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:6379".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    let mut clients = HashMap::new();
    let mut unique_token = Token(SERVER.0 + 1);

    println!("listening for connections");

    loop {
        if let Err(err) = poll.poll(&mut events, None) {
            if interrupted(&err) {
                continue;
            }
            return Err(err);
        }

        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let (stream, address) = match server.accept() {
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

                    let token = next(&mut unique_token);
                    let registry = poll.registry();
                    let mut client = Client::new(token, stream);

                    client.start(registry)?;
                    clients.insert(token, client);
                },
                token => {
                    let done = if let Some(client) = clients.get_mut(&token) {
                        handle_connection_event(
                            poll.registry(),
                            client,
                            event,
                            &parser,
                            &mut store,
                        )?
                    } else {
                        // ignore event for unknown token
                        false
                    };

                    if done {
                        clients.remove(&token);
                    }
                }
            }
        }
    }
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

/// Returns `true` if the connection is done.
fn handle_connection_event(
    registry: &Registry,
    client: &mut Client,
    event: &Event,
    parser: &Parser,
    store: &mut Store,
) -> io::Result<bool> {
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
                store.expire_items();
                match client.run_commands(parser, store, registry) {
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
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
