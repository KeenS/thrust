#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate tokio_thrift_codegen;
#[macro_use]
extern crate log;

use syntax::tokenstream::TokenTree;
use syntax::ext::base::{self, ExtCtxt, MacResult, DummyResult, get_single_str_from_tts};
use syntax::ext::quote::rt::Span;
use syntax::{ast, ptr};
use syntax::parse::{self, token, new_parser_from_source_str};
use syntax::util::small_vector::SmallVector;
use rustc_plugin::Registry;
use tokio_thrift_codegen::parser::Document;
use tokio_thrift_codegen::{compile, find_rust_namespace};
use std::io::{Write, Read};
use std::fs::File;

macro_rules! panictry {
    ($e: expr) => {
        match $e {
            Ok(e) => e,
            // FIXME: raise appropriate error
            Err(e) => panic!("error: {:?}", e),
        }
    }
}


fn codegen<'cx>(cx: &'cx mut ExtCtxt, text: String, file: String) -> Box<MacResult + 'cx> {
    let mut output = Vec::new();
    let doc = Document::parse(&text)
        .expect("failed to parse thrift file")
        .expect("EOF while parsing thrift file");
    {
        let ns = find_rust_namespace(&doc).expect("cannot find namespace");
        output.write_all(format!("mod {} {{", ns.module).as_ref())
            .expect("internal error failed to write the vec");
    }
    compile(doc, &mut output).expect("failed to generate code");
    output.write_all(format!("}}").as_ref()).expect("internal error failed to write the vec");
    let output = match std::str::from_utf8(&output) {
        Ok(s) => s,
        Err(_) => "",
    };

    trace!("{}", output);


    let parser = new_parser_from_source_str(cx.parse_sess(), file, output.to_string());

    struct ExpandResult<'a> {
        p: parse::parser::Parser<'a>,
    }
    impl<'a> base::MacResult for ExpandResult<'a> {
        fn make_items(mut self: Box<ExpandResult<'a>>) -> Option<SmallVector<ptr::P<ast::Item>>> {
            let mut ret = SmallVector::default();
            while self.p.token != token::Eof {
                match panictry!(self.p.parse_item()) {
                    Some(item) => ret.push(item),
                    None => {
                        panic!(self.p
                            .diagnostic()
                            .span_fatal(self.p.span,
                                        &format!("expected item, found `{}`",
                                                 self.p.this_token_to_string())))
                    }
                }
            }
            Some(ret)
        }
    }

    Box::new(ExpandResult { p: parser })

}

fn macro_thrift_file<'cx>(cx: &'cx mut ExtCtxt,
                          sp: Span,
                          tts: &[TokenTree])
                          -> Box<MacResult + 'cx> {

    let file = match get_single_str_from_tts(cx, sp, tts, "thrift_file!") {
        Some(f) => f,
        None => return DummyResult::expr(sp),
    };


    let mut text = String::new();
    File::open(&file)
        .expect(&format!("thrift file not found: {}", &file))
        .read_to_string(&mut text)
        .expect(&format!("failed to read file: {}", &file));

    codegen(cx, text, file)

}

fn macro_thrift<'cx>(cx: &'cx mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'cx> {

    let text = match get_single_str_from_tts(cx, sp, tts, "thrift!") {
        Some(f) => f,
        None => return DummyResult::expr(sp),
    };

    codegen(cx, text, "trift!".to_string())
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("thrift", macro_thrift);
    reg.register_macro("thrift_file", macro_thrift_file);
}
