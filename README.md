A fork project of [thehydroimpulse/thrust: Thrift RPC in Rust (Async I/O) http://thehydroimpulse.github.io/thrust](https://github.com/thehydroimpulse/thrust)  to integrate with [tokio-rs/tokio: Asynchronous I/O for Rust](https://github.com/tokio-rs/tokio).

# Status
UNDER DEVELOPMENT, NOT READY TO USE

## Thrift IDL Support
### Types

* [x] `bool`
* [x] `byte`
* [x] `i8`
* [x] `i16`
* [x] `i32`
* [x] `i64`
* [x] `double`
* [x] `string`
* [x] `bynary`
* [x] `list`
* [x] `set`
* [x] `map`

### Constants

* [x] `int`
* [x] `double`
* [x] `literal`
* [ ] `list`
* [ ] `map`


### Other Directives

* [x] comment
* requiredness
  + [x] `required`
  + [x] `optional`
  + [ ] implicit (always infered to `required`)
* [x] `namespace`
* [ ] `include`
* [x] `const` (see [Constants](#constants) for concrete supported literal)
* [x] `typedef`
* [x] `struct`
* [x] `enum`
  + [x] `VARIANT = n`
* [ ] `union`
* [x] `exception`
* [x] `service`
  + [x] `extends`
  + [x] `function`
    - [ ] `oneway`
    - [x] `void`
    - [ ] `throws`
    - [x] `required`
    - [x] `optional`

and at `service` , exception handling is not so matured that the server may hung when it received corrupted data.

## Code Generation

* [x] Command (`tokio-thrift` command)
* ~~[x] compiler plugin (`thrift!` , `thrift_file!` macro)~~ abandaned because not so useful
* [x] build.rs (see [example build.rs](examples/simple_server_client/build.rs))

## Thrift Implementation

* protocol
  + [x] binary protocol
* transport
  + [x] tokio integrated async TCP transport (framed transport)

Currently, framed transport supports only binary protocol.

# Installing
## using CLI
Note again that this project is NOT READY TO USE.

First, checkout this project and install the binary

```
git clone https://github.com/KeenS/tokio-thrift
cd tokio-thrift/tokio-thrift-bin
cargo install
```

then, run this command in your project to generate rust file

```
tokio-thrift your_file.thrift src/
```

## using build.rs

see [example build.rs](examples/simple_server_client/build.rs)



and in both cases, write below at your Cargo.toml

```toml
[dependencies]
tokio-thrift = {path = "path/to/tokio-thrift/tokio-thrift-lib"}
```

# More

see [examples](examples).

# License
MIT. See [LICENSE](LICENSE).
