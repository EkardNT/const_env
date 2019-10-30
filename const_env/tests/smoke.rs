use const_env::from_env;

// This needs to be run like so:
// MY_STR=bar MY_U32=321 cargo test

// TODO: use compiletest-rs

#[from_env("MY_STR")]
const MY_STR: &'static str = "foo";

#[from_env("MY_U32")]
const MY_U32: u32 = 123;

#[test]
fn test_str() {
    assert_eq!("bar", MY_STR);
    assert_eq!(321, MY_U32);
}