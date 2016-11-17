extern crate rustc_serialize;
use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use std::char;
use std::str::from_utf8;
use nom::{alpha, digit, multispace, IResult, Err};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    String,
    Void,
    Byte,
    Bool,
    Binary,
    I8,
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
                &I8  => s.emit_enum_variant("i8", 5, 0, |_| Ok(())),
                &I16 => s.emit_enum_variant("i16", 6, 0, |_| Ok(())),
                &I32 => s.emit_enum_variant("i32", 7, 0, |_| Ok(())),
                &I64 => s.emit_enum_variant("i64", 8, 0, |_| Ok(())),
                &Double => s.emit_enum_variant("double", 8, 0, |_| Ok(())),
                &List(ref ty) => s.emit_enum_variant("list", 10, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        ty.encode(s)
                    }));
                    Ok(())
                }),
                &Set(ref ty) => s.emit_enum_variant("set", 11, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        ty.encode(s)
                    }));
                    Ok(())
                }),
                &Map(ref kty, ref vty) => s.emit_enum_variant("map", 12, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        kty.encode(s)
                    }));
                    try!(s.emit_enum_variant_arg(1, |s| {
                        vty.encode(s)
                    }));
                    Ok(())
                }),
                &Option(ref ty) => s.emit_enum_variant("option", 13, 1, |s| {
                    try!(s.emit_enum_variant_arg(0, |s| {
                        ty.encode(s)
                    }));
                    Ok(())
                }),
                // User-defined type.
                &Ident(ref string) => s.emit_enum_variant("ident", 14, 1, |s| {
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
            &Ty::I8 => "i8".to_string(),
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

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Document {
    pub headers: Vec<Header>,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Header {
    Include(Include),
    Namespace(Namespace),
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Definition {
    Const(Const),
    Typedef(Typedef),
    Enum(Enum),
    Struct(Struct),
    Union(Union),
    Exception(Exception),
    Service(Service),
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Include {
    pub path: String
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Service {
    pub extends: Option<String>,
    pub ident: String,
    pub methods: Vec<ServiceMethod>
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct ServiceMethod {
    pub oneway: bool,
    pub ident: String,
    pub ty: Ty,
    pub args: Vec<StructField>
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Enum {
    pub ident: String,
    pub variants: Vec<(String, Option<i64>)>
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Union {
    pub ident: String,
    pub fields: Vec<StructField>
}


#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Struct {
    pub ident: String,
    pub fields: Vec<StructField>
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Exception {
    pub ident: String,
    pub fields: Vec<StructField>
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Throws {
    pub fields: Vec<StructField>
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct StructField {
    pub seq: Option<i64>,
    pub optional: bool,
    pub ty: Ty,
    pub ident: String,
    pub value: Option<ConstValue>,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Typedef {
    pub ty: Ty,
    pub ident: String,
}

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


#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Namespace {
    pub lang: String,
    pub module: String
}



pub fn parse(input: &str) -> Result<Option<Document>, Err<&[u8], u32>> {
    match document(input.as_bytes()) {
        IResult::Done(i, d) => {
            println!("{}", from_utf8(i).unwrap());
            Ok(Some(d))
        },
        IResult::Incomplete(_) => Ok(None),
        IResult::Error(e) => Err(e),
    }
}


named!(document <Document>, chain!(
    headers: separated_list!(multispace, header) ~ multispace ~
        defs: separated_list!(multispace, definition),
    || Document {
        headers: headers,
        definitions: defs,
    }));

named!(header <Header>, alt!(
    include    => {Header::Include} |
    namespace  => {Header::Namespace}));

named!(include <Include>, chain!(
    tag!("include") ~ multispace ~
        file: literal,
    || Include{
        path: file,
    }));

named!(namespace <Namespace>, chain!(
    tag!("namespace") ~ multispace ~
        lang: identifier ~ multispace ~
        ns: identifier,
    || Namespace{
        lang: lang,
        module: ns,
    }));
named!(definition <Definition>, alt!(
    const_    => {Definition::Const}|
    typedef   => {Definition::Typedef}|
    enum_     => {Definition::Enum}|
    struct_   => {Definition::Struct}|
    union     => {Definition::Union}|
    exception => {Definition::Exception}|
    service   => {Definition::Service}));

named!(const_ <Const>, chain!(
    tag!("const") ~ multispace ~
        ty: field_type ~ multispace ~
        id: identifier ~ multispace ~
        tag!("=") ~ multispace ~
        value: const_value ~
        opt!(multispace) ~ opt!(list_separator),
    || Const {
        ident: id,
        ty: ty,
        value: value,
    }));

named!(typedef <Typedef>, chain!(
    tag!("typedef") ~ multispace ~
        ty: definition_type ~ multispace ~
        id: identifier,
    || Typedef{
        ty: ty,
        ident: id,
    }));

named!(enum_ <Enum>, chain!(
    tag!("enum") ~ multispace ~
        id: identifier ~ multispace ~
        variants: delimited!(
            tag!("{"),
            separated_list!(
                multispace,
                chain!(
                    variant: identifier ~
                        index: chain!(
                            multispace? ~
                                tag!("=") ~
                                multispace? ~
                                idx: int_constant, || idx)? ~
                        multispace ~
                        list_separator,
                    || (variant, index))),
            tag!("}")),
    || Enum{
        ident: id,
        variants: variants,
    }));

named!(struct_ <Struct>, chain!(
    tag!("struct") ~ multispace ~
        id: identifier ~ multispace ~
        tag!("{") ~ multispace ~
        fields: many0!(field) ~
        tag!("}") ,
    || Struct {
        ident: id,
        fields: fields,
    }));
named!(union <Union>, chain!(
    tag!("union")  ~ id: identifier ~  tag!("{") ~
        fields: many0!(field) ~
        tag!("}") ,
    || Union {
        ident: id,
        fields: fields,
    }));

named!(exception <Exception>, chain!(
    tag!("exception")  ~ id: identifier ~  tag!("{") ~
        fields: many0!(field) ~
        tag!("}") ,
    || Exception {
        ident: id,
        fields: fields,
    }));

named!(service <Service>, chain!(
    tag!("service")  ~ id: identifier ~
        ext: chain!(tag!("extends") ~ exid: identifier, || exid)? ~
        tag!("{") ~
        functions: many0!(function) ~
        tag!("}") ,
    || Service{
        extends: ext,
        ident: id,
        methods: functions,
    }));

named!(field <StructField>, chain!(
    idx: field_id? ~
        req: field_req? ~
        ty: field_type ~
        id: identifier ~
        value: chain!(tag!("=")? ~ v: const_value, || v)? ~
        list_separator?,
    || StructField {
        seq: idx,
        optional: req.unwrap_or(false),
        ty: ty,
        ident: id,
        value: value,
    }));

named!(field_id <i64>, chain!(id: int_constant ~ tag!(":"), || id));

named!(field_req <bool>, alt!(
    tag!("required") => {|_| false}|
    tag!("optional") => {|_| true}));

named!(function <ServiceMethod>, chain!(
    oneway: tag!("oneway")? ~
        ty: function_type ~
        id: identifier ~
        tag!("(") ~ args: many0!(field) ~ tag!(")") ~
        throws? ~
        list_separator?,
    || ServiceMethod{
        oneway: oneway.is_some(),
        ident: id,
        ty: ty,
        args: args}));

named!(function_type <Ty>, alt!(
    field_type |
    tag!("void") => {|_| Ty::Void}));

named!(throws <Throws>, chain!(
    tag!("throws") ~ fields: many0!(field),
    || Throws{fields: fields}));

named!(field_type <Ty>, alt!(
    identifier => {|i| Ty::Ident(i)} |
    base_type |
    container_type));

named!(definition_type <Ty>, alt!(
    base_type |
    container_type));

named!(base_type <Ty>, alt!(
    tag!("bool")   => {|_| Ty::Bool} |
    tag!("byte")   => {|_| Ty::Byte} |
    tag!("i8")     => {|_| Ty::I8} |
    tag!("i16")    => {|_| Ty::I16} |
    tag!("i32")    => {|_| Ty::I32} |
    tag!("i64")    => {|_| Ty::I64} |
    tag!("double") => {|_| Ty::Double} |
    tag!("string") => {|_| Ty::String} |
    tag!("binary") => {|_| Ty::Binary}));

named!(container_type <Ty>, alt!(map_type | set_type | list_type));

named!(map_type <Ty>, chain!(
    tag!("map") ~ tag!("<") ~ k: field_type ~ tag!(",") ~ v: field_type ~ tag!(">"),
    || Ty::Map(Box::new(k), Box::new(v))));

named!(set_type <Ty>, chain!(
    tag!("set") ~ tag!("<") ~ v: field_type ~ tag!(">"),
    || Ty::Set(Box::new(v))));

named!(list_type <Ty>, chain!(
    tag!("list") ~ tag!("<") ~ v: field_type ~ tag!(">"),
    || Ty::List(Box::new(v))));

named!(const_value <ConstValue>, alt!(
    int_constant    => {ConstValue::Int} |
    double_constant => {ConstValue::Double} |
    literal         => {ConstValue::String} |
    identifier      => {|_| panic!("not supported identifier")} |
    const_list |
    const_map));

named!(int_constant <i64>, chain!
       (sgn: sgn? ~ n: map_res!(digit, from_utf8),
        || sgn.unwrap_or(1) * n.parse::<i64>().unwrap()));

named!(double_constant <f64>, chain!(
    sgn: sgn? ~
        n: map!(digit, |d| from_utf8(d).expect("invalid utf8").parse::<i64>().unwrap()) ~
        frac: chain!(tag!(".") ~ f: digit, || f)? ~
        pow: chain!(alt!(tag!("E") | tag!("e")) ~ p: int_constant, || p) ?
    , || {
        let sgn = sgn.unwrap_or(1) as f64;
        let n = n as f64;
        let frac: Option<f64> = frac.map(|f| from_utf8(f).unwrap().parse().unwrap());
        let frac = frac.map(|f| f / f.log10()).unwrap_or(0.0);
        let pow  = pow.unwrap_or(1);
        let ret: f64 = ((sgn * (n + frac)) as f64).powi(pow as i32);
        ret
        }));

named!(sgn <i64>, alt!(
    tag!("+") => {|_| 1} |
    tag!("-") => {|_| -1}));

named!(const_list <ConstValue>, chain!(
    tag!("[") ~
        vs: many0!(chain!(v: const_value ~ list_separator?, || v)) ~
        tag!("]"),
    || ConstValue::List));

named!(const_map <ConstValue>, chain!(
    tag!("{") ~
        vs: many0!(chain!(k: const_value ~ tag!(":") ~ v: const_value ~ list_separator?, || v)) ~
        tag!("}"),
    || ConstValue::Map));

named!(literal <String>, map_res!(alt!(
    chain!(tag!("\"") ~ s: is_not!("\"") ~ tag!("\""), || s) |
    chain!(tag!("'")  ~ s: is_not!("'")  ~ tag!("'") , || s)),
                                  |s| from_utf8(s).map(|s| s.to_string())));

named!(identifier <String>, chain!(
    h: map_res!(alt!(alpha | tag!("_")), from_utf8) ~
        t: map!(many0!(
            map_res!(alt!(
                alpha | digit |
                tag!(".") | tag!("_")),
                     from_utf8)),
                |vs| {
                    let mut s = String::new();
                    for v in vs {
                        s += v
                    };
                    s
                }),
    || format!("{}{}", h, t)));

named!(list_separator, alt!(tag!(",") | tag!(";")));


#[test]
fn test_namespace() {
    assert_eq!(namespace(b"namespace rust aaaa").unwrap().1, Namespace {
        lang: "rust".to_string(),
        module: "aaaa".to_string(),
    });

    assert_eq!(namespace(b"namespace ruby Aaa").unwrap().1, Namespace {
        lang: "ruby".to_string(),
        module: "Aaa".to_string(),
    });
}


#[test]
fn test_identifier() {
    assert_eq!(identifier(b"aiueo").unwrap().1, "aiueo".to_string());
    assert_eq!(identifier(b"_aiueo").unwrap().1, "_aiueo".to_string());
    assert_eq!(identifier(b"_aiu3o").unwrap().1, "_aiu3o".to_string());
    assert_eq!(identifier(b"_aiu.o").unwrap().1, "_aiu.o".to_string());
}
