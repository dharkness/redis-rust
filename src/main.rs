#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    dead_code,
    unused_imports
)]

use std::io;

use crate::network::Server;

mod commands;
mod errors;
mod network;
mod parse;
mod resp;
mod storage;

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:6379".parse().expect("unable to parse address");

    Server::new()?.start(addr)
}
