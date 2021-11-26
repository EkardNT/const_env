extern crate const_env;

use const_env::{env_item, env_lit};

#[env_item]
const NEGATIVE_I16: i16 = 0;

const NEGATIVE_I16_LIT: i16 = env_lit!("NEGATIVE_I16", 0);

#[env_item]
const NEGATIVE_ISIZE: isize = -1;

const NEGATIVE_ISIZE_LIT: isize = env_lit!("NEGATIVE_ISIZE", -1);

#[env_item]
const NEGATIVE_F32: f32 = 0.0;

const NEGATIVE_F32_LIT: f32 = env_lit!("NEGATIVE_F32", 0.0);

fn main() {
    assert_eq!(-123, NEGATIVE_I16);
    assert_eq!(-123, NEGATIVE_I16_LIT);
    assert_eq!(-321, NEGATIVE_ISIZE);
    assert_eq!(-321, NEGATIVE_ISIZE_LIT);
    assert_eq!(-456.0, NEGATIVE_F32);
    assert_eq!(-456.0, NEGATIVE_F32_LIT);
}