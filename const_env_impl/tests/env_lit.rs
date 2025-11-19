use const_env_impl::{env_lit, TestEnv};

use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn test_str() {
    let env = TestEnv::builder()
        .set("MYVAR", "world")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", "default"
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            "world"
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_str_default() {
    let env = TestEnv::builder()
        .build();
    let input: TokenStream = quote! {
        "MYVAR", "default"
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            "default"
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_int() {
    let env = TestEnv::builder()
        .set("MYVAR", "1")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            1
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_negative_int() {
    let env = TestEnv::builder()
        .set("MYVAR", "-1")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            -1
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_u32_with_suffix() {
    let env = TestEnv::builder()
        .set("MYVAR", "1u32")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            1u32
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_i64_with_suffix() {
    let env = TestEnv::builder()
        .set("MYVAR", "1i64")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            1i64
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_char() {
    let env = TestEnv::builder()
        .set("MYVAR", "b")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 'a'
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            'b'
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_float() {
    let env = TestEnv::builder()
        .set("MYVAR", "1.0")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0.0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            1.0
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_negative_float() {
    let env = TestEnv::builder()
        .set("MYVAR", "-1.0")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0.0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            -1.0
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_float_with_suffix() {
    let env = TestEnv::builder()
        .set("MYVAR", "1f32")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0.0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            1f32
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_bool() {
    let env = TestEnv::builder()
        .set("MYVAR", "true")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", false
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            true
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}

#[test]
fn test_i32_negative_whitespace() {
    let env = TestEnv::builder()
        .set("MYVAR", " - 123 ")
        .build();
    let input: TokenStream = quote! {
        "MYVAR", 0
    };
    let expected: TokenStream = quote! {
        {
            let _ = option_env!("MYVAR");
            -123
        }
    };
    let result = env_lit(input, env);
    assert_eq!(format!("{}", expected), format!("{}", result));
}