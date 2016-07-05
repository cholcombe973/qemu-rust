extern crate bytes;
extern crate hyper;
#[macro_use] extern crate log;
extern crate mio;
#[macro_use] extern crate nom;
extern crate simple_logger;
extern crate slab;

use std::io::Read;
use std::io;
use std::fs::File;

use mio::{EventLoop, EventSet, PollOpt, Token};
use mio::tcp::{TcpStream};

use hyper::Client;
use hyper::header::Connection;

mod parse_qapi;
mod QApiConnection;

fn connect_to_qemu(){
    const CLIENT: Token = Token(1);
    let addr = "127.0.0.1:4444".parse().unwrap();

    // Create an event loop
    let mut event_loop = EventLoop::new().unwrap();

    // Setup the client socket
    let sock = TcpStream::connect(&addr).unwrap();

    // Register the socket
    event_loop.register(&sock, CLIENT,
        EventSet::readable(),
        PollOpt::edge()).unwrap();

    // Start handling events
    event_loop.run(&mut QApiConnection::QApiConnection::new(sock)).unwrap();
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn).unwrap();

    let mut f = File::open("test/qapi.json").unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();
    let result = parse_qapi::parse_sections(buffer.as_bytes());
    match result{
        nom::IResult::Done(_, qemu) => {
            println!("Result: {:?}", qemu);
            for result in qemu{
                parse_qapi::print_section(result);
            }
        }
        nom::IResult::Incomplete(needed) => {
            println!("Incomplete: {:?}", needed);
        }
        nom::IResult::Error(e) => {
            println!("Error: {:?}", e);
        }
    }

    //Lets have a chat with the server
    connect_to_qemu();
}
