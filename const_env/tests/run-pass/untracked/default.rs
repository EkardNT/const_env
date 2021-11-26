extern crate const_env;

use const_env::{env_item, env_lit};

#[env_item]
const DEFAULT_CHAR: char = 'A';

const DEFAULT_CHAR_LIT: char = env_lit!("DEFAULT_CHAR", 'A');

#[env_item("DEFAULT_BYTE_STR")]
const DEFAULT_BYTE_STR: &'static [u8] = b"abcdefg";

const DEFAULT_BYTE_STR_LIT: &'static [u8] = env_lit!("DEFAULT_BYTE_STR", b"abcdefg");

fn main() {
    assert_eq!('A', DEFAULT_CHAR);
    assert_eq!('A', DEFAULT_CHAR_LIT);
    assert_eq!(b"abcdefg", DEFAULT_BYTE_STR);
    assert_eq!(b"abcdefg", DEFAULT_BYTE_STR_LIT);
}