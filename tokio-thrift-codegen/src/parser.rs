extern crate rustc_serialize;
use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use std::str::from_utf8;
use nom::{alpha, digit, multispace, eof, IResult, Err};

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
            "i8" => Ty::I8,
            "i16" => Ty::I16,
            "i32" => Ty::I32,
            "i64" => Ty::I64,
            "double" => Ty::Double,
            // TODO: ignore or implement list, set and map
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
    pub path: String,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Service {
    pub extends: Option<String>,
    pub ident: String,
    pub methods: Vec<ServiceMethod>,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct ServiceMethod {
    pub oneway: bool,
    pub ident: String,
    pub ty: Ty,
    pub args: Vec<StructField>,
    pub throws: Option<Vec<StructField>>,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Enum {
    pub ident: String,
    pub variants: Vec<Variant>,
}


#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Variant {
    pub ident: String,
    pub seq: Option<i64>,
}


#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Union {
    pub ident: String,
    pub fields: Vec<StructField>,
}


#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Struct {
    pub ident: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Exception {
    pub ident: String,
    pub fields: Vec<StructField>,
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
    List(Vec<ConstValue>),
    // not yet
    Map,
}


#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Namespace {
    pub lang: String,
    pub module: String,
}

impl Document {
    pub fn parse(input: &str) -> Result<Option<Self>, Err<&[u8], u32>> {
        match document(input.as_bytes()) {
            IResult::Done(i, d) => {
                println!("{}", from_utf8(i).unwrap());
                Ok(Some(d))
            },
            IResult::Incomplete(_) => Ok(None),
            IResult::Error(e) => Err(e),
        }
    }

    pub fn rearrange(&mut self) {
        // resolve `include`, field id, oneway and void, warn about unsupported feature and so on.
    }
}


named!(document <Document>, chain!(
    blank? ~
        headers: many0!(chain!(h: header ~ blank?, || h)) ~
        defs:  many0!(chain!(d: definition ~ blank?, || d)) ~
        eof
        ,
    || Document {
        headers: headers,
        definitions: defs,
    }));

named!(header <Header>, alt!(
    include    => {Header::Include} |
    namespace  => {Header::Namespace}));

named!(include <Include>, chain!(
    tag!("include") ~ blank ~
        file: literal,
    || Include{
        path: file,
    }));

named!(namespace <Namespace>, chain!(
    tag!("namespace") ~ blank ~
        lang: identifier ~ blank ~
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
    tag!("const") ~ blank ~
        ty: field_type ~ blank ~
        id: identifier ~ blank? ~
        tag!("=") ~ blank? ~
        value: const_value ~ blank? ~
        list_separator?,
    || Const {
        ident: id,
        ty: ty,
        value: value,
    }));

named!(typedef <Typedef>, chain!(
    tag!("typedef") ~ blank ~
        ty: definition_type ~ blank ~
        id: identifier,
    || Typedef{
        ty: ty,
        ident: id,
    }));

named!(enum_ <Enum>, chain!(
    tag!("enum") ~ blank ~
        id: identifier ~ blank? ~
        tag!("{") ~ blank? ~
        variants: many0!(chain!(
            variant: identifier ~
                index: chain!(
                    blank? ~
                        tag!("=") ~
                        blank? ~
                        idx: int_constant, || idx)? ~
                blank? ~
                list_separator? ~
                blank? ,
            || Variant{ident: variant, seq: index})) ~
            tag!("}"),
    || Enum{
        ident: id,
        variants: variants,
    }));

named!(struct_ <Struct>, chain!(
    tag!("struct") ~ blank ~
        id: identifier ~ blank? ~
        tag!("{") ~
        fields: many0!(chain!(blank? ~ f: field, || f)) ~
        blank? ~
        tag!("}") ,
    || Struct {
        ident: id,
        fields: fields,
    }));

named!(union <Union>, chain!(
    tag!("union") ~ blank ~
        id: identifier ~ blank? ~
        tag!("{") ~
        fields: many0!(chain!(blank? ~ f: field, || f)) ~
        blank? ~
        tag!("}") ,
    || Union {
        ident: id,
        fields: fields,
    }));

named!(exception <Exception>, chain!(
    tag!("exception") ~ blank ~
        id: identifier ~ blank? ~
        tag!("{") ~
        fields: many0!(chain!(blank? ~ f: field, || f)) ~
        blank? ~
        tag!("}") ,
    || Exception {
        ident: id,
        fields: fields,
    }));

named!(service <Service>, chain!(
    tag!("service")  ~ blank ~
        id: identifier ~ blank? ~
        ext: chain!(tag!("extends") ~ blank ~
                    exid: identifier, || exid)? ~
        blank? ~
        tag!("{") ~ blank? ~
        functions: many0!(chain!(f: function ~ blank?, || f)) ~
        tag!("}") ,
    || Service{
        extends: ext,
        ident: id,
        methods: functions,
    }));

named!(field <StructField>, chain!(
    idx: chain!(idx: field_id ~  blank?, || idx)? ~
        req: chain!(req: field_req ~ blank, || req)? ~
        ty: field_type ~ blank ~
        id: identifier ~ blank? ~
        value: chain!(tag!("=") ~ blank? ~
                      v: const_value, || v)? ~
        list_separator?
        ,
    || StructField {
        seq: idx,
       optional: req.unwrap_or(false),
        ty: ty,
        ident: id,
//        value: None,
        value: value,
    }));

named!(field_id <i64>, chain!(id: int_constant ~ blank? ~ tag!(":"), || id));

named!(field_req <bool>, alt!(
    tag!("required") => {|_| false}|
    tag!("optional") => {|_| true}));

named!(function <ServiceMethod>, chain!(
    oneway: chain!(tag!("oneway") ~ blank?, ||())? ~
        ty: function_type ~ blank ~
        id: identifier ~ blank? ~
        tag!("(") ~ blank? ~
        args: many0!(chain!(f: field ~ blank?, || f)) ~
        tag!(")")  ~
        throws: chain!(blank? ~ th: throws, || th)? ~
        chain!(blank? ~ list_separator, ||())?
        ,
    || {
        let oneway = oneway.is_some();
        ServiceMethod{
            oneway: oneway,
            ident: id,
            ty: ty,
            args: args,
            throws: throws,
        }}));

named!(function_type <Ty>, alt!(
    tag!("void") => {|_| Ty::Void} |
    field_type));

named!(throws < Vec<StructField> >, chain!(
    tag!("throws") ~ blank? ~
        tag!("(") ~ blank? ~
        fields: many0!(chain!(f: field ~ blank?, || f)) ~
        tag!(")"),
    || fields));

named!(field_type <Ty>, alt!(
    base_type |
    container_type |
    identifier => {|i| Ty::Ident(i)}));

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
    tag!("map") ~ blank? ~
        tag!("<") ~ blank? ~
        k: field_type ~ blank? ~
        tag!(",") ~ blank? ~
        v: field_type ~ blank? ~
        tag!(">"),
    || Ty::Map(Box::new(k), Box::new(v))));

named!(set_type <Ty>, chain!(
    tag!("set") ~ blank? ~
        tag!("<")  ~ blank? ~
        v: field_type ~ blank?
    ~ tag!(">"),
    || Ty::Set(Box::new(v))));

named!(list_type <Ty>, chain!(
    tag!("list") ~ blank? ~
        tag!("<")  ~ blank? ~
        v: field_type ~ blank?
    ~ tag!(">"),
    || Ty::List(Box::new(v))));

named!(const_value <ConstValue>, alt!(
    double_constant => {ConstValue::Double} |
    int_constant    => {ConstValue::Int} |
    literal         => {ConstValue::String} |
    identifier      => {|_| panic!("identifier not supported")} |
    const_list |
    const_map));

named!(int_constant <i64>, chain!
       (sgn: sgn? ~ n: map_res!(digit, from_utf8),
        || format!("{}{}", sgn.unwrap_or(""), n).parse::<i64>().unwrap()));

// buggy
named!(double_constant <f64>, chain!(
    // frac exists
    sgn: sgn? ~
        n: map_res!(digit, from_utf8) ~
        fp: alt!(chain!(tag!(".") ~
                        frac: map_res!(digit, from_utf8) ~
                        pow: chain!(alt!(tag!("E") | tag!("e")) ~ p: int_constant, || p)?
                        ,
                        || (Some(frac), pow))
                 |
                 chain!(
                     // frac doesn't exist
                     alt!(tag!("E") | tag!("e")) ~
                         pow: int_constant,
                     || (None, Some(pow))))
    , || {
        let sgn = sgn.unwrap_or("");
        let (frac, pow) = fp;
        let frac = frac.unwrap_or("0");
        let pow = pow.map(|i| format!("e{}", i)).unwrap_or("".to_string());
        let f = format!("{}{}.{}{}", sgn, n, frac, pow).parse::<f64>().expect("internal error: failed to parse double literal internally");
        f
    }));

named!(sgn <&str>, map_res!(alt!(tag!("+") | tag!("-")), from_utf8));

named!(const_list <ConstValue>, chain!(
    tag!("[") ~ blank? ~
        vs: many0!(chain!(
            v: const_value ~ blank? ~
                list_separator? ~ blank?, || v)) ~
        tag!("]"),
    || ConstValue::List(vs)));

named!(const_map <ConstValue>, chain!(
    tag!("{") ~ blank? ~
        vs: many0!(chain!(
            k: const_value ~ blank? ~
                tag!(":") ~  blank? ~
                v: const_value ~ blank? ~
                list_separator? ~ blank?, || v)) ~
        tag!("}"),
    || ConstValue::Map));


named!(literal <String>, map_res!(alt!(
    chain!(tag!("\"") ~ s: is_not!("\"") ~ tag!("\""), || s) |
    chain!(tag!("'")  ~ s: is_not!("'")  ~ tag!("'") , || s)),
                                  |v| from_utf8(v).map(|s| s.to_string())));

fn cat_vec(vs: Vec<&[u8]>) -> Vec<u8> {
    let mut ret = Vec::new();
    for v in vs {
        ret.extend_from_slice(v);
    }
    ret
}

named!(identifier <String>, chain!(
    h: map_res!(alt!(alpha | tag!("_")), from_utf8) ~
        t: map_res!(many0!(
            alt!(
                alpha |
                digit |
                is_a!("._"))),
                    |vs| from_utf8(&cat_vec(vs)).map(|s|s.to_string())),
    || format!("{}{}", h, t)));

named!(list_separator, alt!(tag!(",") | tag!(";")));


named!(blank <()>, map!(many1!(alt!(comment | map!(multispace, |_| ()))), |_|()));

named!(comment <()>, alt!(
    chain!(
        tag!("/*") ~
            take_until_and_consume!("*/"),
        ||()) |
    chain!(tag!("//") ~ take_until_and_consume!("\n"), ||()) |
    chain!(tag!("#") ~ take_until_and_consume!("\n"), ||())));




#[test]
fn test_document() {
    assert_eq!(document(b"include \"foo.thrift\" ").unwrap().1,
               Document{
                   headers: vec![Header::Include(Include {path: "foo.thrift".to_string()})],
                   definitions: vec![]}
    );

    assert_eq!(document(b"const i32 foo = 1;").unwrap().1,
               Document{
                   headers: vec![],
                   definitions: vec![Definition::Const(Const {ident: "foo".to_string(), ty: Ty::I32, value: ConstValue::Int(1)})]}
    );


    assert_eq!(document(b"
include \"foo.thrift\"

const i32 foo = 1

struct Foo {}

").unwrap().1,
               Document{
                   headers: vec![Header::Include(Include {path: "foo.thrift".to_string()})],
                   definitions: vec![Definition::Const(Const {ident: "foo".to_string(), ty: Ty::I32, value: ConstValue::Int(1)}),
                                     Definition::Struct(Struct {ident: "Foo".to_string(), fields: vec![],})]}
    );


}


#[test]
fn test_header() {
    assert_eq!(header(b"include \"foo.thrift\"").unwrap().1,
               Header::Include(Include {path: "foo.thrift".to_string()})
    );

    assert_eq!(header(b"namespace rust aaaa").unwrap().1,
               Header::Namespace(Namespace {
                   lang: "rust".to_string(),
                   module: "aaaa".to_string(),
               }));

}

#[test]
fn test_include() {
    assert_eq!(include(b"include \"foo.thrift\"").unwrap().1,
               Include {path: "foo.thrift".to_string()}
    );
}

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
fn test_definition() {
    assert_eq!(definition(b"const i32 foo = 1;").unwrap().1,
               Definition::Const(Const {ident: "foo".to_string(), ty: Ty::I32, value: ConstValue::Int(1)})
    );

    assert_eq!(definition(b"typedef string foo").unwrap().1,
               Definition::Typedef(Typedef {ty: Ty::String, ident: "foo".to_string()}));

    assert_eq!(definition(b"enum Foo {}").unwrap().1,
               Definition::Enum(Enum {
                   ident: "Foo".to_string(),
                   variants: vec![],
               }));

    assert_eq!(definition(b"struct Foo {}").unwrap().1,
               Definition::Struct(Struct {
                   ident: "Foo".to_string(),
                   fields: vec![],
               }));

    assert_eq!(definition(b"union Foo {}").unwrap().1,
               Definition::Union(Union {
                   ident: "Foo".to_string(),
                   fields: vec![],
               }));

    assert_eq!(definition(b"exception Foo {}").unwrap().1,
               Definition::Exception(Exception {
                   ident: "Foo".to_string(),
                   fields: vec![],
               }));
    assert_eq!(definition(b"service Foo {} ").unwrap().1,
               Definition::Service(Service {
                   extends: None,
                   ident: "Foo".to_string(),
                   methods: vec![],
               }));

}

#[test]
fn test_const() {
    assert_eq!(const_(b"const i32 foo = 1;").unwrap().1,
               Const {ident: "foo".to_string(), ty: Ty::I32, value: ConstValue::Int(1)}
    );
    assert_eq!(const_(b"const i32 foo = 1,").unwrap().1,
               Const {ident: "foo".to_string(), ty: Ty::I32, value: ConstValue::Int(1)}
    );
}


#[test]
fn test_typedef() {
    assert_eq!(typedef(b"typedef string foo").unwrap().1,
               Typedef {ty: Ty::String, ident: "foo".to_string()});
}


#[test]
fn test_enum() {
    assert_eq!(enum_(b"enum Foo {}").unwrap().1,
               Enum {
                   ident: "Foo".to_string(),
                   variants: vec![],
               });

    assert_eq!(enum_(b"enum Foo {
foo
}").unwrap().1,
               Enum {
                   ident: "Foo".to_string(),
                   variants: vec![Variant{ident:"foo".to_string(), seq: None}]
               });

    assert_eq!(enum_(b"enum Foo {
foo
}").unwrap().1,
               Enum {
                   ident: "Foo".to_string(),
                   variants: vec![Variant{ident: "foo".to_string(), seq: None}]
               });

    assert_eq!(enum_(b"enum Foo {
foo
bar
}").unwrap().1,
               Enum {
                   ident: "Foo".to_string(),
                   variants: vec![Variant {ident: "foo".to_string(), seq: None},
                                  Variant{ident: "bar".to_string(), seq: None}
                   ]
               });

    assert_eq!(enum_(b"enum Foo {
foo,
bar,
}").unwrap().1,
               Enum {
                   ident: "Foo".to_string(),
                   variants: vec![Variant{ident: "foo".to_string(), seq: None},
                                  Variant{ident: "bar".to_string(), seq: None}
                   ]
               });
    assert_eq!(enum_(b"enum Foo {
foo;
bar;
}").unwrap().1,
               Enum {
                   ident: "Foo".to_string(),
                   variants: vec![Variant{ident: "foo".to_string(), seq: None},
                                  Variant{ident: "bar".to_string(), seq: None}
                   ]
               });
}


#[test]
fn test_struct() {
    assert_eq!(struct_(b"struct Foo {}").unwrap().1,
               Struct {
                   ident: "Foo".to_string(),
                   fields: vec![],
               });

    assert_eq!(struct_(b"struct Foo {1: required string foo}").unwrap().1,
               Struct {
                   ident: "Foo".to_string(),
                   fields: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "foo".to_string(),
                           ty: Ty::String,
                           value: None,
                       }],
               });
}

#[test]
fn test_union() {
    assert_eq!(union(b"union Foo {}").unwrap().1,
               Union {
                   ident: "Foo".to_string(),
                   fields: vec![],
               });

    assert_eq!(union(b"union Foo {1: required string foo}").unwrap().1,
               Union {
                   ident: "Foo".to_string(),
                   fields: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "foo".to_string(),
                           ty: Ty::String,
                           value: None,
                       }],
               });
}


#[test]
fn test_exception() {
    assert_eq!(exception(b"exception Foo {}").unwrap().1,
               Exception{
                   ident: "Foo".to_string(),
                   fields: vec![],
               });

    assert_eq!(exception(b"exception Foo {1: required string foo}").unwrap().1,
               Exception{
                   ident: "Foo".to_string(),
                   fields: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "foo".to_string(),
                           ty: Ty::String,
                           value: None,
                       }],
               });
}

#[test]
fn test_service() {
    assert_eq!(service(b"service Foo {
}
").unwrap().1, Service {
extends: None,
ident: "Foo".to_string(),
methods: vec![],
});

    assert_eq!(service(b"service Foo {
void foo(),
}
").unwrap().1, Service {
extends: None,
ident: "Foo".to_string(),
methods: vec![
    ServiceMethod {
        oneway: false,
        ident: "foo".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    }
],
});

    assert_eq!(service(b"service Foo {
void foo();
}
").unwrap().1, Service {
extends: None,
ident: "Foo".to_string(),
methods: vec![
    ServiceMethod {
        oneway: false,
        ident: "foo".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    }
],
});

    assert_eq!(service(b"service Foo {
void foo()
void bar()
}
").unwrap().1, Service {
extends: None,
ident: "Foo".to_string(),
methods: vec![
    ServiceMethod {
        oneway: false,
        ident: "foo".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    },
    ServiceMethod {
        oneway: false,
        ident: "bar".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    }
],
});

    assert_eq!(service(b"service Foo {
void foo();
void bar();
}
").unwrap().1, Service {
extends: None,
ident: "Foo".to_string(),
methods: vec![
    ServiceMethod {
        oneway: false,
        ident: "foo".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    },
    ServiceMethod {
        oneway: false,
        ident: "bar".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    }
],
});

    assert_eq!(service(b"service Foo {
void foo(),
void bar(),
}
").unwrap().1, Service {
extends: None,
ident: "Foo".to_string(),
methods: vec![
    ServiceMethod {
        oneway: false,
        ident: "foo".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    },
    ServiceMethod {
        oneway: false,
        ident: "bar".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    }
],
});


    assert_eq!(service(b"service Foo extends some.Bar {
void foo(),
}
").unwrap().1, Service {
extends: Some("some.Bar".to_string()),
ident: "Foo".to_string(),
methods: vec![
    ServiceMethod {
        oneway: false,
        ident: "foo".to_string(),
        ty: Ty::Void,
        args: vec![],
        throws: None,
    },
],
});

}

#[test]
fn test_field() {
    assert_eq!(field(b"string foo;").unwrap().1,
               StructField {seq: None,
                            optional: false,
                            ident: "foo".to_string(),
                            ty: Ty::String,
                            value: None,
               });
    assert_eq!(field(b"1: string foo;").unwrap().1,
               StructField {seq: Some(1),
                            optional: false,
                            ident: "foo".to_string(),
                            ty: Ty::String,
                            value: None,
               });
    assert_eq!(field(b"1: i32 foo;").unwrap().1,
               StructField {seq: Some(1),
                            optional: false,
                            ident: "foo".to_string(),
                            ty: Ty::I32,
                            value: None,});
    assert_eq!(field(b"1: i32 foo = 3;").unwrap().1,
               StructField {seq: Some(1),
                            optional: false,
                            ident: "foo".to_string(),
                            ty: Ty::I32,
                            value: Some(ConstValue::Int(3)),
               });
    assert_eq!(field(b"2: required set<binary> foo,").unwrap().1,
               StructField {seq: Some(2),
                            optional: false,
                            ident: "foo".to_string(),
                            ty: Ty::Set(Box::new(Ty::Binary)),
                            value: None,
               });
    assert_eq!(field(b"3: optional string foo;").unwrap().1,
               StructField {seq: Some(3),
                            optional: true,
                            ident: "foo".to_string(),
                            ty: Ty::String,
                            value: None,
               });

}

#[test]
fn test_field_id() {
    assert_eq!(field_id(b"1:").unwrap().1, 1);
    assert_eq!(field_id(b"1 :").unwrap().1, 1);
}

#[test]
fn test_field_req() {
    assert_eq!(field_req(b"required").unwrap().1, false);
    assert_eq!(field_req(b"optional").unwrap().1, true);
}

#[test]
fn test_function() {
    assert_eq!(function(b"i32 foo();").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![],
                   throws: None,
               });

    assert_eq!(function(b"i32 foo(1: string bar);").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                   ],
                   throws: None,
               });
    assert_eq!(function(b"i32 foo(1: required string bar);").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                   ],
                   throws: None,
               });
    assert_eq!(function(b"void foo(1: required string bar),").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::Void,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                   ],
                   throws: None,
               });
    assert_eq!(function(b"oneway void foo(1: required string bar);").unwrap().1,
               ServiceMethod {
                   oneway: true,
                   ident: "foo".to_string(),
                   ty: Ty::Void,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                   ],
                   throws: None,
               });
    assert_eq!(function(b"oneway i32 foo(1: required string bar);").unwrap().1,
               ServiceMethod {
                   oneway: true,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                   ],
                   throws: None,
               });
    assert_eq!(function(b"i32 foo(1: required string bar; optional binary baz);").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                       StructField {
                           seq: None,
                           optional: true,
                           ident: "baz".to_string(),
                           ty: Ty::Binary,
                           value: None,
                       },
                   ],
                   throws: None,
               });
    assert_eq!(function(b"i32 foo(1: required string bar, 2: optional binary baz);").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                       StructField {
                           seq: Some(2),
                           optional: true,
                           ident: "baz".to_string(),
                           ty: Ty::Binary,
                           value: None,
                       },
                   ],
                   throws: None,
               });
    assert_eq!(function(b"i32 foo(1: required string bar) throws (1: list<i32> pee);").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                   ],
                   throws: Some(vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "pee".to_string(),
                           ty: Ty::List(Box::new(Ty::I32)),
                           value: None,
                       }
                   ]),
               });

    assert_eq!(function(b"i32 foo(1: required string bar) throws (1: list<i32> pee, 2: optional set<byte> poo),").unwrap().1,
               ServiceMethod {
                   oneway: false,
                   ident: "foo".to_string(),
                   ty: Ty::I32,
                   args: vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "bar".to_string(),
                           ty: Ty::String,
                           value: None,
                       },
                   ],
                   throws: Some(vec![
                       StructField {
                           seq: Some(1),
                           optional: false,
                           ident: "pee".to_string(),
                           ty: Ty::List(Box::new(Ty::I32)),
                           value: None,
                       },
                       StructField {
                           seq: Some(2),
                           optional: true,
                           ident: "poo".to_string(),
                           ty: Ty::Set(Box::new(Ty::Byte)),
                           value: None,
                       },

                   ]),
               });
}

#[test]
fn test_function_type() {
    assert_eq!(function_type(b"void").unwrap().1, Ty::Void);
    assert_eq!(function_type(b"list<i32>").unwrap().1, Ty::List(Box::new(Ty::I32)));
    // wicked but legal according to formal definition
    assert_eq!(function_type(b"list<void>").unwrap().1, Ty::List(Box::new(Ty::Ident("void".to_string()))));
}

#[test]
fn test_throws() {
    assert_eq!(throws(b"throws(1: string foo)").unwrap().1,
               vec![StructField {seq: Some(1),
                                 optional: false,
                                 ident: "foo".to_string(),
                                 ty: Ty::String,
                                 value: None,
               }]);
    assert_eq!(throws(b"throws( 1: string foo )").unwrap().1,
               vec![StructField {seq: Some(1),
                                 optional: false,
                                 ident: "foo".to_string(),
                                 ty: Ty::String,
                                 value: None,
               }]
    );
    assert_eq!(throws(b"throws(1: string foo, 2: optional i32 bar)").unwrap().1,
               vec![StructField {seq: Some(1),
                                 optional: false,
                                 ident: "foo".to_string(),
                                 ty: Ty::String,
                                 value: None,
               },
                    StructField {seq: Some(2),
                                 optional: true,
                                 ident: "bar".to_string(),
                                 ty: Ty::I32,
                                 value: None,
                    }]
    );
}

#[test]
fn test_field_type() {
    assert_eq!(field_type(b"i8").unwrap().1, Ty::I8);
    assert_eq!(field_type(b"list < i32 >").unwrap().1, Ty::List(Box::new(Ty::I32)));
    assert_eq!(field_type(b"aaaa").unwrap().1, Ty::Ident("aaaa".to_string()));
}

#[test]
fn test_definition_type() {
    assert_eq!(definition_type(b"i8").unwrap().1, Ty::I8);
    assert_eq!(definition_type(b"list < i32 >").unwrap().1, Ty::List(Box::new(Ty::I32)));
}

#[test]
fn test_base_type() {
    assert_eq!(base_type(b"bool").unwrap().1, Ty::Bool);
    assert_eq!(base_type(b"byte").unwrap().1, Ty::Byte);
    assert_eq!(base_type(b"i8").unwrap().1, Ty::I8);
    assert_eq!(base_type(b"i16").unwrap().1, Ty::I16);
    assert_eq!(base_type(b"i32").unwrap().1, Ty::I32);
    assert_eq!(base_type(b"i64").unwrap().1, Ty::I64);
    assert_eq!(base_type(b"double").unwrap().1, Ty::Double);
    assert_eq!(base_type(b"string").unwrap().1, Ty::String);
    assert_eq!(base_type(b"binary").unwrap().1, Ty::Binary);
}

#[test]
fn test_container_type() {
    assert_eq!(container_type(b"map<i32, bool>").unwrap().1, Ty::Map(Box::new(Ty::I32), Box::new(Ty::Bool)));
    assert_eq!(container_type(b"map < i32 , string >").unwrap().1, Ty::Map(Box::new(Ty::I32), Box::new(Ty::String)));
    assert_eq!(container_type(b"map<map<i32, bool>, map<bool, i32>>").unwrap().1,
               Ty::Map(Box::new(Ty::Map(Box::new(Ty::I32), Box::new(Ty::Bool))),
                       Box::new(Ty::Map(Box::new(Ty::Bool), Box::new(Ty::I32)))));
    assert_eq!(container_type(b"set<i32>").unwrap().1, Ty::Set(Box::new(Ty::I32)));
    assert_eq!(container_type(b"set < i32 >").unwrap().1, Ty::Set(Box::new(Ty::I32)));
    assert_eq!(container_type(b"set<set<i32>>").unwrap().1, Ty::Set(Box::new(Ty::Set(Box::new(Ty::I32)))));
    assert_eq!(container_type(b"list<i32>").unwrap().1, Ty::List(Box::new(Ty::I32)));
    assert_eq!(container_type(b"list < i32 >").unwrap().1, Ty::List(Box::new(Ty::I32)));
    assert_eq!(container_type(b"list<list<i32>>").unwrap().1, Ty::List(Box::new(Ty::List(Box::new(Ty::I32)))));
}

#[test]
fn test_map_type() {
    assert_eq!(map_type(b"map<i32, bool>").unwrap().1, Ty::Map(Box::new(Ty::I32), Box::new(Ty::Bool)));
    assert_eq!(map_type(b"map < i32 , string >").unwrap().1, Ty::Map(Box::new(Ty::I32), Box::new(Ty::String)));
    assert_eq!(map_type(b"map<map<i32, bool>, map<bool, i32>>").unwrap().1,
               Ty::Map(Box::new(Ty::Map(Box::new(Ty::I32), Box::new(Ty::Bool))),
                       Box::new(Ty::Map(Box::new(Ty::Bool), Box::new(Ty::I32)))));
}



#[test]
fn test_set_type() {
    assert_eq!(set_type(b"set<i32>").unwrap().1, Ty::Set(Box::new(Ty::I32)));
    assert_eq!(set_type(b"set < i32 >").unwrap().1, Ty::Set(Box::new(Ty::I32)));
    assert_eq!(set_type(b"set<set<i32>>").unwrap().1, Ty::Set(Box::new(Ty::Set(Box::new(Ty::I32)))));
}


#[test]
fn test_list_type() {
    assert_eq!(list_type(b"list<i32>").unwrap().1, Ty::List(Box::new(Ty::I32)));
    assert_eq!(list_type(b"list < i32 >").unwrap().1, Ty::List(Box::new(Ty::I32)));
    assert_eq!(list_type(b"list<list<i32>>").unwrap().1, Ty::List(Box::new(Ty::List(Box::new(Ty::I32)))));
}

#[test]
fn test_const_value() {
    assert_eq!(const_value(b"1 ").unwrap().1, ConstValue::Int(1));
    assert_eq!(const_value(b"-1 ").unwrap().1, ConstValue::Int(-1));
    assert_eq!(const_value(b"1.0 ").unwrap().1, ConstValue::Double(1.0));
    assert_eq!(const_value(b"-1.01 ").unwrap().1, ConstValue::Double(-1.01));
    assert_eq!(const_value(b"'aaa'").unwrap().1, ConstValue::String("aaa".to_string()));
    assert_eq!(const_value(b"\"aaa\"").unwrap().1, ConstValue::String("aaa".to_string()));
}

#[test]
fn test_int_constant() {
    assert_eq!(int_constant(b"1").unwrap().1,  1);
    assert_eq!(int_constant(b"-10").unwrap().1, -10);

}

#[test]
fn test_double_constant() {
    println!("{:?}", double_constant(b"-1.0 "));
    assert_eq!(double_constant(b"1.0 ").unwrap().1,  1.0);
    assert_eq!(double_constant(b"-0.01 ").unwrap().1, -0.01);
    assert_eq!(double_constant(b"-1.0e-2 ").unwrap().1, -0.01);
    assert_eq!(double_constant(b"-1E2 ").unwrap().1, -100.0);

}


#[test]
fn test_literal() {
    assert_eq!(literal(b"\"literal\"").unwrap().1, "literal".to_string());
    assert_eq!(literal(b"'literal'").unwrap().1, "literal".to_string());
}


#[test]
fn test_identifier() {
    assert_eq!(identifier(b"aiueo").unwrap().1, "aiueo".to_string());
    assert_eq!(identifier(b"_aiueo").unwrap().1, "_aiueo".to_string());
    assert_eq!(identifier(b"_aiu3o").unwrap().1, "_aiu3o".to_string());
    assert_eq!(identifier(b"_aiu.o").unwrap().1, "_aiu.o".to_string());
}


#[test]
fn test_comment() {
    assert_eq!(comment(b"# aaaaa
").unwrap().1, ());
    assert_eq!(comment(b"// aaaaa
").unwrap().1, ());
    assert_eq!(comment(b"/*aaa*/").unwrap().1, ());
    assert_eq!(comment(b"/*
aaa
*/").unwrap().1, ());
    assert_eq!(comment(b"/*
* aaa
*/").unwrap().1, ());
}
