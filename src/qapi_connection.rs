extern crate bytes;
extern crate json;
extern crate mio;

use std::collections::HashMap;

use bytes::{Buf, ByteBuf, MutByteBuf};
use mio::*;
use mio::tcp::{TcpStream};

use std::io;

enum ClientState{
    ReadyForGreeting,
    ReadyForCapabilities,
    ReadyForCommands,
    Ready,
}

//QEMU Greeting
#[derive(Debug)]
struct RustVersion{
    major: u32,
    micro: u32,
    minor: u32,
}

impl RustVersion{
    fn from(j: &json::JsonValue)->RustVersion{
        RustVersion{
            major: j["major"].as_u32().unwrap_or(0),
            micro: j["micro"].as_u32().unwrap_or(0),
            minor: j["minor"].as_u32().unwrap_or(0),
        }
    }
}

#[derive(Debug)]
struct Version{
    package: String,
    qemu: RustVersion,
}

impl Version{
    fn from(j: &json::JsonValue)->Version{
        let package_version = j["package"].as_str().unwrap_or("");
        let qemu = RustVersion::from(&j["qemu"]);

        Version{
            package: package_version.to_string(),
            qemu: qemu,
        }
    }
}

#[derive(Debug)]
struct QMP{
    capabilities: Vec<String>,
    version: Version,
}

impl QMP{
    fn from(j: json::JsonValue)->QMP{
        //j: Object({"QMP": Object({"capabilities": Array([]),
        //"version": Object({"package": String(" (Debian 2.0.0+dfsg-2ubuntu1.19)"),
        //"qemu": Object({"major": Number(2), "micro": Number(0), "minor": Number(0)})})})})

        let qmp = j["QMP"].clone();
        let mut capabilities = Vec::new();
        for cap in qmp["capabilities"].members(){
            capabilities.push(cap.as_str().unwrap_or("").to_string())
        }
        let version = Version::from(&qmp["version"]);

        QMP{
            capabilities: capabilities,
            version: version,
        }
    }
}

#[derive(Debug)]
struct Command{
    name: String,
}
impl Command{
    fn from(j: &json::JsonValue) -> Command{
        Command{
            name: j["name"].as_str().unwrap_or("").to_string(),
        }
    }
}

#[derive(Debug)]
struct QemuCommands{
    commands: Vec<Command>,
}

impl QemuCommands{
    fn from(j: json::JsonValue)->QemuCommands{
        //j: {"return": Array([Object({"name": String("query-named-block-nodes")}), ...

        let mut commands = Vec::new();
        for cmd_object in j["return"].members(){
            let c = Command::from(&cmd_object);
            commands.push(c);
        }

        QemuCommands{
            commands: commands,
        }
    }
}

pub struct QApiConnection{
    sock: TcpStream,
    buf: Option<ByteBuf>,
    mut_buf: Option<MutByteBuf>,
    token: Token,
    interest: EventSet,
    state: ClientState,
}

struct QemuEventHandler{
    /// Takes a String Event name and matches it up with a Function to process the event
    handlers: HashMap<String,Box<(Fn(String))>>
}


impl QApiConnection{
    pub fn new(sock: TcpStream) -> QApiConnection {
        QApiConnection {
            buf: None,
            sock: sock,
            mut_buf: Some(ByteBuf::mut_with_capacity(4096)),
            token: Token(1),
            interest: EventSet::none(),
            state: ClientState::ReadyForGreeting,
        }
    }

    fn parse_greeting<'a>(&self, bytes: &'a [u8]){
        let result = json::parse(&String::from_utf8_lossy(bytes)).unwrap();
        let greeting = QMP::from(result);
        println!("Json parse_greeting result: {:?}", greeting);
    }

    fn parse_capabilities<'a>(&self, bytes: &'a [u8]){
        let result = json::parse(&String::from_utf8_lossy(bytes)).unwrap();
        println!("Json parse_capabilities result: {:?}", result);
    }

    fn parse_commands<'a>(&self, bytes: &'a [u8]){
        let result = json::parse(&String::from_utf8_lossy(bytes)).unwrap();
        let commands = QemuCommands::from(result);
        println!("Json parse_commands result: {:?}", commands);
    }

    /// Send Qemu a command
    fn send_command(){

    }

    ///There's an initial ping/pong type communication going on between the client and the
    ///qemu server to figure out what version we're talking and what the server is capable of.
    ///Once the ClientState settles into ClientState::Ready then we're ready to take
    ///user commands.
    fn read_event(&mut self, event_loop: &mut EventLoop<QApiConnection>) -> io::Result<()> {
        println!("Socket readable");

        //TODO: This returns None sometimes?  Why
        //let mut buf = self.mut_buf.take().unwrap();
        let mut buf = ByteBuf::mut_with_capacity(4096);

        match self.sock.try_read_buf(&mut buf) {
            Ok(None) => {
                println!("CONN : spurious read wakeup");
                self.mut_buf = Some(buf);
            }
            Ok(Some(r)) => {
                println!("CONN : we read {} bytes!", r);

                match self.state{
                    ClientState::ReadyForGreeting => {
                        self.parse_greeting(buf.bytes());

                        self.interest.remove(EventSet::readable());
                        self.interest.insert(EventSet::writable());

                        let reply = ByteBuf::from_slice("{ \"execute\": \"qmp_capabilities\" }".as_bytes());
                        self.buf = Some(reply);
                        //Update our state
                        //Lets ask for the capabilities set
                        self.state = ClientState::ReadyForCapabilities;
                    },
                    ClientState::ReadyForCapabilities => {
                        self.parse_capabilities(buf.bytes());

                        let reply = ByteBuf::from_slice("{ \"execute\": \"query-commands\" }".as_bytes());
                        self.buf = Some(reply);
                        //Update our state
                        //Lets ask for the capabilities set
                        self.state = ClientState::ReadyForCommands;
                        self.interest.remove(EventSet::readable());
                        self.interest.insert(EventSet::writable());
                    },
                    ClientState::ReadyForCommands => {
                        self.parse_commands(buf.bytes());

                        //Update our state
                        //Lets ask for the capabilities set
                        self.state = ClientState::Ready;

                        //Wait for user input now
                    },
                    ClientState::Ready => {
                        //Ready to receive events and user input
                        self.interest.remove(EventSet::readable());
                        self.interest.insert(EventSet::writable());
                    },
                }
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

        match self.state{
            ClientState::ReadyForGreeting => {
                //Nothing to do here
            },
            ClientState::ReadyForCapabilities => {
                //Lets ask for the capabilities set
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

                self.interest.insert(EventSet::readable());
                self.interest.remove(EventSet::writable());
            },
            ClientState::ReadyForCommands => {
                //Lets ask for the capabilities set
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

                self.interest.insert(EventSet::readable());
                self.interest.remove(EventSet::writable());
            },
            ClientState::Ready => {
                //Ready to receive events and user input
                self.interest.insert(EventSet::readable());
                self.interest.remove(EventSet::writable());
            },
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
