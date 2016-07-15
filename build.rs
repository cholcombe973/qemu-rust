extern crate hyper;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
extern crate parse_qapi;
extern crate simple_logger;

use hyper::Client;
use hyper::header::Connection;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{PathBuf};

fn write_docs(description: Vec<String>, f: &mut File){
  for d in description{
    let s = d.replace("#","///");
    f.write(s.as_bytes());
  }
}
fn write_sections_to_files(data: Vec<parse_qapi::Section>){
    //Organize the output into appropriate directories
    let events_src = PathBuf::from("src/events.rs");
    let structs_src = PathBuf::from("src/structs.rs");
    let enums_src = PathBuf::from("src/enums.rs");
    let commands_src = PathBuf::from("src/commands.rs");

    let mut events = OpenOptions::new().write(true).truncate(true).open(events_src).unwrap();
    let mut structs = OpenOptions::new().write(true).truncate(true).open(structs_src).unwrap();
    let mut enums = OpenOptions::new().write(true).truncate(true).open(enums_src).unwrap();
    let mut commands = OpenOptions::new().write(true).truncate(true).open(commands_src).unwrap();

    //Include the necessary files
    commands.write("use QemuCmd;\n".as_bytes()).unwrap();
    commands.write("use rustc_serialize::json as rustc_json;\n".as_bytes()).unwrap();
    commands.write("use rustc_serialize::Decodable as rustc_decodable;\n".as_bytes()).unwrap();
    commands.write("use json;\n".as_bytes()).unwrap();
    commands.write("use events::*;\n".as_bytes()).unwrap();
    commands.write("use enums::*;\n".as_bytes()).unwrap();
    commands.write("use structs::*;\n".as_bytes()).unwrap();

    for section in data{
        match section.qemu_type{
            parse_qapi::QemuType::Struct(st) => {
                write_docs(section.description, &mut structs);
                structs.write(st.to_rust_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Command(c) => {
                write_docs(section.description, &mut commands);
                //TODO: qom-get has a ** return value and I don't know what to do with that
                if c.name != "qom-get"{
                    commands.write(c.to_rust_string().as_bytes()).unwrap();
                }
            },
            parse_qapi::QemuType::Event(e) => {
                write_docs(section.description, &mut events);
                events.write(e.to_rust_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Enum(e) => {
                write_docs(section.description, &mut enums);
                enums.write(e.to_rust_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Include{name} => {
                write_docs(section.description, &mut commands);
                commands.write(name.to_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Union(u) => {
                write_docs(section.description, &mut structs);
                structs.write(u.to_rust_string().as_bytes()).unwrap();
            },
            parse_qapi::QemuType::Unknown => {},
        }
    }
    events.sync_all().unwrap();
    enums.sync_all().unwrap();
    structs.sync_all().unwrap();
    commands.sync_all().unwrap();
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
        "http://repo.or.cz/qemu/qmp-unstable.git/blob_plain/refs/heads/master:/qapi/event.json",

        //Main file
        "http://repo.or.cz/w/qemu/qmp-unstable.git/blob_plain/HEAD:/qapi-schema.json",
    ];

    let mut buffer = String::new();
    for source in json_sources{
        let mut res = client.get(source)
            .header(Connection::close())
            .send().unwrap();
        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        //Sanitize input
        let body = body.replace("\'", "\"");
        buffer.push_str(&body);
    }


    let result = parse_qapi::parse_sections(buffer.as_bytes());
    match result{
        nom::IResult::Done(leftover, qemu_sections) => {
            debug!("leftover: {:?}", String::from_utf8_lossy(leftover));
            debug!("Result: {:?}", qemu_sections);
            write_sections_to_files(qemu_sections);
        },
        nom::IResult::Incomplete(needed) => {
            debug!("Incomplete: {:?}", needed);
        },
        nom::IResult::Error(e) => {
            debug!("Error: {:?}", e);
        },
    }
}
