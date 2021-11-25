use std::env::set_var;
use std::path::PathBuf;

use compiletest_rs as ct;

fn run_test(mode: &str, dir: &str, configure: impl FnOnce(&mut ct::Config)) {
    let mut config = ct::Config::default();
    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = PathBuf::from(format!("tests/{}", dir));
    configure(&mut config);
    config.link_deps();
    config.clean_rmeta();

    ct::run_tests(&config);
}

#[test]
fn smoke() {
    set_var("EXPLICIT_OVERRIDE_ISIZE", "123");
    set_var("EXPLICIT_OVERRIDE_F64", "123.0");
    set_var("IMPLICIT_ISIZE", "123");
    set_var("IMPLICIT_F64", "123.0");
    set_var("NEGATIVE_I16", "-123");
    set_var("NEGATIVE_ISIZE", "-321");
    set_var("NEGATIVE_F32", "-456f32");
    set_var("SMOKE_STR", "bar");
    set_var("SMOKE_U32", "321");
    run_test("run-pass", "run-pass/untracked", |_| {});
}

#[cfg(feature = "tracked")]
#[test]
fn tracked() {
    set_var("TRACKED_STR", "tracked");
    set_var("TRACKED_U32", "4321");
    run_test("run-pass", "run-pass/tracked", |config| {
        config.target_rustcflags = Some("--cfg feature=\"tracked\"".to_string())
    });
}