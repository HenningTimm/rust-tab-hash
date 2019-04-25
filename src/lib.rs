//! This crate offers rust implementations of simple and twisted tabulation hashing for 32-bit integer values.
//!
//! # Example:
//!
//! ```rust
//! use tab_hash::Tab32Simple;
//!
//! fn main() {
//!     let keys = vec![0, 8, 15, 47, 11];
//!     let tab_simple = Tab32Simple::new();
//!     for k in keys {
//!         println!("{}", tab_simple.hash(k));
//!     }
//! }
//! ```
//!

use rand;

/// Split up a 32bit number into 8bit chunks
fn byte_chunks(x: u32) -> [u8; 4] {
    [
        (x & 0x000000FF) as u8,
        ((x & 0x0000FF00) >> 8) as u8,
        ((x & 0x00FF0000) >> 16) as u8,
        ((x & 0xFF000000) >> 24) as u8,
    ]
}

pub struct Tab32Simple {
    table: [[u32; 256]; 4],
}

impl Tab32Simple {
    pub fn new() -> Self {
        Tab32Simple {
            table: Tab32Simple::initialize_table(),
        }
    }

    pub fn with_table(table: [[u32; 256]; 4]) -> Self {
        Tab32Simple { table }
    }

    /// Generate a table of 32bit uints for simple tabulation hashing
    fn initialize_table() -> [[u32; 256]; 4] {
        let table: [[u32; 256]; 4] =
            array_init::array_init(|_| array_init::array_init(|_| rand::random()));
        table
    }

    pub fn get_table(&self) -> [[u32; 256]; 4] {
        self.table
    }

    /// Simple tabulation hash for a 32bit integer number.
    pub fn hash(&self, x: u32) -> u32 {
        let mut h: u32 = 0; // initialize hash values as 0

        for (i, c) in byte_chunks(x).iter().enumerate() {
            h ^= self.table[i as usize][*c as usize];
        }
        h
    }
}

pub struct Tab32Twisted {
    table: [[u64; 256]; 4],
}

impl Tab32Twisted {
    pub fn new() -> Self {
        Tab32Twisted {
            table: Tab32Twisted::initialize_table(),
        }
    }

    pub fn with_table(table: [[u64; 256]; 4]) -> Self {
        Tab32Twisted { table }
    }

    /// Generate a table of 64bit uints for twistedtabulation hashing
    fn initialize_table() -> [[u64; 256]; 4] {
        let table: [[u64; 256]; 4] =
            array_init::array_init(|_| array_init::array_init(|_| rand::random()));
        table
    }

    pub fn get_table(&self) -> [[u64; 256]; 4] {
        self.table
    }

    /// Twisted tabulation hash for a 32bit integer number.
    pub fn hash(&self, x: u32) -> u32 {
        let mut h: u64 = 0; // initialize hash values as 0
        let chunks = byte_chunks(x);
        for (i, c) in chunks[0..3].iter().enumerate() {
            h ^= self.table[i as usize][*c as usize];
        }
        let c = chunks[3] ^ (h & 0xFF) as u8;
        h ^= self.table[3][c as usize];
        h = h.overflowing_shr(32).0;

        h as u32
    }
}
