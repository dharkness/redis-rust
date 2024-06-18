use std::io;

pub use client::Client;
pub use error::Error;
pub use response::*;
pub use server::Server;

mod client;
mod error;
mod response;
mod server;

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
