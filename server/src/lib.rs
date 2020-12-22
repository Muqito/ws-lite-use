#![forbid(unsafe_code)]
#![feature(array_methods)]
#![feature(trait_alias)]
#![feature(const_generics)]
extern crate async_net;
extern crate futures_lite;

pub mod connection;
pub mod incoming_connection;
pub mod incoming_connections;
pub mod messages;
pub mod server;

pub use self::server::WsServer;

pub use self::async_net::{TcpListener, TcpStream};

#[derive(Debug)]
pub enum WsError {
    ConnectionError,
    ConnectionClosed,
    InvalidPayload,
    Unknown,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
