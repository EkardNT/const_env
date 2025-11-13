# const_env

- [Motivation](#motivation)
- [Crate Features](#crate-features)
- [Usage](#usage)
- [Supported Types](#supported-types)
- [Limitations](#known-limitations)
- [Alternatives](#alternatives)

## Motivation

**Your goal**: You want to define various constants for your code at compile time based on environment variables.

**Your problem**: Its only possible to do this for `&'static str` constants in today's Rust.

```rust
const MY_STR: &'static str = env!("SOME_ENV");
```

You cannot similarly initialize other types such as constant `u32`s, `bool`s, or
other primitive types. See [issues](https://github.com/rust-lang/rfcs/issues/1907) and
[workarounds](https://stackoverflow.com/questions/37526598/overriding-constant-via-compiler-option).
Eventually you will be able to do so once support for running `parse` and `unwrap` in
`const fn` lands, but for now this crate offers an easy workaround that you can use today.

## Crate Features

| Feature name | Enabled by default? | Requires nightly? | Description |
|---|---|---|---|
| `tracked` | No | Yes | Use the unstable [proc_macro_tracked_env](https://github.com/rust-lang/rust/issues/99515) feature to inform the build system about the used environment variables. |

## Usage

Add the dependency. If your crate uses nightly, enable the `tracked` feature for better
build dependency tracking.

```toml
# If using a stable compiler:
[dependencies]
const_env = "0.1"

# If using a nightly compiler:
[dependencies]
const_env = { version = "0.1", features = ["tracked"] }
```

At the top of your file import the `env_item!` and/or `env_lit!` macros.

```rust
use const_env::{env_item, env_lit};
```

Use the macro to override the value of constants based on environment variables at build time.

```rust
#[env_item]
const FOO: u32 = 123;

const BAR: bool = env_lit!("BAR", false);

// This test will PASS if invoked as `FOO=456 BAR=true cargo test`
#[test]
fn test() {
    assert_eq!(456, FOO);
    assert_eq!(true, BAR);
}
```

By default, the `env_item` macro looks for an environment variable with the same name as the constant that it is attached to.

```rust
// Use `FOO=true cargo build` to configure the value.
#[env_item]
const FOO: bool = false;
```

But you can specify a name explicitly too.

```rust
// Use `BAR=true cargo build` to configure the value.
#[env_item("BAR")]
const FOO: bool = false;
```

The expression that you assign in your source acts as a default in case the environment variable does not exist.

```rust
// If env var FOO is not set then the FOO constant will have the default value of '🦀'.
#[env_item]
const FOO: char = '🦀';
```

Both `const` and `static` declarations are supported.

```rust
// Both of these may be set by `FOO=abc BAZ=def cargo build`.
#[env_item]
const FOO: &'static str = "hello";
#[env_item("BAZ")]
static BAR: &'static [u8] = b"world";
```

## Supported Types

Strings!

```rust
#[env_item]
const FOO: &'static str = "hello";

// example: `FOO=abc cargo build`
// results in:
const FOO: &'static str = "abc";
```

Byte strings!

```rust
#[env_item]
const FOO: &'static [u8] = b"hello";

// example: `FOO=world cargo build`
// results in:
const FOO: &'static [u8] = b"world";
```

Bytes!
```rust
#[env_item]
const FOO: u8 = b'⚙';

// example: `FOO=🦀 cargo build`
// results in:
const FOO: u8 = b'🦀';
```

Characters!

```rust
#[env_item]
const FOO: char = '⚙';

// example: `FOO=🦀 cargo build`
// results in:
const FOO: car = '🦀';
```

Integers of all shapes and sizes!

```rust
#[env_item]
const FOO: u32 = 123;
#[env_item]
const BAR: i64 = 456;
#[env_item]
const BAZ: usize = 0;

// example: `FOO=321 BAR=-456 BAZ=1usize cargo build`
// results in:
const FOO: u32 = 321;
const BAR: i64 = -456;
const BAZ: usize = 1usize;
```

Floats of all shapes and sizes!

```rust
#[env_item]
const FOO: f32 = 123.0;
#[env_item]
const BAR: f64 = 456.0;
#[env_item]
const BAZ: f32 = 0.0;

// example: `FOO=321.0 BAR=-456.0 BAZ=1f32 cargo build`
// results in:
const FOO: f32 = 321.0;
const BAR: f64 = -456.0;
const BAZ: f32 = 1f32;
```

Booleans!

```rust
#[env_item]
const FOO: bool = false;

// example: `FOO=true cargo build`
// results in:
const FOO: bool = true;
```

## Known Limitations

- Only top-level `const` and `static` declarations are supported.

## Alternatives

An immediately available alternative would be to write a `build.rs` script which looks at the env vars and generates code based on them. This is conceptually similar to how this crate works, except that this crate uses a procedural macro instead of a build script.

Longer term, this crate will hopefully become completely superceded by being able to write normal Rust code once compile-time const evaluation of [FromStr](https://github.com/rust-lang/rust/issues/143773) and [various Option methods](https://github.com/rust-lang/rust/issues/143956). That will allow you to modify your code like so:

```rust
// Current code using this library.
#[env_item]
const FOO: u32 = 0;

// New code, no library needed.
const FOO: u32 = option_env!("FOO").and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
```

Until that is possible, I'm not aware of an alternative approach.