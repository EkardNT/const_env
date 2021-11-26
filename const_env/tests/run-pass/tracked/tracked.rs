extern crate const_env;

use const_env::{env_item, env_lit};

#[env_item("TRACKED_STR")]
const TRACKED_STR: &'static str = "foo";

const TRACKED_STR_LIT: &'static str = env_lit!("TRACKED_STR", "foo");

#[env_item("TRACKED_U32")]
const TRACKED_U32: u32 = 123;

const TRACKED_U32_LIT: u32 = env_lit!("TRACKED_U32", 123);

fn main() {
    assert_eq!("tracked", TRACKED_STR);
    assert_eq!("tracked", TRACKED_STR_LIT);
    assert_eq!(4321, TRACKED_U32);
    assert_eq!(4321, TRACKED_U32_LIT);
}