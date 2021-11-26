extern crate const_env;

use const_env::env_lit;

const USIZE_ARRAY: [usize; 3] = env_lit!("INT_ARRAY", [1, 2, 3]);

const STRING_ARRAY: [&'static str; 1] = env_lit!("STRING_ARRAY", ["foo"]);

const TUPLE_ARRAY: [(&'static str, bool); 2] = env_lit!("TUPLE_ARRAY", [("hello", true), ("world", false)]);

fn main() {
    assert_eq!([10, 11, 12], USIZE_ARRAY);
    assert_eq!(["bar"], STRING_ARRAY);
    assert_eq!([("goodbye", false), ("planet", true)], TUPLE_ARRAY);
}