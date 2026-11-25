use std::{env, fs, path::PathBuf, process::ExitCode};

fn target() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../web/tokens/graphite-tokens.css")
}

fn package_target() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../web/ui/tokens.css")
}

fn write_css() -> ExitCode {
    let css = graphite_css::stylesheet();
    let first = fs::write(target(), &css);
    let second = fs::write(package_target(), css);
    if let Err(error) = first.and(second) {
        eprintln!("Could not write token sheets: {error}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn check_css() -> ExitCode {
    let expected = graphite_css::stylesheet();
    for path in [target(), package_target()] {
        match fs::read_to_string(path) {
            Ok(existing) if existing == expected => {}
            Ok(_) => {
                eprintln!("Token sheet is stale; run with --write");
                return ExitCode::FAILURE;
            }
            Err(error) => {
                eprintln!("Could not read token sheet: {error}");
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    match env::args().nth(1).as_deref() {
        Some("--write") => write_css(),
        Some("--check") => check_css(),
        None => {
            print!("{}", graphite_css::stylesheet());
            ExitCode::SUCCESS
        }
        Some(_) => {
            eprintln!("use --write, --check, or no arguments");
            ExitCode::FAILURE
        }
    }
}
