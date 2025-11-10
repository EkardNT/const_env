use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote_spanned};
use syn::{Expr, ExprLit, Lit};
use syn::spanned::Spanned;

pub trait ReadEnv {
    fn read_env(&self, var_name: &String) -> Option<String>;
}

pub struct TestEnv {
    env_vars: HashMap<String, String>
}

impl TestEnv {
    pub fn builder() -> TestEnvBuilder {
        TestEnvBuilder {
            env_vars: HashMap::new()
        }
    }
}

impl ReadEnv for TestEnv {
    fn read_env(&self, var_name: &String) -> Option<String> {
        self.env_vars.get(var_name).cloned()
    }
}

pub struct TestEnvBuilder {
    env_vars: HashMap<String, String>
}

impl TestEnvBuilder {
    pub fn set(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.insert(name.into(), value.into());
        self
    }

    pub fn build(self) -> TestEnv {
        TestEnv {
            env_vars: self.env_vars
        }
    }
}

struct MacroInput {
    env_var_name: syn::LitStr,
    default_value: syn::Expr,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = syn::punctuated::Punctuated::<syn::Expr, syn::token::Comma>::parse_terminated(input)?;
        if args.len() != 2 {
            return Err(syn::Error::new(input.span(), "Exactly 2 arguments expected"));
        }
        let env_var_name = match args.first().unwrap() {
            Expr::Lit(ExprLit { lit: syn::Lit::Str(lit_str), .. }) => {
                lit_str.clone()
            },
            otherwise => return Err(syn::Error::new(otherwise.span(), "Expected first argument to be a string literal"))
        };
        let default_value = args.last().unwrap().clone();
        Ok(Self {
            env_var_name,
            default_value
        })
    }
}

/// Include environment variable contents as a Rust literal.
pub fn env_lit(tokens: TokenStream, read_env: impl ReadEnv) -> TokenStream {
    let input: MacroInput = match syn::parse2(tokens) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error()
    };
    let env_var_value = match read_env.read_env(&input.env_var_name.value()) {
        Some(env_var_value) => env_var_value,
        None => return input.default_value.into_token_stream()
    };
    let env_var_value_tokens = match env_var_value.parse::<TokenStream>() {
        Ok(tokens) => tokens,
        Err(err) => return syn::Error::new(input.env_var_name.span(), format!("{}", err)).to_compile_error()
    };
    // Special case logic for quoted literals such as strings. We want to allow users not
    // to need to quote their environment variable values for strings, even though this is
    // technically inconsistent with other literals. So here we handle string-like literals
    // specially by auto-adding quotes. Note that we only do this for top-level string literals -
    // other instances of literal strings, such as elements of an array, require the user to
    // manually quote them.
    match input.default_value {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(_), ..}) => {
            let quoted = format!("\"{}\"", env_var_value);
            match syn::parse_str::<syn::LitStr>(&quoted) {
                Ok(literal) => literal.to_token_stream(),
                Err(err) => syn::Error::new(input.env_var_name.span(), format!("Invalid string literal contents: {}", err)).to_compile_error()
            }
        }
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::ByteStr(_), ..}) => {
            let quoted = format!("b\"{}\"", env_var_value);
            match syn::parse_str::<syn::LitByteStr>(&quoted) {
                Ok(literal) => literal.to_token_stream(),
                Err(err) => syn::Error::new(input.env_var_name.span(), format!("Invalid byte string literal contents: {}", err)).to_compile_error()
            }
        }
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Char(_), ..}) => {
            let quoted = format!("'{}'", env_var_value);
            match syn::parse_str::<syn::LitChar>(&quoted) {
                Ok(literal) => literal.to_token_stream(),
                Err(err) => syn::Error::new(input.env_var_name.span(), format!("Invalid char literal contents: {}", err)).to_compile_error()
            }
        }
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Byte(_), ..}) => {
            let quoted = format!("b'{}'", env_var_value);
            match syn::parse_str::<syn::LitByte>(&quoted) {
                Ok(literal) => literal.to_token_stream(),
                Err(err) => syn::Error::new(input.env_var_name.span(), format!("Invalid byte literal contents: {}", err)).to_compile_error()
            }
        }
        _ => env_var_value_tokens
    }
}

/// Inner implementation details of `const_env::from_env`.
pub fn from_env(attr: TokenStream, item: TokenStream, read_env: impl ReadEnv) -> TokenStream {
    match try_from_env(attr, item, read_env) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error()
    }
}

fn try_from_env(attr: TokenStream, item: TokenStream, read_env: impl ReadEnv) -> Result<TokenStream, syn::Error> {
    if let Ok(mut item_const) = syn::parse2::<syn::ItemConst>(item.clone()) {
        let default_var_name = format!("{}", item_const.ident);
        let var_name = extract_var_name(attr, default_var_name)?;
        let var_value = match read_env.read_env(&var_name) {
            Some(val) => val,
            None => return Ok(item)
        };
        let new_expr = value_to_literal(&var_value, &item_const.expr)?;
        let span = item_const.span();
        item_const.expr = Box::new(new_expr);
        Ok(quote_spanned!(span => #item_const))
    } else if let Ok(mut item_static) = syn::parse2::<syn::ItemStatic>(item.clone()) {
        let default_var_name = format!("{}", item_static.ident);
        let var_name = extract_var_name(attr, default_var_name)?;
        let var_value = match read_env.read_env(&var_name) {
            Some(val) => val,
            None => return Ok(item)
        };
        let new_expr = value_to_literal(&var_value, &item_static.expr)?;
        let span = item_static.span();
        item_static.expr = Box::new(new_expr);
        Ok(quote_spanned!(span => #item_static))
    } else {
        Err(syn::Error::new(attr.span(), "Macro is only valid on const or static items"))
    }
}

fn extract_var_name(attr: TokenStream, default: String) -> Result<String, syn::Error> {
    if attr.is_empty() {
        return Ok(default);
    }
    let span = attr.span();
    let expr: Expr = syn::parse2(attr)
        .map_err(|_| syn::Error::new(span,"Unable to parse attribute args as expression"))?;
    extract_var_name_from_expr(&expr)
}

fn extract_var_name_from_expr(expr: &Expr) -> Result<String, syn::Error> {
    match expr {
        Expr::Lit(literal) => {
            match &literal.lit {
                Lit::Str(lit_str) => {
                    Ok(lit_str.value())
                },
                _ => Err(syn::Error::new_spanned(expr, "Attribute arguments are not a valid string literal"))
            }
        },
        Expr::Paren(paren) => {
            extract_var_name_from_expr(&paren.expr)
        },
        _ => {
            Err(syn::Error::new_spanned(expr, "Attribute arguments are not a valid string literal expression"))
        }
    }
}

fn value_to_literal(value: &str, original_expr: &Expr) -> Result<Expr, syn::Error> {
    Ok(match original_expr {
        Expr::Array(array) => {
            syn::Expr::Array(syn::parse_str::<syn::ExprArray>(value)
                .map_err(|_| syn::Error::new_spanned(array, "Failed to parse environment variable contents as valid array"))?)
        },
        Expr::Unary(unary) => {
            // A unary sign indicates this is a numeric literal which doesn't need any
            // escaping, so we can parse it directly.
            let new: Expr = syn::parse_str(value)
                .map_err(|_| syn::Error::new_spanned(unary, "Failed to parse environment variable contents as valid expression"))?;
            return Ok(new);
        },
        Expr::Lit(literal) => {
            let new_lit = match &literal.lit {
                Lit::Str(original) => {
                    let mut new: syn::LitStr = syn::parse_str(&format!("\"{}\"", value))
                        .map_err(|_| syn::Error::new_spanned(original, "Failed to parse environment variable contents as literal string"))?;
                    new.set_span(original.span());
                    Lit::Str(new)
                },
                Lit::ByteStr(original) => {
                    let mut new: syn::LitByteStr = syn::parse_str(&format!("b\"{}\"", value))
                        .map_err(|_| syn::Error::new_spanned(original, "Failed to parse environment variable contents as literal byte string"))?;
                    new.set_span(original.span());
                    Lit::ByteStr(new)
                },
                Lit::Byte(original) => {
                    let mut new: syn::LitByte = syn::parse_str(&format!("b'{}'", value))
                        .map_err(|_| syn::Error::new_spanned(original, "Failed to parse environment variable contents as literal byte"))?;
                    new.set_span(original.span());
                    Lit::Byte(new)
                },
                Lit::Char(original) => {
                    let mut new: syn::LitChar = syn::parse_str(&format!("'{}'", value))
                        .map_err(|_| syn::Error::new_spanned(original, "Failed to parse environment variable contents as literal character"))?;
                    new.set_span(original.span());
                    Lit::Char(new)
                },
                // These variants do not need any escaping and can be parsed as an expression
                // directly.
                Lit::Bool(_) | Lit::Int(_) | Lit::Float(_) | Lit::Verbatim(_) => {
                    let new: Expr = syn::parse_str(value)
                        .map_err(|_| syn::Error::new_spanned(original_expr, "Failed to parse environment variable contents as valid expression"))?;
                    return Ok(new);
                },
                unhandled => {
                    return Err(syn::Error::new_spanned(unhandled, "Unsupported literal type"));
                }
            };
            ExprLit {
                attrs: literal.attrs.clone(),
                lit: new_lit
            }.into()
        },
        expr => {
            return Err(syn::Error::new_spanned(expr, "Original const expression was not a recognized literal expression"));
        }
    })
}