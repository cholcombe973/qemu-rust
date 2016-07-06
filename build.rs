extern crate hyper;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
extern crate parse_qapi;
extern crate simple_logger;

use hyper::Client;
use hyper::header::Connection;

use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};

fn write_sections_to_file(p: &PathBuf, data: Vec<parse_qapi::Section>){
    let mut f = File::open(p).unwrap();

    for section in data{
        match section.qemu_type{
            parse_qapi::QemuType::Struct(st) => {
                f.write(section.description.join("\n///").as_bytes()).unwrap();
                f.write(st.to_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Command(c) => {
                f.write(section.description.join("\n///").as_bytes()).unwrap();
                f.write(c.to_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Enum(e) => {
                f.write(section.description.join("\n///").as_bytes()).unwrap();
                f.write(e.to_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Include{name} => {
                f.write(section.description.join("\n///").as_bytes()).unwrap();
                f.write(name.to_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Union(u) => {},
            parse_qapi::QemuType::Unknown => {},
        }
    }
    f.sync_all().unwrap();
}

pub fn main() {
    simple_logger::init_with_level(log::LogLevel::Debug).unwrap();

    let client = Client::new();
    let json_sources = vec![
        //Supporting stuff
        "http://repo.or.cz/qemu/qmp-unstable.git/blob_plain/refs/heads/master:/qapi/block.json",
        "http://repo.or.cz/qemu/qmp-unstable.git/blob_plain/refs/heads/master:/qapi/block-core.json",
        "http://repo.or.cz/qemu/qmp-unstable.git/blob_plain/refs/heads/master:/qapi/common.json",
        "http://repo.or.cz/qemu/qmp-unstable.git/blob_plain/refs/heads/master:/qapi/trace.json",

        //Main file
        "http://repo.or.cz/w/qemu/qmp-unstable.git/blob_plain/HEAD:/qapi-schema.json",
    ];

    //Organize the output into appropriate directories
    let src = PathBuf::from("src/qapi/mod.rs");

    debug!("QAPI path: {:?}", src);
    //let events = Path::new(&out_dir).join("events").join("mod.rs");
    //let structs = Path::new(&out_dir).join("structs").join("mod.rs");
    //let enums = Path::new(&out_dir).join("enums").join("mod.rs");
    //let commands = Path::new(&out_dir).join("commands").join("mod.rs");

    //TODO: Parser can't handle this yet
    //let mut events_json = client.get("http://repo.or.cz/qemu/qmp-unstable.git/blob_plain/refs/heads/master:/qapi/event.json");

    for source in json_sources{
        let mut res = client.get(source)
            .header(Connection::close())
            .send().unwrap();

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        //Sanitize input
        let body = body.replace("\'", "\"");

        //debug!("Hyper downloaded body: {:?}", body);
        let result = parse_qapi::parse_sections(body.as_bytes());
        match result{
            nom::IResult::Done(leftover, qemu_sections) => {
                debug!("leftover: {:?}", String::from_utf8_lossy(leftover));
                debug!("Result: {:?}", qemu_sections);
                write_sections_to_file(&src, qemu_sections);
            },
            nom::IResult::Incomplete(needed) => {
                debug!("Incomplete: {:?}", needed);
            },
            nom::IResult::Error(e) => {
                debug!("Error: {:?}", e);
            },
        }
    }
}
