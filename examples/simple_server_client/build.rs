extern crate tokio_thrift_codegen;

use tokio_thrift_codegen::{compile, find_rust_namespace};
use tokio_thrift_codegen::parser::Document;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;


fn main() {
    let dst = env::var_os("OUT_DIR").unwrap();
    let mut input = File::open("src/hello.thrift").expect("input file does not exist.");
    let mut s = String::new();
    input.read_to_string(&mut s).expect("file io error");
    let doc = Document::parse(&s)
        .expect("failed to parse thrift file")
        .expect("EOF while parsing thrift file");

    let module = {
        let ns = find_rust_namespace(&doc).expect("cannot find namespace");
        Path::new(&dst).join(&ns.module).with_extension("rs")
    };
    let mut output = File::create(module).expect("error creating the module.");

    compile(doc, &mut output).expect("failed to generate code");

}
