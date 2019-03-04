use std::num::ParseIntError;

use hmac::{Hmac, Mac};
use sha2::Sha256;

pub mod admin;
pub mod link;
pub mod user;

fn verify_hash(key: String, value: String, hash: String) -> bool {
    let mut mac = Hmac::<Sha256>::new_varkey(key.as_bytes()).unwrap();
    mac.input(value.as_bytes());

    let hash: Vec<u8> = decode_hex(&hash).unwrap();

    mac.verify(&hash).is_ok()
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod utils {
    mod verify_hash {
        use super::super::*;

        static GOOD_KEY: &str = "henlo world";
        static GOOD_VALUE: &str = "origin=asdf&dest=hosd&ts=1551678053";
        static GOOD_HASH: &str = "0064d5411c0be45db07661f221c9bff43cd9417d1a7488ef6350393c8c839fc8";

        #[test]
        fn normal_input() {
            assert!(verify_hash(
                String::from(GOOD_KEY),
                String::from(GOOD_VALUE),
                String::from(GOOD_HASH)
            ))
        }

        #[test]
        fn bad_key() {
            assert!(!verify_hash(
                String::new(),
                String::from(GOOD_VALUE),
                String::from(GOOD_HASH)
            ))
        }

        #[test]
        fn bad_value() {
            assert!(!verify_hash(
                String::from(GOOD_KEY),
                String::new(),
                String::from(GOOD_HASH)
            ))
        }

        #[test]
        fn bad_hash() {
            assert!(!verify_hash(
                String::from(GOOD_KEY),
                String::from(GOOD_VALUE),
                String::new()
            ))
        }
    }
    mod decode_hex {
        use super::super::*;

        #[test]
        fn normal_input() {
            assert_eq!(Ok(vec![255, 10, 0, 35]), decode_hex("ff0a0023"));
        }

        #[test]
        fn empty_input() {
            assert_eq!(Ok(vec![]), decode_hex(""));
        }

        #[test]
        fn invalid_hex_pair() {
            assert!(decode_hex("fg").is_err());
        }

        #[test]
        #[should_panic]
        fn str_too_short() {
            let _result = decode_hex("f");
        }

        #[test]
        #[should_panic]
        fn odd_len() {
            let _result = decode_hex("fff");
        }

    }
}
