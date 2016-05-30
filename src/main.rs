extern crate hyper;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
extern crate simple_logger;

use std::io::Read;
use std::io;
use std::fs::File;

use hyper::Client;
use hyper::header::Connection;

mod parse_qapi;

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn).unwrap();
    /*
    // Create a client.
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://repo.or.cz/w/qemu/qmp-unstable.git/blob_plain/HEAD:/qapi-schema.json")
        .header(Connection::close())
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let result = parse_sections(body.as_bytes());
    println!("Parsing result: {:?}", result);
    */

    let mut f = File::open("/home/chris/repos/qemu-rust-generator/test/test-file.txt").unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();
    let result = parse_qapi::parse_sections(buffer.as_bytes());
    match result{
        nom::IResult::Done(_, qemu) => {
            println!("Result: {:?}", qemu);
        }
        nom::IResult::Incomplete(needed) => {
            println!("Incomplete: {:?}", needed);
        }
        nom::IResult::Error(e) => {
            println!("Error: {:?}", e);
        }
    }
}
