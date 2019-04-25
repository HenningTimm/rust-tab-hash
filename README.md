# tab-hash -- Tabulation Hashing for Rust

This crate offers rust implementations of simple and twisted tabulation hashing for 32-bit integer values.

## Example

```rust
use tab_hash;

fn main() {
    let keys = vec![0, 8, 15, 47, 11];
    let tab_simple = Tab32Simple::new();
    for k in keys {
        println!("{}", tab_simple.hash(k));
    }
}
```

