extern crate rustc_serialize;
use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use std::char;
use std::str::from_utf8;
use nom::{alpha, digit};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    String,
    Void,
    Byte,
    Bool,
    Binary,
    I16,
    I32,
    I64,
    Double,
    List(Box<Ty>),
    Set(Box<Ty>),
    Map(Box<Ty>, Box<Ty>),
    Option(Box<Ty>),
    // User-defined type.
    Ident(String)
}

impl From<String> for Ty {
    fn from(val: String) -> Ty {
        match &*val {
            "string" => Ty::String,
            "void" => Ty::Void,
            "byte" => Ty::Byte,
            "bool" => Ty::Bool,
            "binary" => Ty::Binary,
            "i16" => Ty::I16,
            "i32" => Ty::I32,
            "i64" => Ty::I64,
            "double" => Ty::Double,
            _ => Ty::Ident(val)
        }
    }
}

impl Decodable for Ty {
    fn decode<D: Decoder>(_d: &mut D) -> Result<Self, D::Error> {
        unimplemented!()
    }
}

impl Encodable for Ty {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        use self::Ty::*;
        s.emit_enum("Ty", |s| {
            match self {
                &String => s.emit_enum_variant("string", 0, 0, |_| Ok(())),
                &Void => s.emit_enum_variant("void", 1, 0, |_| Ok(())),
                &Byte => s.emit_enum_variant("byte", 2, 0, |_| Ok(())),
                &Bool => s.emit_enum_variant("bool", 3, 0, |_| Ok(())),
                &Binary => s.emit_enum_variant("binary", 4, 0, |_| Ok(())),
                &I16 => s.emit_enum_variant("i16", 5, 0, |_| Ok(())),
                &I32 => s.emit_enum_variant("i32", 6, 0, |_| Ok(())),
                &I64 => s.emit_enum_variant("i64", 7, 0, |_| Ok(())),
                &Double => s.emit_enum_variant("double", 8, 0, |_| Ok(())),
                &List(ref ty) => s.emit_enum_variant("list", 9, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        ty.encode(s)
                    }));
                    Ok(())
                }),
                &Set(ref ty) => s.emit_enum_variant("set", 10, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        ty.encode(s)
                    }));
                    Ok(())
                }),
                &Map(ref kty, ref vty) => s.emit_enum_variant("map", 11, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        kty.encode(s)
                    }));
                    try!(s.emit_enum_variant_arg(1, |s| {
                        vty.encode(s)
                    }));
                    Ok(())
                }),
                &Option(ref ty) => s.emit_enum_variant("option", 12, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        ty.encode(s)
                    }));
                    Ok(())
                }),
                // User-defined type.
                &Ident(ref string) => s.emit_enum_variant("ident", 13, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        s.emit_str(&string)
                    }));
                    Ok(())
                }),
            }
        })
    }
}

impl Ty {
    pub fn to_protocol(&self) -> String {
        match self {
            &Ty::String => "ThriftType::String".to_string(),
            &Ty::Void => "ThriftType::Void".to_string(),
            &Ty::Bool => "ThriftType::Bool".to_string(),
            &Ty::Byte => "ThriftType::Byte".to_string(),
            &Ty::Double => "ThriftType::Double".to_string(),
            &Ty::I16 => "ThriftType::I16".to_string(),
            &Ty::I32 => "ThriftType::I32".to_string(),
            &Ty::I64 => "ThriftType::I64".to_string(),
            &Ty::Map(_, _) => "ThriftType::Map".to_string(),
            &Ty::List(_) => "ThriftType::List".to_string(),
            &Ty::Set(_) => "ThriftType::Set".to_string(),
            &Ty::Binary => "ThriftType::List".to_string(),
            &Ty::Ident(ref s) => s.to_string(),
            t => panic!("Not compatible with ThriftType: {:?}", t)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &Ty::String => "String".to_string(),
            &Ty::Void => "()".to_string(),
            &Ty::Byte => "u8".to_string(),
            &Ty::Bool => "bool".to_string(),
            &Ty::Binary => "Vec<i8>".to_string(),
            &Ty::I16 => "i16".to_string(),
            &Ty::I32 => "i32".to_string(),
            &Ty::I64 => "i64".to_string(),
            &Ty::Double => "double".to_string(),
            &Ty::Option(ref t) => {
                let inner = t.to_string();
                format!("Option<{}>", inner)
            },
            &Ty::List(ref s) => {
                let inner = s.to_string();
                format!("Vec<{}>", inner)
            },
            &Ty::Set(ref s) => {
                let inner = s.to_string();
                format!("HashSet<{}>", inner)
            },
            &Ty::Map(ref a, ref b) => {
                let a = a.to_string();
                let b = b.to_string();
                format!("HashMap<{}, {}>", a, b)
            },
            &Ty::Ident(ref s) => {
                s.clone()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Include {
    pub path: String
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Service {
    pub ident: String,
    pub methods: Vec<ServiceMethod>
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct ServiceMethod {
    pub oneway: bool,
    pub ident: String,
    pub ty: Ty,
    pub args: Vec<StructField>
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Enum {
    pub ident: String,
    pub variants: Vec<String>
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Union {
    pub ident: String,
    pub fields: Vec<StructField>
}


#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Struct {
    pub ident: String,
    pub fields: Vec<StructField>
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Exception {
    pub ident: String,
    pub fields: Vec<StructField>
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Throws {
    pub fields: Vec<StructField>
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct StructField {
    pub seq: i16,
    pub optional: bool,
    pub ty: Ty,
    pub ident: String
}

#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Typedef(pub Ty, pub String);

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Const {
    pub ident: String,
    pub ty: Ty,
    pub value: ConstValue,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub enum ConstValue {
    Int(i64),
    Double(f64),
    String(String),
    // not yet
    List,
    // not yet
    Map,
}


#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Namespace {
    pub lang: String,
    pub module: String
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Keyword {
    Struct,
    Service,
    Enum,
    Namespace,
    Required,
    Optional,
    Oneway,
    Typedef,
    Throws,
    Exception,
    Include,
    Const,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Eq,
    Colon,
    SingleQuote,
    Dot,
    Semi,
    At,
    Comma,
    LCurly,
    RCurly,
    LAngle,
    RAngle,
    LParen,
    RParen,
    Number(i64),
    QuotedString(String),
    Ident(String),
    Keyword(Keyword),

    /// Useless comments.
    Comment,
    Whitespace,
    Eof,
    B,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Expected,
    MissingFieldAttribute,
    ExpectedNumber,
    ExpectedConstValue,
    ExpectedString,
    ExpectedKeyword(Keyword),
    ExpectedIdent,
    ExpectedToken(Token),
    NoMoreItems
}



named!(document, chain!(
    many0!(header) ~
    many0!(definition), || ()));

named!(header, alt!(include | namespace));
named!(include, chain!(tag!("include") ~ file: literal, || file));
named!(namespace <Namespace>, chain!(tag!("namespace") ~ tag!("rust") ~ ns: identifire, || identifier));
named!(definition, alt!(const_ | typedef | enum_ | struct_ | union | exception | service));
named!(const_ <Const>, chain!(
    tag!("const") ~ ty: field_type ~ id: identifier ~
        tag!("=") ~ value: const_value ~ opt!(list_separator),
    || (ty, id, value)));

named!(typedef <Typedef>, chain!(tag!("typedef") ~ ty: definition_type ~ id: identifier, || (ty, id)));
named!(enum_ <Enum>, chain!(tag!("enum") ~ id: identifier ~ tag!("{") ~
                     variants: many0!(chain!(
                         variant: identifier ~
                             index: chain!(tag!("=") ~ idx: int_constant, || idx)? ~
                             list_separator,
                         || (variant, index))) ~
                     tag!("}"),
                     || (id, variands)));
named!(struct_ <Struct>, chain!(tag!("struct") ~ id: identifier ~  tag!("{") ~
                       fields: many0!(field) ~
                       tag!("}") ,
                       || (id, fields)));
named!(union <Union>, chain!(tag!("union")  ~ id: identifier ~  tag!("{") ~
                     fields: many0!(field) ~
                     tag!("}") ,
                     || (id, fields)));

named!(exception <Exception>, chain!(tag!("exception")  ~ id: identifier ~  tag!("{") ~
                     fields: many0!(field) ~
                     tag!("}") ,
                     || (id, fields)));

named!(service <Service>, chain!(tag!("service")  ~ id: identifier ~
                       ext: chain!(tag!("extends") ~ exid: identifier, || exid)? ~
                       tag!("{") ~
                       functions: many0!(function) ~
                       tag!("}") ,
                       || (id, ext, functions)));

named!(field <StructField>, chain!(
    idx: field_id? ~
        req: field_req? ~
        ty: field_type ~
        id: identifier ~
        value: chain!(tag!("=")? ~ v: const_value, || v)? ~
        list_separator?,
    || (idx, req, ty, id, value)));

named!(field_id <i32>, chain!(id: int_constant ~ tag!(":"), || id));
named!(field_req <bool>, alt!(tag!("required") | tag!("optional")));
named!(function <ServiceMethod>, chain!(
    tag!("oneway")? ~
        ty: function_type ~
        id: identifier ~
        tag!("(") ~
        args: many0!(field) ~
        tag!(")") ~
        throws? ~
        list_separator?,
    || (ty, id, args)));
named!(function_type <Ty>, alt!(field_type | tag!("void")));
named!(throws <Throws>, chain!(tag!("throws") ~ fields: many0!(field), || fields));
named!(field_type <Ty>, alt!(identifier | base_type | container_type));
named!(definition_type <Ty>, alt!(base_type | container_type));
named!(base_type <Ty>, alt!(
    tag!("bool") | tag!("byte") | tag!("i8") | tag!("i16") |
    tag!("i32") | tag!("i64") | tag!("double") | tag!("string") |
    tag!("binary")));
named!(container_type <Ty>, alt!(map_type | set_type | list_type));
named!(map_type <Ty>, chain!(
    tag!("map") ~ tag!("<") ~
        k: field_type ~ tag!(",") ~ v: field_type ~
        tag!(">"),
    || (k, v)));
named!(list_type <Ty>, chain!(tag!("list") ~ tag!("<") ~ v: field_type ~ tag!(">"), || v));
named!(const_value <ConstValue>, alt!(int_constant | double_constant | literal |
                        identifier | const_list | const_map));
named!(int_constant <i64>, chain!(sgn: alt!(tag!("+") | tag!("-"))? ~ n: digit, || (sgn, n)));
named!(double_constant <f64>, chain!(
    sgn: alt!(tag!("+") | tag!("-"))? ~
        n: digit ~
        frac: chain!(tag!(".") ~ f: digit, || f)? ~
        pow: chain!(alt!(tag!("E") | tag!("e")) ~ p: int_constant, || p) ?
    , || (sgn, n, frac, pow)));
named!(const_list <ConstValue>, chain!(
    tag!("[") ~
        vs: many0!(chain!(v: const_value ~ list_separator?, || v)) ~
        tag!("]"),
    || vs));
named!(const_map <ConstValue>, chain!(
    tag!("{") ~
        vs: many0!(chain!(k: const_value ~ v: const_value ~ list_separator?, || v)) ~
        tag!("}"),
    || vs));
named!(literal <String>, alt!(chain!(
    tag!("\"") ~
        s: not!(char!('\"'))~
        tag!("\""),
    || s) |
                     chain!(
                         tag!("'") ~
                             s: not!(char!('\''))~
                             tag!("'"),
                         || s)));
named!(identifier <String>, chain!(
    h: map_res!(alt!(alpha | tag!("_")), from_utf8) ~
        t: map!(many0!(alt!(alpha | digit | tag!(".") | tag!("_"))), |vs| {
            let mut s = String::new();
            for v in vs {
                s + &from_utf8(v).expect("invalid utf8");
            };
            s
        }
        ), || format!("{}{}", h, t)));
named!(list_separator, alt!(tag!(",") | tag!(";")));
