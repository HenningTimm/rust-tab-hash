use array_init;
use rand;
use rustab::{Tab32Simple, Tab32Twisted};

extern "C" {
    fn SimpleTab32(x: u32, H: &[[u32; 256]; 4]) -> u32;
    fn TwistedTab32(x: u32, H: &[[u64; 256]; 4]) -> u32;
}

#[test]
fn simple_equal() {
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
fn twisted_equal() {
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
