extern crate proc_macro;

use const_env_impl::RealEnv;
use proc_macro::TokenStream;

/// A procedural macro!
#[proc_macro_attribute]
pub fn from_env(attr: TokenStream, item: TokenStream) -> TokenStream {
    const_env_impl::from_env(attr.into(), item.into(), RealEnv {}).into()
}


