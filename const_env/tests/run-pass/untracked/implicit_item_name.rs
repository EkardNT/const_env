extern crate const_env;

use const_env::env_item;

#[env_item]
const IMPLICIT_ISIZE: isize = 0;

#[env_item]
const IMPLICIT_F64: f64 = 0.0;

fn main() {
    assert_eq!(123, IMPLICIT_ISIZE);
    assert_eq!(123.0, IMPLICIT_F64);
}