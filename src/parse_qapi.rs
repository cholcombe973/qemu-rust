use std::str::{from_utf8};

use nom;
use nom::{eof, multispace, not_line_ending, space};

named!(blanks,
       chain!(
           many0!(alt!(multispace | comment_one_line)),
           //TODO: This gets the comments but breaks the parser somewhere
           //many0!(multispace),
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
        tag!("{") ~
        values: take_until!("}") ~
        tag!("}"),
        ||{
            values
        }
    ),from_utf8)
);

//Take input from unsplit_field_list and split it into fields
named!(data_field_list<&[u8], Vec<(String,String)> >,
    chain!(
        field_list: unsplit_field_list,
        ||{
            //Remove whitespace
            let mut result: Vec<(String,String)> = Vec::with_capacity(field_list.len());
            let parts: Vec<&str> = field_list.split(",").collect();
            let clean: Vec<String> = parts.iter()
                .map(|s| s.replace("'", ""))
                .map(|s| s.replace(" ", ""))
                .map(|s| s.replace("\n", ""))
                .map(|s| s.replace("*", "")).collect();

            for part in clean.iter(){
                let s: Vec<&str> = part.split(":").collect();
                result.push(
                    (
                        s.get(0).unwrap().to_string(),
                        s.get(1).unwrap().to_string()
                    )
                )
            }
            result
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
            ("protocol".to_string(),"str".to_string()),
            ("fdname".to_string(),"str".to_string()),
            ("skipauth".to_string(),"bool".to_string()),
            ("tls".to_string(),"bool".to_string()),
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
#[test]
fn test_struct_base(){
    let x: &[u8] = &[];
    let input = "'base': 'ChardevCommon' ";
    let result = struct_base(input.as_bytes());
    println!("struct_base: {:?}", result);
    assert_eq!(nom::IResult::Done(x, "ChardevCommon".to_string()), result);
}

#[test]
fn test_union_base(){
    let x: &[u8] = &[];
    let input = r#"'base': {'CPU': 'int', 'current': 'bool', 'halted': 'bool',
           'qom_path': 'str', 'thread_id': 'int', 'arch': 'CpuInfoArch' },"#;
    let result = union_base(input.as_bytes());
    println!("union_base: {:?}", result);
    assert_eq!(nom::IResult::Done(x,
        vec![
            ("CPU".to_string(),"int".to_string()),
            ("current".to_string(),"bool".to_string()),
            ("halted".to_string(),"bool".to_string()),
            ("qom_path".to_string(),"str".to_string()),
            ("thread_id".to_string(),"int".to_string()),
            ("arch".to_string(),"CpuInfoArch".to_string())
        ]
    ), result);
}

named!(union_base<&[u8], Vec<(String,String)> >,
    chain!(
        tag!("'base':") ~
        blanks? ~
        fields: data_field_list~
        opt!(tag!(","))~
        blanks?,
        ||{
            fields
        }
    )
);

named!(struct_base<&[u8], String>,
    chain!(
        tag!("'base': ") ~
        base: quoted_string ~
        opt!(tag!(","))~
        blanks?,
        ||{
            base
        }
    )
);
/*
named!(data<Vec<String> >,
    chain!(
        tag!("'data': ") ~
        data: data_field_list~
        tag!(",")~
        blanks?,
        ||{
            data
        }
    )
);
*/
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
        blanks? ~
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
pub struct Struct{
    pub name: String,
    pub fields: Vec<(String,String)>,
    pub base: Option<String>, //Fields from the base class
}

impl Struct{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        //println!("Input to Struct: {:?}", String::from_utf8_lossy(input));

        //Check if base is first. Sometimes it comes first and sometimes data comes first
        let base_first = struct_base(input);
        match base_first {
            nom::IResult::Done(remaining, base) => {
                //println!("base first: {:?}", String::from_utf8_lossy(remaining));
                chain!(
                    remaining,
                    //dbg!(tag!("'data': ")) ~
                    tag!("'data'")~
                    opt!(blanks)~
                    tag!(":")~
                    opt!(blanks)~
                    data: data_field_list~
                    tag!(",")?~
                    blanks?,
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
                //println!("Data first input: {:?}", String::from_utf8_lossy(input));
                chain!(
                    input,
                    //dbg!(tag!("'data': ")) ~
                    tag!("'data'")~
                    opt!(blanks)~
                    tag!(":")~
                    opt!(blanks)~
                    data: data_field_list~
                    opt!(tag!(","))~
                    blanks?~
                    base: opt!(struct_base),
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
    fn to_string(self)->String{
        let mut struct_fields:Vec<String> = Vec::new();

        if let Some(b) = self.base{
            struct_fields.push(format!("base: {}", b));
        };

        for f in self.fields.clone(){
            struct_fields.push(format!("pub {name}:{type}",
                name=f.0.replace("-", "_"),
                type=f.1.replace("str", "String")
                .replace("int", "i64")
            ));
        }


        format!(r#"
            #[derive(Debug, Serialize, Deserialize)]
            pub struct {name}{{
                {fields}
            }}
            "#, name=self.name, fields=struct_fields.join(","))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command{
    pub name: String,
    pub fields: Option<Vec<(String,String)>>,
    pub gen: Option<bool>,
    pub returns: Option<QemuReturnType>, //Either a type or a list of type
}

impl Command{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        //println!("Input to Command: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            opt!(tag!("'data': "))~
            data: opt!(data_field_list)~
            opt!(tag!(","))~
            opt!(blanks)~
            gen: opt!(gen) ~
            returns_list: parse_return_list ? ~
            returns_string: parse_return_string ? ~
            blanks?,
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
    //TODO Put this in a mod of just qemu commands
    fn to_string(self)->String{
        let mut struct_fields:Vec<String> = Vec::new();
        let mut impl_fields:Vec<String> = Vec::new();
        let mut impl_input:Vec<String> = Vec::new();

        if let Some(f) = self.fields{
            for field in f{
                let name = field.0.replace("-", "_");
                let field_type =field.1.replace("str", "String")
                    .replace("int", "i64");

                struct_fields.push(format!("pub {name}:{type}", name=name, type=field_type));
                impl_fields.push(format!("{name}:{name}", name=name));
                impl_input.push(format!("{name}:{type}",name=name, type=field_type));
            }
        }

        if let Some(r) = self.returns{
            match r{
                QemuReturnType::List(l) => {
                    struct_fields.push(format!("#[serde(skip_serializing)]\nreturns:{}", l.replace("str", "String")));
                    impl_fields.push(format!("returns:{}", l.replace("str", "String")));
                    impl_input.push(format!("{name}:Vec<{type}>",name=l, type=l));
                },
                QemuReturnType::String(s) => {
                    struct_fields.push(format!("#[serde(skip_serializing)]\nreturns:{}", s.replace("str", "String")));
                    impl_fields.push(format!("returns:{}", s.replace("str", "String")));
                    impl_input.push(format!("{name}:{type}",name=s, type=s));
                },
            }
        }

        let mut gen = String::new();
        if let Some(g) = self.gen{
            struct_fields.push("gen: bool".to_string());
        }

        format!(r#"
        #[derive(Debug, Serialize, Deserialize)]
        pub struct {name} {{
            execute: String,
            {fields}
        }}
        impl {name} {{
            pub fn new({impl_input})->{name}{{
                {name}{{
                    execute: "{name}".to_string(),
                    {impl_fields}
                }}
            }}
        }}
        "#,
        name=self.name.replace("-", "_"),
        fields=struct_fields.join(","),
        impl_fields=impl_fields.join(","),
        impl_input=impl_input.join(",")
    )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Union{
    pub name: String,
    //pub base: Option<Vec<String>>,
    pub discriminator: Option<String>,
    pub data: Vec<(String,String)>,

}

impl Union{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        //println!("Input to Union: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            //base: opt!(union_base) ~
            discriminator_name: opt!(discriminator) ~
            //dbg!(tag!("'data':"))~
            tag!("'data'")~
            opt!(blanks)~
            tag!(":")~
            opt!(blanks)~
            //dbg!(opt!(blanks))~
            fields: data_field_list~
            blanks?,
            ||{
                Union{
                    name: name,
                    //base: base,
                    discriminator: discriminator_name,
                    data: fields,
                }
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Enum{
    pub name: String,
    pub fields: Vec<String>,
}

impl Enum{
    fn parse(input: & [u8], name: String) -> nom::IResult<&[u8], Self> {
        //println!("Input to Enum: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            tag!("'data'")~
            opt!(blanks)~
            tag!(":")~
            opt!(blanks)~
            fields: enum_list~
            blanks?,
            ||{
                Enum{
                    name: name,
                    fields: fields,
                }
            }
        )
    }
    fn to_string(self)->String{
        format!(r#"
            #[derive(Debug, Serialize, Deserialize)]
            pub enum {} {{
                {fields}
            }}"#, self.name, fields=self.fields.into_iter().map(|s| s.replace("-", "_")).collect::<Vec<String>>().join(","))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum QemuReturnType{
    List(String), //A list type
    String(String), // Returns a single thing
}

#[derive(Debug, Eq, PartialEq)]
pub enum QemuType{
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
            fields: field_pair~
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
                        fields: vec![
                            ("compress-level".to_string(), "int".to_string()),
                            ("compress-threads".to_string(),"int".to_string()),
                            ("decompress-threads".to_string(),"int".to_string()),
                            ("x-cpu-throttle-initial".to_string(),"int".to_string()),
                            ("x-cpu-throttle-increment".to_string(),"int".to_string())
                        ], base: None })
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
fn test_section(){
    let input = r#"##
# @InputButton
#
# Button of a pointer input device (mouse, tablet).
#
# Since: 2.0
##
{ 'enum'  : 'InputButton',
  'data'  : [ 'left', 'middle', 'right', 'wheel-up', 'wheel-down' ] }"#;
    let result = Section::parse(input.as_bytes());
    println!("test_section result: {:?}", result);
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
pub struct Section {
    pub description: Vec<String>,
    pub qemu_type: QemuType,
}

impl Section{
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self>{
        //println!("Section parse input: {:?}", String::from_utf8_lossy(input));
        chain!(
            input,
            comments: comment_block~
            tag!("{")~
            blanks? ~
            qemu_type: call!(QemuType::parse) ~
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

pub fn print_section(s: Section){
    match s.qemu_type{
        QemuType::Struct(s) => {
            //TODO: Write these to structs/mod.rs
            println!("{}", s.to_string());
        },
        QemuType::Command(c) => {
            //TODO: Write these to commands/mod.rs
            println!("{}", c.to_string());
        },
        QemuType::Enum(e) => {
            //TODO: Write these to enums/mod.rs
            println!("{}", e.to_string());
        },
        QemuType::Include{name} => {
            //Download and parse these
            println!("//{}", name.to_string());
        },
        QemuType::Union(u) => {

        },
        QemuType::Unknown => {},
    }
}

pub fn parse_sections(input: &[u8])-> nom::IResult<&[u8], Vec<Section>>{
    chain!(
        input,
        sections: many0!(call!(Section::parse)),
        ||{
            sections
        }
    )
}
