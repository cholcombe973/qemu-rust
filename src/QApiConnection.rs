extern crate bytes;
extern crate mio;
extern crate slab;

use self::bytes::{Buf, ByteBuf, MutByteBuf, SliceBuf};
use self::mio::*;
use self::mio::tcp::{TcpStream};

use std::io;

pub struct QApiConnection{
    sock: TcpStream,
    buf: Option<ByteBuf>,
    mut_buf: Option<MutByteBuf>,
    token: Token,
    interest: EventSet
}

impl QApiConnection{
    fn new(sock: TcpStream) -> QApiConnection {
        QApiConnection {
            buf: None,
            sock: sock,
            mut_buf: Some(ByteBuf::mut_with_capacity(2048)),
            token: Token(1),
            interest: EventSet::none()
        }
    }
    fn read_event(&mut self, event_loop: &mut EventLoop<QApiConnection>) -> io::Result<()> {
        println!("client socket readable");
        let mut buf = self.mut_buf.take().unwrap();

        match self.sock.try_read_buf(&mut buf) {
            Ok(None) => {
                println!("CONN : spurious read wakeup");
                self.mut_buf = Some(buf);
            }
            Ok(Some(r)) => {
                println!("CONN : we read {} bytes!", r);
                println!("{:?}", String::from_utf8_lossy(buf.bytes()));

                // prepare to provide this to writable
                self.buf = Some(buf.flip());

                self.interest.remove(EventSet::readable());
                self.interest.insert(EventSet::writable());
            }
            Err(e) => {
                println!("not implemented; client err={:?}", e);
                self.interest.remove(EventSet::readable());
            }

        };

        event_loop.reregister(&self.sock, self.token, self.interest,
                              PollOpt::edge())
    }

    fn write_event(&mut self, event_loop: &mut EventLoop<QApiConnection>) -> io::Result<()> {
        println!("client socket writable");
        let mut buf = self.buf.take().unwrap();

        match self.sock.try_write_buf(&mut buf) {
            Ok(None) => {
                println!("client flushing buf; WOULDBLOCK");
                self.interest.insert(EventSet::writable());
            }
            Ok(Some(r)) => {
                println!("CLIENT : we wrote {} bytes!", r);
                self.interest.insert(EventSet::readable());
                self.interest.remove(EventSet::writable());
            }
            Err(e) => println!("not implemented; client err={:?}", e)
        }

        event_loop.reregister(&self.sock, self.token, self.interest,
                              PollOpt::edge() | PollOpt::oneshot())
    }
}

impl Handler for QApiConnection {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<QApiConnection>, token: Token, events: EventSet) {
        println!("ready {:?} {:?}", token, events);
        if events.is_readable(){
            match token {
                CLIENT => self.read_event(event_loop).unwrap(),
            }
        }
        if events.is_writable(){
            match token {
                CLIENT => {
                    //let mut reply = ByteBuf::from_slice("".as_bytes());
                    //TODO: Set or allow setting of buf to write back a reply
                    self.write_event(event_loop).unwrap()
                },
            }
        }
    }

}

/*
fn main(){
    //NOTE: QEMU only allows 1 client at a time to connect.  Everyone else will hang
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
    event_loop.run(&mut MyHandler::new(sock)).unwrap();
}
*/
