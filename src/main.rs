#[macro_use]
extern crate log;
extern crate tokio_thrift_codegen;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::io::Read;
use std::fs::File;
use std::path::Path;
use tokio_thrift_codegen::{compile, find_rust_namespace};
use tokio_thrift_codegen::parser::parse;

const USAGE: &'static str = "
Thrust: Thrift compiler for Rust

Usage:
  tokio_thrift <input> <output>
  tokio_thrift --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_input: String,
    arg_output: String
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    debug!("{:?}", args);

    let mut input = File::open(args.arg_input).expect("input file does not exist.");
    let mut s = String::new();
    input.read_to_string(&mut s).expect("file io error");
    let doc = parse(&s)
        .expect("failed to parse thrift file")
        .expect("EOF while parsing thrift file");
    println!("{:?}", doc);

    let ns = find_rust_namespace(&doc).expect("cannot find namespace");

    let module = Path::new(&args.arg_output).join(&ns.module).with_extension("rs");
    let mut output = File::create(module).expect("error creating the module.");

    compile(&doc, &mut output).expect("failed to generate code");
}
