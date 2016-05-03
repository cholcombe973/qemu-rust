extern crate hyper;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
extern crate simple_logger;

use std::str::{from_utf8};

use nom::{eof, multispace, not_line_ending, space};
/*
# QAPI common definitions
{ 'include': 'qapi/common.json' }
*/
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
    /*
    assert_eq!(nom::IResult::Done(x,
        vec![
            "protocol:str".to_string(),
            "fdname:str".to_string(),
            "skipauth:bool".to_string(),
            "tls:bool".to_string(),
        ]), result);
    */
}

named!(parse_return_type<&[u8], &str>,
    map_res!(chain!(
        tag!("'returns': '")~
        return_type: take_until!("'")~
        tag!("',"),
        ||{
            return_type
        }
    ), from_utf8)
);

named!(quoted_string<&[u8], &str>,
    map_res!(
        chain!(
            tag!("'") ~
            s: take_until!("'") ~
            tag!("'") ,
            ||{
                s
            }
        ),
    from_utf8)
);

named!(unsplit_list<&[u8], &str>,
    map_res!(chain!(
        tag!("[") ~
        values: take_until!("]") ~
        tag!("]") ,
        ||{
            values
        }
    ),from_utf8)
);

named!(unsplit_field_list<&[u8], &str>,
    map_res!(chain!(
        tag!("{") ~
        values: take_until!("}") ~
        tag!("}") ,
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
    assert_eq!(nom::IResult::Done(x, vec!["discard".to_string(), "delay".to_string(), "merge".to_string(), "slew".to_string()]), result);
}

named!(field_pair<&[u8], (&str, &str) >,
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

named!(type_and_name<&[u8], (&str, &str) >,
    chain!(
        tag!("{ ") ~
        type_and_name: field_pair,
        ||{
            type_and_name
        }
    )
);

/*
fn parse_spec(input: &[u8]) -> T{
    match type_and_name(input){
        nom::IResult::Done(remaining, result) => {
            match result.0{
                "enum" => {
                    let fields = unsplit_list(remaining);
                },
                "struct" => {

                },
                "command" => {

                },
                _ => {
                    //Unknown
                    ()
                }
            }
            return nom::IResult::Done(remaining, ());
        }
        nom::IResult::Incomplete(_) => {
            //Ran out of input.  We're done
            return nom::IResult::Done(input, ());
        },
        nom::IResult::Error(_) => {

        }
    }
}
*/

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
/*
{ 'enum': 'LostTickPolicy',
  'data': ['discard', 'delay', 'merge', 'slew' ] }

{ 'command': 'add_client',
  'data': { 'protocol': 'str', 'fdname': 'str', '*skipauth': 'bool',
            '*tls': 'bool' } }
{ 'struct': 'UuidInfo', 'data': {'UUID': 'str'} }
{ 'struct': 'SpiceServerInfo',
  'base': 'SpiceBasicInfo', //I think this means it inherits the other fields from this parent struct
  'data': { '*auth': 'str' } }
{ 'command': 'query-rx-filter', 'data': { '*name': 'str' },
    'returns': ['RxFilterInfo'] } //This can be a list or a string
*/

#[derive(Debug)]
struct Struct{
    name: String,
    fields: Vec<String>,
    base: Option<Vec<String>>, //Fields from the base class
}

impl Struct{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        println!("Input to Struct: {:?}", input);
        chain!(
            input,
            data: data_field_list,
            ||{
                Struct{
                    name: name,
                    fields: vec![],
                    base: None,
                }
            }
        )
    }
}

#[derive(Debug)]
struct Command{
    name: String,
    fields: Option<Vec<String>>,
    returns: Option<String>, //Either a Struct or a list of Struct
}

impl Command{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        println!("Input to Command: {:?}", input);
        chain!(
            input,
            tag!("'data': ")? ~
            data: data_field_list?~
            returns: parse_return_type?,
            ||{
                Command{
                    name: name,
                    fields: None,
                    returns: None,
                }
            }
        )
    }
}

#[derive(Debug)]
struct Enum{
    name: String,
    fields: Vec<String>,
}

impl Enum{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        println!("Input to Enum: {:?}", input);
        chain!(
            input,
            tag!("'data': ")~
            fields: enum_list,
            ||{
                Enum{
                    name: name,
                    fields: vec![]
                }
            }
        )
    }
}

#[derive(Debug)]
enum QemuType{
    Struct(Struct),
    Command(Command),
    Enum(Enum),
    Unknown,
}

impl QemuType {
    fn parse(input: & [u8]) -> nom::IResult<&[u8], Self> {
        println!("Input to Type: {:?}", input);
        let f = chain!(
            input,
            tag!(" ")~
            fields: dbg!(field_pair),
            ||{
                fields
            }
        );
        println!("field_pair: {:?}", f);
        match f{
            nom::IResult::Done(remaining, fields) => {
                match fields.0{
                    "struct" => {
                        match Struct::parse(remaining, fields.1.to_string()){
                            nom::IResult::Done(left, s) => {
                                nom::IResult::Done(remaining, QemuType::Struct(s))
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
                                nom::IResult::Done(remaining, QemuType::Command(c))
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
                                nom::IResult::Done(remaining, QemuType::Enum(e))
                            }
                            nom::IResult::Incomplete(needed) => {
                                nom::IResult::Incomplete(needed)
                            }
                            nom::IResult::Error(err) => {
                                nom::IResult::Error(err)
                            }
                        }
                    }
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

struct Spec {
    qemu_type: QemuType,
    name: String,
    data: Option<Vec<String>>,
    returns: Option<String>,
}

struct Description<T> {
    docs: Vec<String>,
    fields: Field<T>,
}

struct Field<T> {
    name: String,
    qemu_type: T,
}

#[test]
fn test_section_parser(){
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

#[derive(Debug)]
struct Section {
    description: Vec<String>,
    qemu_type: QemuType,
}

impl Section{
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self>{
        chain!(
            input,
            comments: comment_block~
            tag!("{")~
            qemu_type: dbg!(call!(QemuType::parse)) ~
            tag!("}"),
            ||{
                Section{
                    description: comments,
                    qemu_type: qemu_type,
                }
            }
        )
    }
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn).unwrap();
    /*
    */
    // Create a client.
    /*
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://repo.or.cz/w/qemu/qmp-unstable.git/blob_plain/HEAD:/qapi-schema.json")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    //println!("Response: {}", body);
    */
}
