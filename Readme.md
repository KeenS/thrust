A fork project of [thehydroimpulse/thrust: Thrift RPC in Rust (Async I/O) http://thehydroimpulse.github.io/thrust](https://github.com/thehydroimpulse/thrust)  to integrate with [tokio-rs/tokio: Asynchronous I/O for Rust](https://github.com/tokio-rs/tokio).

# Status
UNDER DEVELOPMENT, NOT READY TO USE

## Thrift IDL Support
### Types

* [x] `bool`
* [x] `byte`
* [ ] `i8`
* [x] `i16`
* [x] `i32`
* [x] `i64`
* [x] `double`
* [x] `string`
* [x] `bynary`
* [ ] `list`
* [ ] `set`
* [ ] `map`

### Constants

* [ ] `int`
* [ ] `double`
* [x] `literal`
* [ ] `list`
* [ ] `map`


### Other Directives

* [x] `namespace`
* [ ] `include`
* [x] `const` (see [Constants](#Constants) for concrete literal)
* [x] `typedef`
* [x] `struct`
  + [x] `required`
  + [x] `optional`
* [x] `enum`
  + [ ] `VARIANT = n`
* [ ] `union`
* [ ] `exception`
* [x] `service`
  + [ ] `extends`
  + [x] `function`
    - [ ] `oneway`
    - [x] `void`
    - [ ] `throws`
    - [x] `required`
    - [x] `optional`

and at `service` , exception handling is not so matured that the server may hung when it received corrupted data.

## Code Generation

* [x] Command (`tokio-thrift` command)
* [x] compiler plugin (`thrift!` , `thrift_file!` macro)

## Thrift Implementation

* protocol
  + [x] binary protocol
* transport
  + [x] tokio integrated async TCP transport (framed transport)

Currently, framed transport supports only binary protocol.

# Installing
Notice again that this project is NOT READY TO USE.

First, checkout this project and install the binary

```
git clone https://github.com/KeenS/tokio-thrift
cd tokio-thrift
cargo install
```

then, run this command in your project to generate rust file

```
tokio-thrift your_file.thrift src/
```

and write this at your Cargo.toml

```toml
[dependencies]
tokio-thrift = {path = "path/to/tokio-thrift"}
```

# More

see [example](example).

# License
MIT. See [LICENSE](LICENSE).
