extern crate hyper;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
extern crate simple_logger;

use std::io::Read;
use std::str::{from_utf8};
use std::io;
use std::fs::File;

use hyper::Client;
use hyper::header::Connection;
use nom::{eof, multispace, not_line_ending, space};

named!(blanks,
       chain!(
           many0!(alt!(multispace | comment_one_line)),
           || { &b""[..] }));

named!(eol,
       alt!(tag!("\n") | tag!("\r\n") | tag!("\u{2028}") | tag!("\u{2029}")));

named!(comment_one_line,
       chain!(
           tag!("#") ~
           not_line_ending? ~
           alt!(eof | eol),
           || { &b""[..] }));

named!(comment_block<&[u8], Vec<String> >,
    chain!(
        comments: many0!(comment_line),
        ||{
            comments
        }
    )
);

named!(comment_line<&[u8], String>,
    chain!(
        tag!("#") ~
        line: map_res!(take_until_and_consume!("\n"), from_utf8),
        ||{
            line.to_string()
        }
    )
);

#[test]
fn test_comment_parsing(){
    let x: &[u8] = &[];

    let input = r#"##
# @query-vnc:
#
# Returns information about the current VNC server
#
# Returns: @VncInfo
#
# Since: 0.14.0
##"#;
    let result = comment_block(input.as_bytes());
    println!("test_comment_parsing Result: {:?}", result);
}

named!(gen<bool>,
    chain!(
        tag!("'gen':") ~
        blanks~
        gen: alt!(
            map_res!(tag!("false"), from_utf8) |
            map_res!(tag!("true"), from_utf8)),
        ||{
            if gen == "false"{
                false
            }else{
                true
            }
        }
    )
);

//'returns': [ 'ObjectPropertyInfo' ]
//'returns': 'VncInfo'
named!(parse_return_list<String>,
    chain!(
        tag!("'returns':")~
        blanks~
        tag!("[") ~
        blanks~
        return_type: quoted_string ~
        blanks ~
        tag!("]") ~
        blanks?,
        ||{
            return_type
        }
    )
);

named!(parse_return_string<String>,
    chain!(
        tag!("'returns':")~
        blanks~
        return_type: quoted_string ~
        blanks?,
        ||{
            return_type
        }
    )
);

#[test]
fn test_returns_parser(){
    let x: &[u8] = &[];

    //TODO: How do I return both a List and a Struct
    let input = "'returns': [ 'ObjectPropertyInfo' ]";
    let result = parse_return_list(input.as_bytes());
    println!("test_returns Result: {:?}", result);
    assert_eq!(
        nom::IResult::Done(x,
            "ObjectPropertyInfo".to_string()),
            result);

    //Slightly different format
    let input2 = "'returns': ['MouseInfo']";
    let result2 = parse_return_list(input2.as_bytes());
    println!("test_returns Result: {:?}", result2);
    assert_eq!(
        nom::IResult::Done(x,
            "MouseInfo".to_string()),
            result2);
}

named!(quoted_string<&[u8], String>,
    chain!(
        tag!("'") ~
        s: map_res!(take_until!("'"), from_utf8) ~
        tag!("'") ,
        ||{
            s.to_string()
        }
    )
);

named!(quoted_string_vec<&[u8], Vec<String> >,
    chain!(
        tag!("'") ~
        s: map_res!(take_until!("'"), from_utf8) ~
        tag!("'") ,
        ||{
            vec![s.to_string()]
        }
    )
);

named!(unsplit_list<&[u8], String>,
    chain!(
        tag!("[") ~
        values: map_res!(take_until!("]"), from_utf8) ~
        tag!("]") ,
        ||{
            values.to_string()
        }
    )
);

named!(unsplit_field_list<&[u8], &str>,
    map_res!(chain!(
        dbg!(tag!("{")) ~
        values: dbg!(take_until!("}")) ~
        dbg!(tag!("}")) ,
        ||{
            values
        }
    ),from_utf8)
);

//Take input from unsplit_field_list and split it into fields
named!(data_field_list<&[u8], Vec<String> >,
    chain!(
        field_list: unsplit_field_list,
        ||{
            //Remove whitespace
            let parts: Vec<&str> = field_list.split(",").collect();
            parts.iter()
                .map(|s| s.replace("'", ""))
                .map(|s| s.replace(" ", ""))
                .map(|s| s.replace("\n", ""))
                .map(|s| s.replace("*", "")).collect()
        }
    )
);

#[test]
fn test_data_field_list(){
    let x: &[u8] = &[];

    let input = "{ 'protocol': 'str', 'fdname': 'str', '*skipauth': 'bool',
            '*tls': 'bool' }";
    let result = data_field_list(input.as_bytes());
    println!("test_data_field_list Result: {:?}", result);
    assert_eq!(nom::IResult::Done(x,
        vec![
            "protocol:str".to_string(),
            "fdname:str".to_string(),
            "skipauth:bool".to_string(),
            "tls:bool".to_string(),
        ]), result);
}

named!(discriminator<&[u8], String>,
    chain!(
        tag!("'discriminator':") ~
        blanks? ~
        name: quoted_string ~
        tag!(",")~
        blanks?,
        ||{
            name.to_string()
        }
    )
);

/*
'base': {'CPU': 'int', 'current': 'bool', 'halted': 'bool',
         'qom_path': 'str', 'thread_id': 'int', 'arch': 'CpuInfoArch' },

'base': 'VncBasicInfo',
*/
named!(base<&[u8], Vec<String> >,
    chain!(
        tag!("'base':") ~
        blanks? ~
        fields: alt!(
            dbg!(data_field_list) |
            dbg!(quoted_string_vec))~
        tag!(",")~
        blanks?,
        ||{
            fields
        }
    )
);

named!(data<Vec<String> >,
    chain!(
        dbg!(tag!("'data': ")) ~
        data: dbg!(data_field_list)~
        dbg!(tag!(","))~
        dbg!(blanks)?,
        ||{
            data
        }
    )
);
//Take input from unsplit_list and split it into fields
named!(enum_list<&[u8], Vec<String> >,
    chain!(
        l: unsplit_list,
        ||{
            let parts: Vec<&str> = l.split(",").collect();
            parts.iter()
                .map(|s| s.replace("'", ""))
                .map(|s| s.replace(" ", "")).collect()
        }
    )
);

#[test]
fn test_enum_list(){
    let x: &[u8] = &[];
    let input = "['discard', 'delay', 'merge', 'slew' ]";
    let result = enum_list(input.as_bytes());
    println!("test_enum_list Result: {:?}", result);
    assert_eq!(nom::IResult::Done(x,
        vec![
            "discard".to_string(),
            "delay".to_string(),
            "merge".to_string(),
            "slew".to_string()]), result);
}

named!(field_pair<&[u8], (String, String) >,
    chain!(
        name: quoted_string ~
        tag!(": ") ~
        qemu_type: quoted_string ~
        call!(trailing_chars),
        ||{
            (name, qemu_type)
        }
    )
);

fn trailing_chars(input: &[u8]) ->nom::IResult<&[u8], ()>{
    //3 possible trailing chars either "," " " or "".  They all need to be handled
    let comma = tag!(input,",");
    match comma{
        nom::IResult::Done(remaining, _) => {
            //Found a comma, we're done
            return nom::IResult::Done(remaining, ());
        },
        nom::IResult::Incomplete(_) => {
            //Ran out of input.  We're done
            return nom::IResult::Done(input, ());
        },
        nom::IResult::Error(_) => {
            //Possibly a space?
            let space = tag!(input, " ");
            match space{
                nom::IResult::Done(remaining, _) => {
                    //Found a space, we're done
                    return nom::IResult::Done(remaining, ());
                },
                nom::IResult::Incomplete(_) => {
                    //Ran out of input.  We're done
                    return nom::IResult::Done(input, ());
                },
                nom::IResult::Error(_) => {
                    return nom::IResult::Done(input, ());
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Struct{
    name: String,
    fields: Vec<String>,
    base: Option<Vec<String>>, //Fields from the base class
}

impl Struct{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        println!("Input to Struct: {:?}", String::from_utf8_lossy(input));

        //Check if base is first. Sometimes it comes first and sometimes data comes first
        let base_first = base(input);
        match base_first {
            nom::IResult::Done(remaining, base) => {
                //println!("base first: {:?}", String::from_utf8_lossy(remaining));
                chain!(
                    remaining,
                    dbg!(tag!("'data': ")) ~
                    data: dbg!(data_field_list)~
                    dbg!(tag!(","))?~
                    dbg!(blanks)?,
                    ||{
                        Struct{
                            name: name,
                            fields: data,
                            base: Some(base),
                        }
                    }
                )
            }
            nom::IResult::Incomplete(needed) => nom::IResult::Incomplete(needed),
            nom::IResult::Error(_) => {
                //Data is probably first
                println!("Data first input: {:?}", String::from_utf8_lossy(input));
                chain!(
                    input,
                    dbg!(tag!("'data': ")) ~
                    data: dbg!(data_field_list)~
                    dbg!(tag!(","))?~
                    dbg!(blanks)?~
                    base: dbg!(base) ? ,
                    ||{
                        Struct{
                            name: name,
                            fields: data,
                            base: base,
                        }
                    }
                )
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Command{
    name: String,
    fields: Option<Vec<String>>,
    gen: Option<bool>,
    returns: Option<QemuReturnType>, //Either a type or a list of type
}

impl Command{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        //println!("Input to Command: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            dbg!(tag!("'data': "))? ~
            data: dbg!(data_field_list)?~
            dbg!(tag!(","))?~
            dbg!(blanks)?~
            gen: dbg!(gen) ? ~
            returns_list: parse_return_list ? ~
            returns_string: parse_return_string ? ~
            dbg!(blanks)?,
            ||{
                let return_value: Option<QemuReturnType>;
                if returns_list.is_some(){
                    return_value = Some(QemuReturnType::List(returns_list.unwrap()));
                }else if returns_string.is_some(){
                    return_value = Some(QemuReturnType::String(returns_string.unwrap()));
                }else{
                    return_value = None;
                }
                Command{
                    name: name,
                    gen: gen,
                    fields: data,
                    returns: return_value,
                }
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Union{
    name: String,
    base: Option<Vec<String>>,
    discriminator: Option<String>,
    data: Vec<String>,

}

impl Union{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        println!("Input to Union: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            base: dbg!(base) ? ~
            discriminator_name: dbg!(discriminator) ? ~
            dbg!(tag!("'data':"))~
            dbg!(blanks)?~
            fields: dbg!(data_field_list)~
            blanks?,
            ||{
                Union{
                    name: name,
                    base: base,
                    discriminator: discriminator_name,
                    data: fields,
                }
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Enum{
    name: String,
    fields: Vec<String>,
}

impl Enum{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        //println!("Input to Enum: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            tag!("'data': ")~
            dbg!(blanks)?~
            fields: dbg!(enum_list)~
            blanks?,
            ||{
                Enum{
                    name: name,
                    fields: fields,
                }
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
enum QemuReturnType{
    List(String), //A list type
    String(String), // Returns a single thing
}

#[derive(Debug, Eq, PartialEq)]
enum QemuType{
    Struct(Struct),
    Command(Command),
    Enum(Enum),
    Include{
        name: String,
    },
    Union(Union),
    Unknown,
}

impl QemuType {
    fn parse(input: & [u8]) -> nom::IResult<&[u8], Self> {
        //println!("Input to Type: {:?}", String::from_utf8_lossy(input));
        let f = chain!(
            input,
            fields: dbg!(field_pair)~
            blanks? ,
            ||{
                fields
            }
        );
        match f{
            nom::IResult::Done(remaining, fields) => {
                match &fields.0[..]{
                    "struct" => {
                        match Struct::parse(remaining, fields.1.to_string()){
                            nom::IResult::Done(left, s) => {
                                nom::IResult::Done(left, QemuType::Struct(s))
                            }
                            nom::IResult::Incomplete(needed) => {
                                nom::IResult::Incomplete(needed)
                            }
                            nom::IResult::Error(err) => {
                                nom::IResult::Error(err)
                            }
                        }
                    }
                    "command" => {
                        match Command::parse(remaining, fields.1.to_string()){
                            nom::IResult::Done(left, c) => {
                                nom::IResult::Done(left, QemuType::Command(c))
                            }
                            nom::IResult::Incomplete(needed) => {
                                nom::IResult::Incomplete(needed)
                            }
                            nom::IResult::Error(err) => {
                                nom::IResult::Error(err)
                            }
                        }
                    }
                    "enum" => {
                        match Enum::parse(remaining, fields.1.to_string()){
                            nom::IResult::Done(left, e) => {
                                nom::IResult::Done(left, QemuType::Enum(e))
                            }
                            nom::IResult::Incomplete(needed) => {
                                nom::IResult::Incomplete(needed)
                            }
                            nom::IResult::Error(err) => {
                                nom::IResult::Error(err)
                            }
                        }
                    }
                    "union" => {
                        match Union::parse(remaining, fields.1.to_string()){
                            nom::IResult::Done(left, u) => {
                                nom::IResult::Done(left, QemuType::Union(u))
                            }
                            nom::IResult::Incomplete(needed) => {
                                nom::IResult::Incomplete(needed)
                            }
                            nom::IResult::Error(err) => {
                                nom::IResult::Error(err)
                            }
                        }
                    }
                    "include" => nom::IResult::Done(remaining, QemuType::Include{name: fields.1.to_string()}),
                    _ => {
                        nom::IResult::Done(remaining, QemuType::Unknown)
                    },
                }
            }
            nom::IResult::Incomplete(needed) => {
                nom::IResult::Incomplete(needed)
            }
            nom::IResult::Error(err) => {
                nom::IResult::Error(err)
            }
        }
    }
}

#[test]
fn test_union_section_parser(){
    let input = r#"##
# @CpuInfo:
#
# Information about a virtual CPU
#
# @CPU: the index of the virtual CPU
#
# @current: this only exists for backwards compatibility and should be ignored
#
# @halted: true if the virtual CPU is in the halt state.  Halt usually refers
#          to a processor specific low power mode.
#
# @qom_path: path to the CPU object in the QOM tree (since 2.4)
#
# @thread_id: ID of the underlying host thread
#
# @arch: architecture of the cpu, which determines which additional fields
#        will be listed (since 2.6)
#
# Since: 0.14.0
#
# Notes: @halted is a transient state that changes frequently.  By the time the
#        data is sent to the client, the guest may no longer be halted.
##
{ 'union': 'CpuInfo',
  'base': {'CPU': 'int', 'current': 'bool', 'halted': 'bool',
           'qom_path': 'str', 'thread_id': 'int', 'arch': 'CpuInfoArch' },
  'discriminator': 'arch',
  'data': { 'x86': 'CpuInfoX86',
            'sparc': 'CpuInfoSPARC',
            'ppc': 'CpuInfoPPC',
            'mips': 'CpuInfoMIPS',
            'tricore': 'CpuInfoTricore',
            'other': 'CpuInfoOther' } }
    "#;
    let result = Section::parse(input.as_bytes());
    println!("test_section_parser result: {:?}", result);
}

#[test]
fn test_enum_section_parser(){
    let input = r#"##
# @TpmModel:
#
# An enumeration of TPM models
#
# @tpm-tis: TPM TIS model
#
# Since: 1.5
##
{ 'enum': 'TpmModel', 'data': [ 'tpm-tis' ] }
    "#;
    let result = Section::parse(input.as_bytes());
    println!("test_section_parser result: {:?}", result);
}

#[test]
fn test_include_section_parser(){
    let x: &[u8] = &[];
    let input = r#"# QAPI common definitions
{ 'include': 'qapi/common.json' }"#;
    let result = Section::parse(input.as_bytes());
    println!("test_include_section_parser result: {:?}", result);
    assert_eq!(nom::IResult::Done(x,
        Section {
            description:
                vec![" QAPI common definitions".to_string()],
                qemu_type: QemuType::Include {
                    name: "qapi/common.json".to_string() } }
        ), result);
}
#[test]
fn test_struct_section_parser3(){
    let input = r#"##
# @ChardevFile:
#
# Configuration info for file chardevs.
#
# @in:  #optional The name of the input file
# @out: The name of the output file
# @append: #optional Open the file in append mode (default false to
#          truncate) (Since 2.6)
#
# Since: 1.4
##
{ 'struct': 'ChardevFile', 'data': { '*in' : 'str',
                                   'out' : 'str',
                                   '*append': 'bool' },
  'base': 'ChardevCommon' }"#;
    let result = Section::parse(input.as_bytes());
    println!("test_section_parser3 result: {:?}", result);
}

#[test]
fn test_struct_section_parser2(){
    let input = r#"##
# @VncServerInfo
#
# The network connection information for server
#
# @auth: #optional, authentication method
#
# Since: 2.1
##
{ 'struct': 'VncServerInfo',
  'base': 'VncBasicInfo',
  'data': { '*auth': 'str' } }
    "#;
    let result = Section::parse(input.as_bytes());
    println!("test_section_parser2 result: {:?}", result);
}

#[test]
fn test_struct_section_parser(){
    let x: &[u8] = &[];
    let input = r#"#
# @MigrationParameters
#
# @compress-level: compression level
#
# @compress-threads: compression thread count
#
# @decompress-threads: decompression thread count
#
# @x-cpu-throttle-initial: Initial percentage of time guest cpus are throttled
#                          when migration auto-converge is activated. The
#                          default value is 20. (Since 2.5)
#
# @x-cpu-throttle-increment: throttle percentage increase each time
#                            auto-converge detects that migration is not making
#                            progress. The default value is 10. (Since 2.5)
#
# Since: 2.4
##
{ 'struct': 'MigrationParameters',
  'data': { 'compress-level': 'int',
            'compress-threads': 'int',
            'decompress-threads': 'int',
            'x-cpu-throttle-initial': 'int',
            'x-cpu-throttle-increment': 'int'} }
    "#;
    let result = Section::parse(input.as_bytes());
    println!("test_struct_section_parser result: {:?}", result);
    assert_eq!(nom::IResult::Done(x,
        Section {
            description: vec![
                "".to_string(),
                " @MigrationParameters".to_string(),
                "".to_string(),
                " @compress-level: compression level".to_string(),
                "".to_string(),
                " @compress-threads: compression thread count".to_string(),
                "".to_string(),
                " @decompress-threads: decompression thread count".to_string(),
                "".to_string(),
                " @x-cpu-throttle-initial: Initial percentage of time guest cpus are throttled".to_string(),
                "                          when migration auto-converge is activated. The".to_string(),
                "                          default value is 20. (Since 2.5)".to_string(),
                "".to_string(),
                " @x-cpu-throttle-increment: throttle percentage increase each time".to_string(),
                "                            auto-converge detects that migration is not making".to_string(),
                "                            progress. The default value is 10. (Since 2.5)".to_string(),
                "".to_string(),
                " Since: 2.4".to_string(),
                "#".to_string()],
                qemu_type: QemuType::Struct(
                    Struct {
                        name: "MigrationParameters".to_string(),
                        fields: vec!["compress-level:int".to_string(), "compress-threads:int".to_string(), "decompress-threads:int".to_string(),
                        "x-cpu-throttle-initial:int".to_string(), "x-cpu-throttle-increment:int".to_string()], base: None })
        }), result);
}

#[test]
fn test_command_section_parser(){
    let input = r#"##
# @qom-list:
#
# This command will list any properties of a object given a path in the object
# model.
#
# @path: the path within the object model.  See @qom-get for a description of
#        this parameter.
#
# Returns: a list of @ObjectPropertyInfo that describe the properties of the
#          object.
#
# Since: 1.2
##
{ 'command': 'qom-list',
  'data': { 'path': 'str' },
  'returns': [ 'ObjectPropertyInfo' ] }"#;
    let result = Section::parse(input.as_bytes());
    println!("test_section_parser result: {:?}", result);

}
#[test]
fn test_command_section_parser2(){
    let input = r#"# @add_client
#
# Allow client connections for VNC, Spice and socket based
# character devices to be passed in to QEMU via SCM_RIGHTS.
#
# @protocol: protocol name. Valid names are "vnc", "spice" or the
#            name of a character device (eg. from -chardev id=XXXX)
#
# @fdname: file descriptor name previously passed via 'getfd' command
#
# @skipauth: #optional whether to skip authentication. Only applies
#            to "vnc" and "spice" protocols
#
# @tls: #optional whether to perform TLS. Only applies to the "spice"
#       protocol
#
# Returns: nothing on success.
#
# Since: 0.14.0
##
{ 'command': 'add_client',
  'data': { 'protocol': 'str', 'fdname': 'str', '*skipauth': 'bool',
            '*tls': 'bool' } }"#;
    let result = Section::parse(input.as_bytes());
    println!("test_section_parser result: {:?}", result);

}

#[derive(Debug, Eq, PartialEq)]
struct Section {
    description: Vec<String>,
    qemu_type: QemuType,
}

impl Section{
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self>{
        //println!("Section parse input: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            comments: comment_block~
            tag!("{ ")~
            qemu_type: dbg!(call!(QemuType::parse)) ~
            tag!("}")~
            blanks?,
            ||{
                Section{
                    description: comments,
                    qemu_type: qemu_type,
                }
            }
        )
    }
}

fn parse_sections(input: &[u8])-> nom::IResult<&[u8], Vec<Section>>{
    chain!(
        input,
        sections: many0!(call!(Section::parse)),
        ||{
            sections
        }
    )
}

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

    let mut f = File::open("/home/chris/repos/qemu-rust-generator/test-file.txt").unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();
    let result = parse_sections(buffer.as_bytes());
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
