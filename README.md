# const_env

**Your goal**: You want to define various constants for your code at compile time based on environment variables.

**Your problem**: Its only possible to do this for `&'static str` constants in today's Rust.

```rust
const MY_STR: &'static str = env!("SOME_ENV");
```

You cannot similarly initialize other types such as constant `u32`s, `bool`s, or
other primitive types. See [issues](https://github.com/rust-lang/rfcs/issues/1907) and
[workarounds](https://stackoverflow.com/questions/37526598/overriding-constant-via-compiler-option).
Eventually you will be able to do so once support for running `parse` and `unwrap` in as
`const fn` lands, but for now this crate offers an easy workaround that you can use today.

## Usage

Add the dependency.

```toml
[dependencies]
const_env = "0.1"
```

At the top of your file import the `from_env!` macro.

```rust
use const_env::from_env;
```

Use the macro to override the value of constants based on environment variables at build time.

```rust
#[from_env]
const FOO: u32 = 123;

// This test will PASS if invoked as `FOO=456 cargo test`
#[test]
fn test() {
    assert_eq!(456, FOO);
}
```

By default, the macro looks for an environment variable with the same name as the constant that it is attached to.

```rust
// Use `FOO=true cargo build` to configure the value.
#[from_env]
const FOO: bool = false;
```

But you can specify a name explicitly too.

```rust
// Use `BAR=true cargo build` to configure the value.
#[from_env("BAR")]
const FOO: bool = false;
```

The expression that you assign in your source acts as a default in case the environment variable does not exist.

```rust
// If env var FOO is not set then the FOO constant will have the default value of '🦀'.
#[from_env]
const FOO: char = '🦀';
```