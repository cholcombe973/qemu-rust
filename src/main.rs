extern crate bytes;
#[macro_use] extern crate log;
extern crate mio;
extern crate simple_logger;
extern crate slab;

use mio::{EventLoop, EventSet, PollOpt, Token};
use mio::tcp::{TcpStream};

mod qapi_connection;

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
    event_loop.run(&mut qapi_connection::QApiConnection::new(sock)).unwrap();
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn).unwrap();

    //Lets have a chat with the server
    connect_to_qemu();
}
