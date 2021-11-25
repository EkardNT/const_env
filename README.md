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
| `tracked` | No | yes | Use the unstable [proc_macro_tracked_env](https://github.com/rust-lang/rust/issues/74690) feature to inform the build system about the used environment variables. |

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

Both `const` and `static` declarations are supported.

```rust
// Both of these may be set by `FOO=abc BAZ=def cargo build`.
#[from_env]
const FOO: &'static str = "hello";
#[from_env("BAZ")]
static BAR: &'static [u8] = b"world";
```

## Supported Types

Strings!

```rust
#[from_env]
const FOO: &'static str = "hello";

// example: `FOO=abc cargo build`
// results in:
const FOO: &'static str = "abc";
```

Byte strings!

```rust
#[from_env]
const FOO: &'static [u8] = b"hello";

// example: `FOO=world cargo build`
// results in:
const FOO: &'static [u8] = b"world";
```

Bytes!
```rust
#[from_env]
const FOO: u8 = b'⚙';

// example: `FOO=🦀 cargo build`
// results in:
const FOO: u8 = b'🦀';
```

Characters!

```rust
#[from_env]
const FOO: char = '⚙';

// example: `FOO=🦀 cargo build`
// results in:
const FOO: car = '🦀';
```

Integers of all shapes and sizes!

```rust
#[from_env]
const FOO: u32 = 123;
#[from_env]
const BAR: i64 = 456;
#[from_env]
const BAZ: usize = 0;

// example: `FOO=321 BAR=-456 BAZ=1usize cargo build`
// results in:
const FOO: u32 = 321;
const BAR: i64 = -456;
const BAZ: usize = 1usize;
```

Floats of all shapes and sizes!

```rust
#[from_env]
const FOO: f32 = 123.0;
#[from_env]
const BAR: f64 = 456.0;
#[from_env]
const BAZ: f32 = 0.0;

// example: `FOO=321.0 BAR=-456.0 BAZ=1f32 cargo build`
// results in:
const FOO: f32 = 321.0;
const BAR: f64 = -456.0;
const BAZ: f32 = 1f32;
```

Booleans!

```rust
#[from_env]
const FOO: bool = false;

// example: `FOO=true cargo build`
// results in:
const FOO: bool = true;
```

## Known Limitations

- Only top-level `const` and `static` declarations are supported.

## Alternatives

- Writing a `build.rs` script which looks at the env vars and generates code based on them. This is conceptually similar to how this crate works, except that this crate uses a procedural macro instead of a build script.
- Wait for [const fn](https://github.com/rust-lang/rust/issues/57563) to be finished. Ultimated
this crate does the equivalent of `static FOO: u32 = std::option_env!("FOO").map(|val| val.parse::<u32>().unwrap_or(0)).unwrap_or(0);`,
so once you can simply write that code this crate will become superfluous.