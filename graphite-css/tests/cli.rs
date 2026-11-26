use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_graphite-css"))
        .args(args)
        .output()
        .expect("graphite-css runs")
}

fn committed_sheet() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../web/tokens/graphite-tokens.css");
    fs::read_to_string(path).expect("committed token sheet")
}

#[test]
fn no_arguments_prints_the_committed_sheet() {
    let out = run(&[]);
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), committed_sheet());
    assert!(out.stderr.is_empty());
}

#[test]
fn check_accepts_the_committed_sheets() {
    let out = run(&["--check"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty());
    assert!(out.stderr.is_empty());
}

#[test]
fn unknown_argument_is_refused_with_usage() {
    let out = run(&["--writ"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "use --write, --check, or no arguments\n"
    );
}
