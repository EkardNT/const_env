#![cfg_attr(
    feature = "tracked",
    feature(proc_macro_tracked_env)
)]

extern crate proc_macro;

use proc_macro::TokenStream;

/// Configure a `const` or `static` item from an environment variable.
#[proc_macro_attribute]
pub fn env_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(not(feature = "tracked"))]
    let read_env = StableEnv {};
    #[cfg(feature = "tracked")]
    let read_env = TrackedEnv {};
    const_env_impl::from_env(attr.into(), item.into(), read_env).into()
}

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