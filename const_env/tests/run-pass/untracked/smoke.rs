extern crate const_env;

use const_env::{env_item, env_lit};

#[env_item("SMOKE_STR")]
const SMOKE_STR: &'static str = "foo";

const SMOKE_STR_LIT: &'static str = env_lit!("SMOKE_STR", "foo");

#[env_item("SMOKE_U32")]
const SMOKE_U32: u32 = 123;

const SMOKE_U32_LIT: u32 = env_lit!("SMOKE_U32", 123);

fn main() {
    assert_eq!("bar", SMOKE_STR);
    assert_eq!("bar", SMOKE_STR_LIT);
    assert_eq!(321, SMOKE_U32);
    assert_eq!(321, SMOKE_U32_LIT);
}