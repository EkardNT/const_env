extern crate const_env;

use const_env::from_env;

#[from_env("TRACKED_STR")]
const TRACKED_STR: &'static str = "foo";

#[from_env("TRACKED_U32")]
const TRACKED_U32: u32 = 123;

fn main() {
    assert_eq!("tracked", TRACKED_STR);
    assert_eq!(4321, TRACKED_U32);
}