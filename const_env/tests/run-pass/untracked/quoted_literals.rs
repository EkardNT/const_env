extern crate const_env;

use const_env::env_lit;

const STR: &'static str = env_lit!("QUOTED_STR", "foo");
const BYTE_STR: &'static [u8] = env_lit!("QUOTED_BYTE_STR", b"123");
// const CHAR: char = env_lit!("QUOTED_CHAR", 'A');
// const BYTE: u8 = env_lit!("QUOTED_BYTE", b'A');

fn main() {
    assert_eq!("hello world", STR);
    assert_eq!(b"01abcS", BYTE_STR);
    // assert_eq!('\t', CHAR);
    // assert_eq!(b'\n', BYTE);
}