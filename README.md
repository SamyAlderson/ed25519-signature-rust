# ed25519-signature-rust

**Fast and secure Ed25519 digital signature implementation in Rust**

This library provides a fast and secure implementation of the Ed25519 digital signature algorithm in Rust.

## What it does

This library allows you to generate and verify Ed25519 signatures in Rust. The Ed25519 algorithm is a highly secure and efficient digital signature scheme, making it suitable for a wide range of applications.

## Install

To use this library, add the following line to your `Cargo.toml` file:
```toml
[dependencies]
ed25519-signature-rust = "0.1.0"
```
Then, run `cargo build` to build the library.

## Usage

Here's an example of how to use the library:
```rust
use ed25519_signature_rust::{Signature, PublicKey, PrivateKey};

fn main() {
    // Generate a private key
    let private_key = PrivateKey::generate().unwrap();

    // Generate a public key from the private key
    let public_key = public_key(&private_key).unwrap();

    // Sign a message
    let signature = private_key.sign(b"Hello, world!").unwrap();

    // Verify the signature
    assert!(public_key.verify(b"Hello, world!", &signature).unwrap());
}
```
## Build from source

To build the library from source, run the following command:
```bash
cargo build
```
## Run tests

To run the tests, run the following command:
```bash
cargo test
```
## Project structure

The project consists of the following files:

* `lib.rs`: The main library code.
* `tests.rs`: The test suite.
* `ed25519.rs`: The implementation of the Ed25519 algorithm.
* `utils.rs`: Utility functions.

## License

Copyright (c) 2026 SamyAlderson

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.