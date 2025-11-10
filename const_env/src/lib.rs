#![cfg_attr(
    feature = "tracked",
    feature(proc_macro_tracked_env)
)]

extern crate proc_macro;

use proc_macro::TokenStream;

/// Configure a `const` or `static` item from an environment variable.
/// 
/// # Usage
/// This macro takes one optional string literal parameter, which if present is used as the name of the environment
/// variable that the literal value will be parsed from. If no parameter is present, then the name of the Rust
/// static/const item decorated by this attribute will be used as the environment variable name.
/// 
/// The static or const item being decorated should be assigned a value, which will function as the default value if
/// no such matching environment variable is defined.
/// 
/// # Examples
/// 
/// ```rust
/// // In this example, the environment variable FOO will be used 
/// // to define the Rust f64 FOO literal. If no FOO environment
/// // variable is defined, then the Rust literal will have the
/// // value 0.0.
/// #[env_item]
/// static FOO: f64 = 0.0;
/// ```
/// 
/// ```rust
/// // In this example, the environment variable BAR will be used 
/// // to define the Rust f64 FOO literal. If no BAR environment
/// // variable is defined, then the Rust literal will have the
/// // value 0.0
/// #[env_item("BAR")]
/// static FOO: f64 = 0.0;
/// ```
#[proc_macro_attribute]
pub fn env_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(not(feature = "tracked"))]
    let read_env = StableEnv {};
    #[cfg(feature = "tracked")]
    let read_env = TrackedEnv {};
    const_env_impl::env_item(attr.into(), item.into(), read_env).into()
}

/// Deprecated alias of env_item, use env_item instead. Will be deleted upon the next major
/// version bump.
#[deprecated = "Deprecated alias of env_item, use env_item instead"]
#[proc_macro_attribute]
pub fn from_env(attr: TokenStream, item: TokenStream) -> TokenStream {
    env_item(attr, item)
}

/// Insert a Rust literal value from an environment variable.
/// 
/// # Usage
/// This macro requires two arguments:
/// - The first argument must be a string literal, which will be used as the name of the environment
///   variable whose value will be parsed as the literal value of this macro.
/// - The second argument is any expression, which will be used as the default value of this macro
///   if there is no environment variable defined.
/// 
/// Unlike in the [env_item] macro, there is no relationship between the environment variable name passed
/// to this macro and the name of the static or const item the macro value is assigned to. They can be the
/// same or different. Only the string name passed to this macro is used to look up an environment variable.
/// 
/// # Examples
/// 
/// ```rust
/// // In this example, the HELLO_WORLD environment variable's value will be used as the value of the FOO
/// // Rust constant. If no such HELLO_WORLD variable is defined, then the value 0 will be used as the default
/// // value of the constant.
/// const FOO: u8 = env_lit!("HELLO_WORLD", 0);
/// ```
#[proc_macro]
pub fn env_lit(tokens: TokenStream) -> TokenStream {
    #[cfg(not(feature = "tracked"))]
    let read_env = StableEnv {};
    #[cfg(feature = "tracked")]
    let read_env = TrackedEnv {};
    const_env_impl::env_lit(tokens.into(), read_env).into()
}

#[cfg(feature = "tracked")]
struct TrackedEnv;

#[cfg(feature = "tracked")]
impl const_env_impl::ReadEnv for TrackedEnv {
    fn read_env(&self, var_name: &String) -> Option<String> {
        proc_macro::tracked_env::var(var_name).ok()
    }
}

struct StableEnv;

impl const_env_impl::ReadEnv for StableEnv {
    fn read_env(&self, var_name: &String) -> Option<String> {
        std::env::var(var_name).ok()
    }
}