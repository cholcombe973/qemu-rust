//extern crate serde_codegen;

use std::env;
use std::path::Path;

pub fn main() {
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

    //let out_dir = env::var_os("OUT_DIR").unwrap();

    //let src = Path::new("src/QApiConnection.rs.in");
    //let dst = Path::new(&out_dir).join("QApiConnection.rs");

    //serde_codegen::expand(&src, &dst).unwrap();
}
