//! This crate offers rust implementations of simple and twisted tabulation hashing for 32-bit integer values.
//!
//! Instatiating `Tab32Simple` or `Tab32Twisted` will initialize a table and
//! create a random hash function from the respective hash family.
//! The hash value of a 32-bit integer can be computed by calling its `hash` method.
//!
//! # Example:
//!
//! ```rust
//! use tab_hash::Tab32Simple;
//!
//! let keys = vec![0, 8, 15, 47, 11];
//! let simple = Tab32Simple::new();
//! for k in keys {
//!     println!("{}", simple.hash(k));
//! }
//! ```
//!
//! To reprocude hashes, save the table used by the hash function and save it.
//! The function can be recreated using the `with_table` constructor.
//!
//! ```rust
//! use tab_hash::Tab32Twisted;
//!
//! let key = 42;
//! let twisted_1 = Tab32Twisted::new();
//! let twisted_2 = Tab32Twisted::with_table(twisted_1.get_table());
//! let twisted_3 = Tab32Twisted::new();
//! assert_eq!(twisted_1.hash(key), twisted_2.hash(key));
//! assert_ne!(twisted_1.hash(key), twisted_3.hash(key));
//! ```
//!
//! # Note:
//! These hash functions do not implement the `std::hash::Hasher` trait,
//! since they do not work on arbitrary length byte streams.
//!
//! # Literature:
//! This implementation is based on the articles of Mihai Patrascu and Mikkel Thorup:
//! - [Simple Tabulation Hashing](http://dx.doi.org/10.1145/1993636.1993638)
//! - [Twisted Tabulation Hashing](https://doi.org/10.1137/1.9781611973105.16)
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Split up a 32bit number into 8bit chunks
fn byte_chunks_32(x: u32) -> [u8; 4] {
    [
        (x & 0x0000_00FF) as u8,
        ((x & 0x0000_FF00) >> 8) as u8,
        ((x & 0x00FF_0000) >> 16) as u8,
        ((x & 0xFF00_0000) >> 24) as u8,
    ]
}

/// Split up a 64bit number into 8bit chunks
fn byte_chunks_64(x: u64) -> [u8; 8] {
    [
        (x & 0x0000_0000_0000_00FF) as u8,
        ((x & 0x0000_0000_0000_FF00) >> 8) as u8,
        ((x & 0x0000_0000_00FF_0000) >> 16) as u8,
        ((x & 0x0000_0000_FF00_0000) >> 24) as u8,
        ((x & 0x0000_00FF_0000_0000) >> 32) as u8,
        ((x & 0x0000_FF00_0000_0000) >> 40) as u8,
        ((x & 0x00FF_0000_0000_0000) >> 48) as u8,
        ((x & 0xFF00_0000_0000_0000) >> 56) as u8,
    ]
}

/// A universal hash function for 32-bit integers using simple tabulation.
///
/// Usage:
/// ```rust
/// use tab_hash::Tab32Simple;
///
/// let keys = vec![0, 8, 15, 47, 11];
/// let simple = Tab32Simple::new();
/// for k in keys {
///     println!("{}", simple.hash(k));
/// }
/// ```
#[derive(Clone, Deserialize)]
pub struct Tab32Simple {
    #[serde(deserialize_with = "tab32simple_from_vec")]
    table: [[u32; 256]; 4],
}

impl Tab32Simple {
    /// Create a new simple tabulation hash function with a random table.
    pub fn new() -> Self {
        Tab32Simple {
            table: Tab32Simple::initialize_table(),
        }
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn to_vec(&self) -> Vec<Vec<u32>> {
        let mut vec = Vec::with_capacity(4);
        for col in self.table.iter() {
            vec.push(col.to_vec());
        }
        vec
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn from_vec(table_data: Vec<Vec<u32>>) -> Self {
        let mut table = [[0_u32; 256]; 4];
        assert_eq!(table_data.len(), 4);
        for (i, column) in table_data.iter().enumerate() {
            assert_eq!(column.len(), 256);
            for (j, value) in column.iter().enumerate() {
                table[i][j] = *value;
            }
        }
        Tab32Simple { table }
    }

    /// Create a new simple tabulation hash function with a given table.
    pub fn with_table(table: [[u32; 256]; 4]) -> Self {
        Tab32Simple { table }
    }

    /// Generate a table of 32bit uints for simple tabulation hashing
    fn initialize_table() -> [[u32; 256]; 4] {
        let table: [[u32; 256]; 4] =
            array_init::array_init(|_| array_init::array_init(|_| rand::random()));
        table
    }

    /// Get the table used by this hash function.
    pub fn get_table(&self) -> [[u32; 256]; 4] {
        self.table
    }

    /// Compute simple tabulation hash value for a 32bit integer number.
    pub fn hash(&self, x: u32) -> u32 {
        let mut h: u32 = 0; // initialize hash values as 0

        for (i, c) in byte_chunks_32(x).iter().enumerate() {
            h ^= self.table[i as usize][*c as usize];
        }
        h
    }
}

/// Custom serialization converting nested array to a nested vec (cannot be derived)
fn tab32simple_from_vec<'de, D>(deserializer: D) -> Result<[[u32; 256]; 4], D::Error>
where
    D: Deserializer<'de>,
{
    let table_data: Vec<Vec<u32>> = Deserialize::deserialize(deserializer)?;

    let mut table = [[0_u32; 256]; 4];
    assert_eq!(table_data.len(), 4);
    for (i, column) in table_data.iter().enumerate() {
        assert_eq!(column.len(), 256);
        for (j, value) in column.iter().enumerate() {
            table[i][j] = *value;
        }
    }
    Ok(table)
}

#[derive(Clone, Serialize)]
struct _VecTab32Simple {
    table: Vec<Vec<u32>>,
}

impl Serialize for Tab32Simple {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        _VecTab32Simple {
            table: self.to_vec(),
        }
        .serialize(s)
    }
}

/// A universal hash function for 64-bit integers using simple tabulation.
///
/// Usage:
/// ```rust
/// use tab_hash::Tab64Simple;
///
/// let keys = vec![0, 8, 15, 47, 11];
/// let simple = Tab64Simple::new();
/// for k in keys {
///     println!("{}", simple.hash(k));
/// }
/// ```
#[derive(Clone, Deserialize)]
pub struct Tab64Simple {
    #[serde(deserialize_with = "tab64simple_from_vec")]
    table: [[u64; 256]; 8],
}

impl Tab64Simple {
    /// Create a new simple tabulation hash function with a random table.
    pub fn new() -> Self {
        Tab64Simple {
            table: Tab64Simple::initialize_table(),
        }
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn to_vec(&self) -> Vec<Vec<u64>> {
        let mut vec = Vec::with_capacity(8);
        for col in self.table.iter() {
            vec.push(col.to_vec());
        }
        vec
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn from_vec(table_data: Vec<Vec<u64>>) -> Self {
        let mut table = [[0_u64; 256]; 8];
        assert_eq!(table_data.len(), 8);
        for (i, column) in table_data.iter().enumerate() {
            assert_eq!(column.len(), 256);
            for (j, value) in column.iter().enumerate() {
                table[i][j] = *value;
            }
        }
        Tab64Simple { table }
    }

    /// Create a new simple tabulation hash function with a given table.
    pub fn with_table(table: [[u64; 256]; 8]) -> Self {
        Tab64Simple { table }
    }

    /// Generate a table of 64bit uints for simple tabulation hashing
    fn initialize_table() -> [[u64; 256]; 8] {
        let table: [[u64; 256]; 8] =
            array_init::array_init(|_| array_init::array_init(|_| rand::random()));
        table
    }

    /// Get the table used by this hash function.
    pub fn get_table(&self) -> [[u64; 256]; 8] {
        self.table
    }

    /// Compute simple tabulation hash value for a 64bit integer number.
    pub fn hash(&self, x: u64) -> u64 {
        let mut h: u64 = 0; // initialize hash values as 0

        for (i, c) in byte_chunks_64(x).iter().enumerate() {
            h ^= self.table[i as usize][*c as usize];
        }
        h
    }
}

/// Custom serialization converting nested array to a nested vec (cannot be derived)
fn tab64simple_from_vec<'de, D>(deserializer: D) -> Result<[[u64; 256]; 8], D::Error>
where
    D: Deserializer<'de>,
{
    let table_data: Vec<Vec<u64>> = Deserialize::deserialize(deserializer)?;

    let mut table = [[0_u64; 256]; 8];
    assert_eq!(table_data.len(), 8);
    for (i, column) in table_data.iter().enumerate() {
        assert_eq!(column.len(), 256);
        for (j, value) in column.iter().enumerate() {
            table[i][j] = *value;
        }
    }
    Ok(table)
}

#[derive(Clone, Serialize)]
struct _VecTab64Simple {
    table: Vec<Vec<u64>>,
}

impl Serialize for Tab64Simple {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        _VecTab64Simple {
            table: self.to_vec(),
        }
        .serialize(s)
    }
}

/// A universal hash function for 32-bit integers using twisted tabulation.
///
/// Usage:
/// ```rust
/// use tab_hash::Tab32Twisted;
///
/// let keys = vec![0, 8, 15, 47, 11];
/// let twisted = Tab32Twisted::new();
/// for k in keys {
///     println!("{}", twisted.hash(k));
/// }
/// ```
#[derive(Clone, Deserialize)]
pub struct Tab32Twisted {
    #[serde(deserialize_with = "tab32twisted_from_vec")]
    table: [[u64; 256]; 4],
}

impl Tab32Twisted {
    /// Create a new twisted tabulation hash function with a random table.
    pub fn new() -> Self {
        Tab32Twisted {
            table: Tab32Twisted::initialize_table(),
        }
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn to_vec(&self) -> Vec<Vec<u64>> {
        let mut vec = Vec::with_capacity(4);
        for col in self.table.iter() {
            vec.push(col.to_vec());
        }
        vec
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn from_vec(table_data: Vec<Vec<u64>>) -> Self {
        let mut table = [[0_u64; 256]; 4];
        assert_eq!(table_data.len(), 4);
        for (i, column) in table_data.iter().enumerate() {
            assert_eq!(column.len(), 256);
            for (j, value) in column.iter().enumerate() {
                table[i][j] = *value;
            }
        }
        Tab32Twisted { table }
    }

    /// Create a new twisted tabulation hash function with a given table.
    pub fn with_table(table: [[u64; 256]; 4]) -> Self {
        Tab32Twisted { table }
    }

    /// Generate a table of 64bit uints for twisted tabulation hashing
    fn initialize_table() -> [[u64; 256]; 4] {
        let table: [[u64; 256]; 4] =
            array_init::array_init(|_| array_init::array_init(|_| rand::random()));
        table
    }

    /// Get the table used by this hash function.
    pub fn get_table(&self) -> [[u64; 256]; 4] {
        self.table
    }

    /// Compute twisted tabulation hash value for a 32bit integer number.
    pub fn hash(&self, x: u32) -> u32 {
        let mut h: u64 = 0; // initialize hash values as 0
        let chunks = byte_chunks_32(x);
        for (i, c) in chunks[0..3].iter().enumerate() {
            h ^= self.table[i as usize][*c as usize];
        }
        // compute address for last chunk by XOring the lowest byte of the
        // current hash value with the content of the last chunk of the key
        let c = chunks[3] ^ (h & 0xFF) as u8;
        h ^= self.table[3][c as usize];
        // shift out the 32 low bits of the resulting hash
        h = h.overflowing_shr(32).0;

        h as u32
    }
}

/// Custom serialization converting nested array to a nested vec (cannot be derived)
fn tab32twisted_from_vec<'de, D>(deserializer: D) -> Result<[[u64; 256]; 4], D::Error>
where
    D: Deserializer<'de>,
{
    let table_data: Vec<Vec<u64>> = Deserialize::deserialize(deserializer)?;

    let mut table = [[0_u64; 256]; 4];
    assert_eq!(table_data.len(), 4);
    for (i, column) in table_data.iter().enumerate() {
        assert_eq!(column.len(), 256);
        for (j, value) in column.iter().enumerate() {
            table[i][j] = *value;
        }
    }
    Ok(table)
}

#[derive(Clone, Serialize)]
struct _VecTab32Twisted {
    table: Vec<Vec<u64>>,
}

impl Serialize for Tab32Twisted {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        _VecTab32Twisted {
            table: self.to_vec(),
        }
        .serialize(s)
    }
}

/// A universal hash function for 64-bit integers using twisted tabulation.
///
/// Usage:
/// ```rust
/// use tab_hash::Tab64Twisted;
///
/// let keys = vec![0, 8, 15, 47, 11];
/// let twisted = Tab64Twisted::new();
/// for k in keys {
///     println!("{}", twisted.hash(k));
/// }
/// ```
#[derive(Clone, Deserialize)]
pub struct Tab64Twisted {
    #[serde(deserialize_with = "tab64twisted_from_vec")]
    table: [[u128; 256]; 8],
}

impl Tab64Twisted {
    /// Create a new twisted tabulation hash function with a random table.
    pub fn new() -> Self {
        Tab64Twisted {
            table: Tab64Twisted::initialize_table(),
        }
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn to_vec(&self) -> Vec<Vec<u128>> {
        let mut vec = Vec::with_capacity(8);
        for col in self.table.iter() {
            vec.push(col.to_vec());
        }
        vec
    }

    /// Create a new simple tabulation hash function with a random table.
    pub fn from_vec(table_data: Vec<Vec<u128>>) -> Self {
        let mut table = [[0_u128; 256]; 8];
        assert_eq!(table_data.len(), 8);
        for (i, column) in table_data.iter().enumerate() {
            assert_eq!(column.len(), 256);
            for (j, value) in column.iter().enumerate() {
                table[i][j] = *value;
            }
        }
        Tab64Twisted { table }
    }

    /// Create a new twisted tabulation hash function with a given table.
    pub fn with_table(table: [[u128; 256]; 8]) -> Self {
        Tab64Twisted { table }
    }

    /// Generate a table of 128bit uints for twisted tabulation hashing
    fn initialize_table() -> [[u128; 256]; 8] {
        let table: [[u128; 256]; 8] =
            array_init::array_init(|_| array_init::array_init(|_| rand::random()));
        table
    }

    /// Get the table used by this hash function.
    pub fn get_table(&self) -> [[u128; 256]; 8] {
        self.table
    }

    /// Compute twisted tabulation hash value for a 64bit integer number.
    pub fn hash(&self, x: u64) -> u64 {
        let mut h: u128 = 0; // initialize hash values as 0
        let chunks = byte_chunks_64(x);
        for (i, c) in chunks[0..7].iter().enumerate() {
            h ^= self.table[i as usize][*c as usize];
        }
        // compute address for last chunk by XOring the lowest byte of the
        // current hash value with the content of the last chunk of the key
        let c = chunks[7] ^ (h & 0xFF) as u8;
        h ^= self.table[7][c as usize];
        // shift out the 64 low bits of the resulting hash
        h = h.overflowing_shr(64).0;

        h as u64
    }
}

/// Custom serialization converting nested array to a nested vec (cannot be derived)
fn tab64twisted_from_vec<'de, D>(deserializer: D) -> Result<[[u128; 256]; 8], D::Error>
where
    D: Deserializer<'de>,
{
    let table_data: Vec<Vec<u128>> = Deserialize::deserialize(deserializer)?;

    let mut table = [[0_u128; 256]; 8];
    assert_eq!(table_data.len(), 8);
    for (i, column) in table_data.iter().enumerate() {
        assert_eq!(column.len(), 256);
        for (j, value) in column.iter().enumerate() {
            table[i][j] = *value;
        }
    }
    Ok(table)
}

#[derive(Clone, Serialize)]
struct _VecTab64Twisted {
    table: Vec<Vec<u128>>,
}

impl Serialize for Tab64Twisted {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        _VecTab64Twisted {
            table: self.to_vec(),
        }
        .serialize(s)
    }
}

// Tests for private methods
#[test]
fn byte_chunking_32() {
    let random_bytes: [u8; 400] = array_init::array_init(|_| rand::random());
    for four_bytes in random_bytes.chunks(4) {
        let mut number = 0_u32;
        for byte in four_bytes.iter().rev() {
            number = (number << 8) | *byte as u32;
        }
        assert_eq!(four_bytes, byte_chunks_32(number));
    }
}

#[test]
fn byte_chunking_64() {
    let random_bytes: [u8; 480] = array_init::array_init(|_| rand::random());
    for four_bytes in random_bytes.chunks(8) {
        let mut number = 0_u64;
        for byte in four_bytes.iter().rev() {
            number = (number << 8) | *byte as u64;
        }
        assert_eq!(four_bytes, byte_chunks_64(number));
    }
}
