#![allow(dead_code)]
#![allow(unused_must_use)]

extern crate core;
extern crate byteorder;
#[macro_use]
extern crate bitflags;
extern crate rand;
extern crate itertools;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_service;
extern crate bytes;


// Private modules
mod frames;
mod header;
mod util;

// Public modules
pub mod error;
pub mod packet;
pub mod client;
pub mod stream;

#[cfg(test)]
mod tests {
    #[test]
    fn creates_new_client() {
        let _ = super::client::QuicClient::new("localhost", 443);
    }
}