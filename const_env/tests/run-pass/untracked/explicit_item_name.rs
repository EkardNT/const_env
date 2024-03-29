extern crate const_env;

use const_env::env_item;

#[env_item("EXPLICIT_OVERRIDE_ISIZE")]
const EXPLICIT_ISIZE: isize = 0;

#[env_item("EXPLICIT_OVERRIDE_F64")]
const EXPLICIT_F64: f64 = 0.0;

fn main() {
    assert_eq!(123, EXPLICIT_ISIZE);
    assert_eq!(123.0, EXPLICIT_F64);
}