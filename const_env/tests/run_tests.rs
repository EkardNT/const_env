use std::env::set_var;
use std::path::PathBuf;

use compiletest_rs as ct;

fn run_test(mode: &str) {
    let mut config = ct::Config::default();
    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    // config.target_rustcflags = Some("-L target/debug -L target/debug/deps".to_string());
    config.link_deps();
    config.clean_rmeta();

    ct::run_tests(&config);
}

#[test]
fn smoke() {
    set_var("SMOKE_STR", "bar");
    set_var("SMOKE_U32", "321");
    run_test("run-pass");
}