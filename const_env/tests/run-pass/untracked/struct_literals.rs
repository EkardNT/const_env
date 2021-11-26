extern crate const_env;

use const_env::env_lit;

#[derive(Eq, PartialEq, Debug)]
struct Vec2<T> {
    x: T,
    y: T
}

static ORIGIN: Vec2<f32> = env_lit!("ORIGIN", Vec2 { x: 0., y: 0.});

fn main() {
    assert_eq!(Vec2 { x: 1., y: 2.}, ORIGIN);
}