//! A library to interface with Qemu.  For more information about Qemu see
//! [qemu](http://www.qemu.org)
//!
extern crate bytes;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate simple_logger;

mod output;

use serde::de::DeserializeOwned;
use serde_json::Value;

/*
pub mod commands;
pub mod qapi_connection;

pub mod enums;
pub mod events;
pub mod structs;
*/

pub trait QemuCmd {
    // Return a json blob that we can send to the Qemu Server

    fn to_json(&self) -> String;
    // fn parse_qemu_response(&self, response: &String) -> rustc_serialize::json::DecodeResult<Self>
    //    where Self: std::marker::Sized;
}

pub fn call_qemu<T: DeserializeOwned>(v: Value) -> Result<T, String> {
    Err("".into())
}

fn connect_to_qemu() {
    //const CLIENT: Token = Token(1);
    //let addr = "127.0.0.1:4444".parse().unwrap();

    // Setup the client socket
    //let sock = TcpStream::connect(&addr).unwrap();

    //let qemu_connection = qapi_connection::QApiConnection::new(sock);
}

fn main() {
    simple_logger::init_with_level(log::Level::Warn).unwrap();

    // Lets have a chat with the server
    connect_to_qemu();
}
