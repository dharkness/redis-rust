#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    dead_code
)]

use std::io;

use crate::network::Server;

mod commands;
mod input;
mod network;
mod parser;
mod pattern;
mod resp;
mod store;

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:6379".parse().expect("unable to parse address");

    Server::new()?.start(addr)
}
