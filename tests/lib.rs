use array_init;
use rand;
use tab_hash::{Tab32Simple, Tab32Twisted, Tab64Simple, Tab64Twisted};

extern "C" {
    fn SimpleTab32(x: u32, H: &[[u32; 256]; 4]) -> u32;
    fn TwistedTab32(x: u32, H: &[[u64; 256]; 4]) -> u32;
    fn SimpleTab64(x: u64, H: &[[u64; 256]; 8]) -> u64;
    fn TwistedTab64(x: u64, H: &[[u128; 256]; 8]) -> u64;
}

#[test]
fn simple32_vs_reference_implementation() {
    for _ in 0..100 {
        let simple_tabhash = Tab32Simple::new();
        let seed = simple_tabhash.get_table();
        let random_keys: [u32; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            let rust_result = simple_tabhash.hash(*key);
            unsafe {
                let reference_result = SimpleTab32(*key, &seed);
                assert_eq!(reference_result, rust_result);
            }
        }
    }
}

#[test]
fn twisted32_vs_reference_implementation() {
    for _ in 0..10 {
        let twisted_tabhash = Tab32Twisted::new();
        let seed = twisted_tabhash.get_table();
        let random_keys: [u32; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            let rust_result = twisted_tabhash.hash(*key);
            unsafe {
                let reference_result = TwistedTab32(*key, &seed);
                eprintln!("{}", reference_result);
                assert_eq!(reference_result, rust_result);
            }
        }
    }
}

#[test]
fn simple64_vs_reference_implementation() {
    for _ in 0..100 {
        let simple_tabhash = Tab64Simple::new();
        let seed = simple_tabhash.get_table();
        let random_keys: [u64; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            let rust_result = simple_tabhash.hash(*key);
            unsafe {
                let reference_result = SimpleTab64(*key, &seed);
                assert_eq!(reference_result, rust_result);
            }
        }
    }
}

#[test]
fn twisted64_vs_reference_implementation() {
    for _ in 0..10 {
        let twisted_tabhash = Tab64Twisted::new();
        let seed = twisted_tabhash.get_table();
        let random_keys: [u64; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            let rust_result = twisted_tabhash.hash(*key);
            unsafe {
                let reference_result = TwistedTab64(*key, &seed);
                eprintln!("{}", reference_result);
                assert_eq!(reference_result, rust_result);
            }
        }
    }
}

#[test]
fn simple_vs_fixed_value() {
    // chunks           3        2        1        0
    let key = 0b_00000100_00000010_00000001_00000000;

    // assemble table for testing
    let mut byte_1 = [0; 256];
    let mut byte_2 = [0; 256];
    let mut byte_3 = [0; 256];
    let mut byte_4 = [0; 256];
    byte_1[0] = 7;
    byte_2[1] = 11;
    byte_3[2] = 13;
    byte_4[4] = 17;
    let table = [byte_1, byte_2, byte_3, byte_4];

    //      111
    // ^   1011
    // ^   1101
    // ^  10001
    // --------
    //    10000
    let result = 16;

    let simple_tabhash = Tab32Simple::with_table(table);
    assert_eq!(simple_tabhash.hash(key), result);
}

#[test]
fn twisted_vs_fixed_value() {
    // chunks           3        2        1        0
    let key = 0b_00000100_00000010_00000001_00000000;

    // assemble table for testing
    let mut byte_1 = [0; 256];
    let mut byte_2 = [0; 256];
    let mut byte_3 = [0; 256];
    let mut byte_4 = [0; 256];
    byte_1[0] = 7;
    byte_2[1] = 11;
    byte_3[2] = 13;
    byte_4[0b101] = 0x_00_00_00_01_80_00_00_00;
    let table = [byte_1, byte_2, byte_3, byte_4];

    //        111
    // ^     1011
    // ^     1101
    // ----------
    // h=    0001
    // c = chunks[3] ^ h = 0b_0000_0100 ^ 0b_0000_0001 = 0b0000_0101
    //
    //
    //      00000000_00000000_00000000_00000101
    // ^  1_10000000_00000000_00000000_00000000

    // ----------------------------------------
    //    1_10000000_00000000_00000000_00000001
    // >> 32
    //                                        1
    let result = 1;

    let simple_tabhash = Tab32Twisted::with_table(table);
    assert_eq!(simple_tabhash.hash(key), result);
}

#[test]
fn simple32_to_and_from_vec() {
    for _ in 0..1000 {
        let h = Tab32Simple::new();
        let table = h.to_vec();
        let h2 = Tab32Simple::from_vec(table);

        let t1 = h.get_table();
        let t2 = h2.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u32; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(h.hash(*key), h2.hash(*key));
        }
    }
}

#[test]
fn twisted32_to_and_from_vec() {
    for _ in 0..1000 {
        let h = Tab32Twisted::new();
        let table = h.to_vec();
        let h2 = Tab32Twisted::from_vec(table);

        let t1 = h.get_table();
        let t2 = h2.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u32; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(h.hash(*key), h2.hash(*key));
        }
    }
}

#[test]
fn simple64_to_and_from_vec() {
    for _ in 0..1000 {
        let h = Tab64Simple::new();
        let table = h.to_vec();
        let h2 = Tab64Simple::from_vec(table);

        let t1 = h.get_table();
        let t2 = h2.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u64; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(h.hash(*key), h2.hash(*key));
        }
    }
}

#[test]
fn twisted64_to_and_from_vec() {
    for _ in 0..1000 {
        let h = Tab64Twisted::new();
        let table = h.to_vec();
        let h2 = Tab64Twisted::from_vec(table);

        let t1 = h.get_table();
        let t2 = h2.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u64; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(h.hash(*key), h2.hash(*key));
        }
    }
}

#[test]
fn simple32_serialization() {
    for _ in 0..1000 {
        let hf = Tab32Simple::new();
        let serialized_hf = bincode::serialize(&hf).unwrap();
        let deserialized_hf: Tab32Simple = bincode::deserialize(&serialized_hf).unwrap();

        let t1 = hf.get_table();
        let t2 = deserialized_hf.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u32; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(hf.hash(*key), deserialized_hf.hash(*key));
        }
    }
}

#[test]
fn simple64_serialization() {
    for _ in 0..1000 {
        let hf = Tab64Simple::new();
        let serialized_hf = bincode::serialize(&hf).unwrap();
        let deserialized_hf: Tab64Simple = bincode::deserialize(&serialized_hf).unwrap();

        let t1 = hf.get_table();
        let t2 = deserialized_hf.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u64; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(hf.hash(*key), deserialized_hf.hash(*key));
        }
    }
}

#[test]
fn twisted32_serialization() {
    for _ in 0..1000 {
        let hf = Tab32Twisted::new();
        let serialized_hf = bincode::serialize(&hf).unwrap();
        let deserialized_hf: Tab32Twisted = bincode::deserialize(&serialized_hf).unwrap();

        let t1 = hf.get_table();
        let t2 = deserialized_hf.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u32; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(hf.hash(*key), deserialized_hf.hash(*key));
        }
    }
}

#[test]
fn twisted64_serialization() {
    for _ in 0..1000 {
        let hf = Tab64Twisted::new();
        let serialized_hf = bincode::serialize(&hf).unwrap();
        let deserialized_hf: Tab64Twisted = bincode::deserialize(&serialized_hf).unwrap();

        let t1 = hf.get_table();
        let t2 = deserialized_hf.get_table();
        for column in 0..4 {
            assert_eq!(t1[column].to_vec(), t2[column].to_vec());
        }

        let random_keys: [u64; 100] = array_init::array_init(|_| rand::random());
        for key in random_keys.iter() {
            assert_eq!(hf.hash(*key), deserialized_hf.hash(*key));
        }
    }
}
