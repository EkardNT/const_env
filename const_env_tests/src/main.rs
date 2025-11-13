use const_env::{env_item, env_lit};

const USIZE_ARRAY: [usize; 3] = env_lit!("INT_ARRAY", [1, 2, 3]);

const STRING_ARRAY: [&'static str; 1] = env_lit!("STRING_ARRAY", ["foo"]);

const TUPLE_ARRAY: [(&'static str, bool); 2] = env_lit!("TUPLE_ARRAY", [("hello", true), ("world", false)]);

#[env_item]
const DEFAULT_CHAR: char = 'A';

const DEFAULT_CHAR_LIT: char = env_lit!("DEFAULT_CHAR", 'A');

#[env_item("DEFAULT_BYTE_STR")]
const DEFAULT_BYTE_STR: &'static [u8] = b"abcdefg";

const DEFAULT_BYTE_STR_LIT: &'static [u8] = env_lit!("DEFAULT_BYTE_STR", b"abcdefg");

#[env_item("EXPLICIT_OVERRIDE_ISIZE")]
const EXPLICIT_ISIZE: isize = 0;

#[env_item("EXPLICIT_OVERRIDE_F64")]
const EXPLICIT_F64: f64 = 0.0;

#[env_item]
const IMPLICIT_ISIZE: isize = 0;

#[env_item]
const IMPLICIT_F64: f64 = 0.0;

#[env_item]
const NEGATIVE_I16: i16 = 0;

const NEGATIVE_I16_LIT: i16 = env_lit!("NEGATIVE_I16", 0);

#[env_item]
const NEGATIVE_ISIZE: isize = -1;

const NEGATIVE_ISIZE_LIT: isize = env_lit!("NEGATIVE_ISIZE", -1);

#[env_item]
const NEGATIVE_F32: f32 = 0.0;

const NEGATIVE_F32_LIT: f32 = env_lit!("NEGATIVE_F32", 0.0);

const STR: &'static str = env_lit!("QUOTED_STR", "foo");

const BYTE_STR: &'static [u8] = env_lit!("QUOTED_BYTE_STR", b"123");

#[env_item("SMOKE_STR")]
const SMOKE_STR: &'static str = "foo";

const SMOKE_STR_LIT: &'static str = env_lit!("SMOKE_STR", "foo");

#[env_item("SMOKE_U32")]
const SMOKE_U32: u32 = 123;

const SMOKE_U32_LIT: u32 = env_lit!("SMOKE_U32", 123);

#[derive(Eq, PartialEq, Debug)]
struct Vec2<T> {
    x: T,
    y: T
}

#[env_item]
static ORIGIN: Vec2<f64> = Vec2 { x: 0., y: 0. };

static ORIGIN_LIT: Vec2<f32> = env_lit!("ORIGIN", Vec2 { x: 0., y: 0.});

fn main() {
    assert_eq!([10, 11, 12], USIZE_ARRAY);
    assert_eq!(["bar"], STRING_ARRAY);
    assert_eq!([("goodbye", false), ("planet", true)], TUPLE_ARRAY);
    assert_eq!('A', DEFAULT_CHAR);
    assert_eq!('A', DEFAULT_CHAR_LIT);
    assert_eq!(b"abcdefg", DEFAULT_BYTE_STR);
    assert_eq!(b"abcdefg", DEFAULT_BYTE_STR_LIT);
    assert_eq!(123, EXPLICIT_ISIZE);
    assert_eq!(123.0, EXPLICIT_F64);
    assert_eq!(234, IMPLICIT_ISIZE);
    assert_eq!(234.0, IMPLICIT_F64);
    assert_eq!(-123, NEGATIVE_I16);
    assert_eq!(-123, NEGATIVE_I16_LIT);
    assert_eq!(-321, NEGATIVE_ISIZE);
    assert_eq!(-321, NEGATIVE_ISIZE_LIT);
    assert_eq!(-456.0, NEGATIVE_F32);
    assert_eq!(-456.0, NEGATIVE_F32_LIT);
    assert_eq!("hello world", STR);
    assert_eq!(b"01abcS", BYTE_STR);
    assert_eq!("bar", SMOKE_STR);
    assert_eq!("bar", SMOKE_STR_LIT);
    assert_eq!(321, SMOKE_U32);
    assert_eq!(321, SMOKE_U32_LIT);
    assert_eq!(Vec2 { x: 1., y: 2.}, ORIGIN);
    assert_eq!(Vec2 { x: 1., y: 2.}, ORIGIN_LIT);

    println!("Tests succeeded!");
}