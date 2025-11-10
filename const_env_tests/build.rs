fn main() {
    set_env("INT_ARRAY", "[10, 11, 12]");
    set_env("STRING_ARRAY", r#"["bar"]"#);
    set_env("TUPLE_ARRAY", r#"[("goodbye", false), ("planet", true)]"#);
    set_env("EXPLICIT_OVERRIDE_ISIZE", "123");
    set_env("EXPLICIT_OVERRIDE_F64", "123.0");
    set_env("IMPLICIT_ISIZE", "234");
    set_env("IMPLICIT_F64", "234.0");
    set_env("NEGATIVE_I16", "-123");
    set_env("NEGATIVE_ISIZE", "-321");
    set_env("NEGATIVE_F32", "-456.0");
    set_env("QUOTED_STR", "hello world");
    set_env("QUOTED_BYTE_STR", "01abcS");
    set_env("SMOKE_STR", "bar");
    set_env("SMOKE_U32", "321");
    set_env("ORIGIN", "Vec2 { x: 1., y: 2.}");
}

fn set_env(name: &str, value: &str) {
    println!("cargo:rustc-env={name}={value}");
}