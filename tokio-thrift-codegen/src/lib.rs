extern crate handlebars;
extern crate rustc_serialize;

pub mod parser;
use std::io::{self, Write};
use std::collections::BTreeMap;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context, JsonRender};
use parser::{Ty, Namespace, Parser, Keyword, ConstValue};


#[derive(Debug)]
pub enum Error {
    Other,
    IO(io::Error),
    Parser(parser::Error),
    Generate(handlebars::RenderError),
    Eof,
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Error {
        Error::IO(val)
    }
}

impl From<parser::Error> for Error {
    fn from(val: parser::Error) -> Error {
        Error::Parser(val)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(val: handlebars::RenderError) -> Error {
        Error::Generate(val)
    }
}

pub fn find_rust_namespace(parser: &mut Parser) -> Result<Namespace, Error> {
    loop {
        let ns = parser.parse_namespace()?;

        if &*ns.lang == "rust" {
            return Ok(ns);
        } else {
            continue;
        }
    }
}

// define a custom helper
fn helper_ty_to_protocol(_: &Context,
                         h: &Helper,
                         _: &Handlebars,
                         rc: &mut RenderContext)
                         -> Result<(), RenderError> {
    let param = h.param(0)
        .ok_or(RenderError::new("Param 0 is required for to_protocol helper."))?;
    let rendered = param.value().render();
    let ty = Ty::from(rendered);
    let ret = ty.to_protocol();
    rc.writer.write(ret.as_bytes())?;
    Ok(())
}

fn helper_ty_to_rust(_: &Context,
                     h: &Helper,
                     _: &Handlebars,
                     rc: &mut RenderContext)
                     -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new("Param 0 is required for to_rust helper."))?;
    let rendered = param.value().render();
    let ty = Ty::from(rendered);
    let ret = ty.to_string();
    rc.writer.write(ret.as_bytes())?;
    Ok(())
}

fn helper_const_to_literal(_: &Context,
                     h: &Helper,
                     _: &Handlebars,
                     rc: &mut RenderContext)
                     -> Result<(), RenderError> {
    use parser::ConstValue::*;
    let param = h.param(0).ok_or(RenderError::new("Param 0 is required for to_rust helper."))?;
    let mut decoder = json::Decoder::new(param.value().clone());
    let v = ConstValue::decode(&mut decoder).expect("internal error: failed to decode json value");
    let ret = match v {
        Int(i) => i.to_string(),
        Double(d) => d.to_string(),
        String(s) => format!("{:?}", s),
        ty @ List | ty @ Map => panic!("not supported type {:?}", ty),
    };
    rc.writer.write(ret.as_bytes())?;
    Ok(())
}



fn helper_ty_expr(_: &Context,
                  h: &Helper,
                  _: &Handlebars,
                  rc: &mut RenderContext)
                  -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new("Param 0 is required for expr helper."))?;
    let rendered = param.value().render();
    let ty = Ty::from(rendered);
    let expr = match ty {
        Ty::String => "de.deserialize_str()".to_string(),
        Ty::Byte => "de.deserialize_u8()".to_string(),
        Ty::I16 => "de.deserialize_i16()".to_string(),
        Ty::I32 => "de.deserialize_i32()".to_string(),
        Ty::I64 => "de.deserialize_i64()".to_string(),
        Ty::Bool => "de.deserialize_bool()".to_string(),
        Ty::Double => "de.deserialize_f64()".to_string(),
        Ty::Ident(s) => format!("{}::deserialize(de)", s) ,
        _ => panic!("Unexpected type to deserialize_arg: {:?}.", ty),
    };
    rc.writer.write(expr.as_bytes())?;
    Ok(())
}

macro_rules! static_register {
    ($handlebar: expr, $name: expr, $file: expr) => {
        $handlebar.register_template_string($name, include_str!($file).to_string()).expect("tokio_thrift internal error: failed to register template");
    }
}

macro_rules! static_register_files {
    ($handlebar: expr $(, $name: expr)*) => {
        $(static_register!($handlebar, $name, concat!($name, ".hbs"));)*
    }
}


pub fn compile(mut parser: &mut Parser, wr: &mut Write) -> Result<(), Error> {
    let mut handlebars = Handlebars::new();
    static_register_files!(handlebars, "base", "service", "service_client", "service_server", "struct", "enum", "typedef", "const", "method");

    handlebars.register_helper("expr", Box::new(helper_ty_expr));
    handlebars.register_helper("to_protocol", Box::new(helper_ty_to_protocol));
    handlebars.register_helper("to_rust", Box::new(helper_ty_to_rust));
    handlebars.register_helper("to_literal", Box::new(helper_const_to_literal));


    let mut data: BTreeMap<String, Json> = BTreeMap::new();
    let namespace = find_rust_namespace(&mut parser).map(|n| n.module).unwrap_or("self".to_string());
    data.insert("namespace".to_string(), Json::String(namespace));

    write!(wr,
           "{}",
           handlebars.render("base", &data).expect("faled to render base file"))?;

    loop {

        if parser.lookahead_keyword(Keyword::Enum) {
            gen_enum(&mut parser, &mut data, wr, &mut handlebars)?
        } else if parser.lookahead_keyword(Keyword::Struct) {
            gen_struct(&mut parser, &mut data, wr, &mut handlebars)?
        } else if parser.lookahead_keyword(Keyword::Typedef) {
            gen_typedef(&mut parser, &mut data, wr, &mut handlebars)?
        } else if parser.lookahead_keyword(Keyword::Const) {
            gen_const(&mut parser, &mut data, wr, &mut handlebars)?
        } else if parser.lookahead_keyword(Keyword::Service) {
            gen_service(&mut parser, &mut data, wr, &mut handlebars)?
        } else {
            break;
        }
    }

    Ok(())
}

fn gen_enum(parser: &mut Parser, data: &mut BTreeMap<String, Json>, wr: &mut Write, handlebars: &mut Handlebars) -> Result<(), Error> {
    let enum_ = parser.parse_enum()?;
    let json = json::encode(&enum_)
        .ok()
        .and_then(|s| Json::from_str(&s).ok())
        .expect("internal error");
    data.insert("enum".to_string(), json);
    write!(wr, "{}", handlebars.render("enum", data)?)?;
    Ok(())
}


fn gen_struct(parser: &mut Parser, data: &mut BTreeMap<String, Json>, wr: &mut Write, handlebars: &mut Handlebars) -> Result<(), Error> {
    let struct_ = parser.parse_struct()?;
    let json = json::encode(&struct_)
        .ok()
        .and_then(|s| Json::from_str(&s).ok())
        .expect("internal error");
    data.insert("struct".to_string(), json);
    println!("{:?}", data);
    write!(wr, "{}", handlebars.render("struct", data)?)?;
    Ok(())
}

fn gen_typedef(parser: &mut Parser, data: &mut BTreeMap<String, Json>, wr: &mut Write, handlebars: &mut Handlebars) -> Result<(), Error> {
    let typedef = parser.parse_typedef()?;
    let json = json::encode(&typedef)
        .ok()
        .and_then(|s| Json::from_str(&s).ok())
        .expect("internal error");
    data.insert("typedef".to_string(), json);
    println!("{:?}", data);
    write!(wr, "{}", handlebars.render("typedef", data)?)?;
    Ok(())
}


fn gen_const(parser: &mut Parser, data: &mut BTreeMap<String, Json>, wr: &mut Write, handlebars: &mut Handlebars) -> Result<(), Error> {
    let const_ = parser.parse_const()?;
    let json = json::encode(&const_)
        .ok()
        .and_then(|s| Json::from_str(&s).ok())
        .expect("internal error");
    data.insert("const".to_string(), json);
    println!("{:?}", data);
    write!(wr, "{}", handlebars.render("const", data)?)?;
    Ok(())
}


fn gen_service(parser: &mut Parser, data: &mut BTreeMap<String, Json>, wr: &mut Write, handlebars: &mut Handlebars) -> Result<(), Error> {
    let service = parser.parse_service()?;
    let json = json::encode(&service)
        .ok()
        .and_then(|s| Json::from_str(&s).ok())
        .expect("internal error");
    data.insert("service".to_string(), json);
    write!(wr, "{}", handlebars.render("service", data)?)?;
    write!(wr, "{}", handlebars.render("service_client", data)?)?;
    write!(wr, "{}", handlebars.render("service_server", data)?)?;
    Ok(())
}
