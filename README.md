[![Build Status](https://travis-ci.org/HenningTimm/rust-tab-hash.svg?branch=master)](https://travis-ci.org/HenningTimm/rust-tab-hash)
[![creates.io-version](https://img.shields.io/crates/v/tab-hash.svg)](https://crates.io/crates/tab-hash)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/tab-hash/badge.svg)](https://docs.rs/tab-hash)

# tab-hash - Tabulation Hashing for Rust

This crate offers rust implementations of simple and twisted tabulation hashing for 32-bit und 64-bit integer values.

Instatiating `Tab32Simple` or `Tab32Twisted` (or their 64-bit counterparts) will initialize a table and
create a random hash function from the respective hash family.
The hash values of an integer key is computed by calling its `hash` method.

## Example:

```rust
use tab_hash::Tab32Simple;

fn main() {
    let keys = vec![0, 8, 15, 47, 11];
    let simple = Tab32Simple::new();
    for k in keys {
        println!("{}", simple.hash(k));
    }
}
```

To reproduce hashes, save the table used by the hash function.
The function can be recreated using the `with_table` constructor.

```rust
use tab_hash::Tab64Twisted;

fn main() {
    let key = 42;
    let twisted_1 = Tab64Twisted::new();
    let twisted_2 = Tab64Twisted::with_table(twisted_1.get_table());
    let twisted_3 = Tab64Twisted::new();
    assert_eq!(twisted_1.hash(key), twisted_2.hash(key));
    assert_ne!(twisted_1.hash(key), twisted_3.hash(key));
}
```

## Note:
These hash functions do not implement the `std::hash::Hasher` trait,
since they do not work on arbitrary length byte streams.

The 64-bit version of twisted tabulation hashing (`Tab64Twisted`) requires 128-bit operations (see [here](https://doi.org/10.1137/1.9781611973105.16)).

## Literature:
This implementation is based on the articles of Mihai Pătraşcu and Mikkel Thorup:
- [Simple Tabulation Hashing](http://dx.doi.org/10.1145/1993636.1993638)
- [Twisted Tabulation Hashing](https://doi.org/10.1137/1.9781611973105.16)
