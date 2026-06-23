[![Build Status](https://travis-ci.org/HenningTimm/rust-tab-hash.svg?branch=master)](https://travis-ci.org/HenningTimm/rust-tab-hash)
[![creates.io-version](https://img.shields.io/crates/v/tab-hash.svg)](https://crates.io/crates/tab-hash)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/tab-hash/badge.svg)](https://docs.rs/tab-hash)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.3936766.svg)](https://doi.org/10.5281/zenodo.3936766)

# tab-hash - Tabulation Hashing for Rust

This crate offers Rust implementations of simple, twisted, and mixed tabulation hashing for 32-bit and 64-bit integer values.

Instantiating `Tab32Simple`, `Tab32Twisted`, or `Tab32Mixed` (or their 64-bit counterparts) will initialize tables and
create a random hash function from the respective hash family.
The hash value of an integer key is computed by calling its `hash` method.

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

Mixed tabulation uses four 8-bit derived characters. `Tab32Mixed` performs 4 + 4 table lookups, while `Tab64Mixed` performs 8 + 4 table lookups.

## Literature:
This implementation is based on the articles of Mihai Pătraşcu and Mikkel Thorup:
- [Simple Tabulation Hashing](http://dx.doi.org/10.1145/1993636.1993638)
- [Twisted Tabulation Hashing](https://doi.org/10.1137/1.9781611973105.16)
- [Hashing for Statistics over k-Partitions](https://doi.org/10.1109/FOCS.2015.83)
- [Fast and Powerful Hashing Using Tabulation](https://arxiv.org/abs/1505.01523)


## Changelog

### Version 0.3.0 [2020-02-12]

Made all structs serializable and deserializable.
