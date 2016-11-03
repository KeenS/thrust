A fork project of [thehydroimpulse/thrust: Thrift RPC in Rust (Async I/O) http://thehydroimpulse.github.io/thrust](https://github.com/thehydroimpulse/thrust)  to integrate with [tokio-rs/tokio: Asynchronous I/O for Rust](https://github.com/tokio-rs/tokio).

# Status
UNDER DEVELOPMENT, NOT READY TO USE

* [ ] `const`
* [ ] `typedef`
* [x] `struct` (`optional` is not supported yet)
* [ ] `enum` (`VARIANT = n` is not supported yet)
* [ ] `union`
* [ ] `exception`
* [x] `service` (`optional` is not supported yet)

and at `service` , exception handling not so matured that the server may hung when received corrupted data

# Installing
Note again that this project is NOT READY TO USE.

First, checkout this project and install the binary

```
git clone https://github.com/KeenS/thrust
cd thrust
cargo install
```

then, run this command in your project to generate rust file

```
thrust your_file.thrift src/
```

and write this at your Cargo.toml

```toml
[dependencies]
thrust = {git = "path/to/thrust"}
thrust-tokio = {git = "path/to/thrust/thrust-tokio"}
```

# More

see [example](example).

# License
MIT. See [LICENSE](LICENSE).
