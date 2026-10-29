# Graphite verification map

This directory is the maintained source for verifying caller-visible Graphite behavior. Read the index before driving a feature, then use the matching feature file as the recipe.

Graphite is a Rust library. A caller supplies text metrics, colors, and rectangles. The observable result is the value the public function returns.

## Baseline preconditions

- The checkout is the Graphite repository root, with `Cargo.toml` naming `graphite-core` and pinning `rust-version = "1.85"`.
- `rustc` and `cargo` are 1.85 or newer.
- `.cursor/skills/verify-graphite/scripts/doctor.sh` has printed `package=graphite-core` for this checkout.
- No verification drive is recorded in the state directory's `driver.pid`. That is `target/verify-graphite/driver.pid` unless `CARGO_TARGET_DIR` is set, in which case it is `$CARGO_TARGET_DIR/verify-graphite/driver.pid`.
- Do not drive a `cargo` process that this run did not start.

## Driving conventions

- Start every recipe by running `doctor.sh` when this checkout has not been checked since the last failed drive.
- Run `.cursor/skills/verify-graphite/scripts/drive.sh <feature>` and read `target/verify-graphite/<feature>/drive.txt`. When `CARGO_TARGET_DIR` is set, read `$CARGO_TARGET_DIR/verify-graphite/<feature>/drive.txt` instead, and pass that same variable to `cleanup.sh`.
- Treat every command and expected line as literal.
- One feature per drive. Do not treat a different feature's evidence as proof for this one.

## Proof and skip reporting

- Capture the driver stdout, which includes the inputs echoed by that feature and the returned values.
- Exit code 0 is required and is not sufficient. The evidence file must contain the lines named in the feature.
- Record the feature id in the evidence path: `target/verify-graphite/<feature>/drive.txt`.
- Report an unreachable feature with the command that was run and the unmet precondition (`doctor.sh` failure, live pid file, or missing toolchain).
- Do not report a skipped feature as verified through another feature.

## Feature entry contract

Each feature file starts with an H1 and one paragraph, then exactly four H2 sections: `Sub-features`, `How to get to it (user POV)`, `Driving it with verify-graphite`, and `Gotchas`.

## Features

- [Ellipsize](./ellipsize.md) truncates a label that does not fit and leaves a label that does fit unchanged.
- [Theme tokens](./theme-tokens.md) returns the dark and light color sets and their contrast.
- [Per-surface text](./per-surface-text.md) resolves one preferred ink against conflicting actual fills.
- [Per-surface text](./per-surface-text.md) resolves one preferred ink against conflicting actual fills.
- [Pane status](./pane-status.md) chooses a pane-header status and measures its label.
- [Legend keys](./legend-keys.md) places a legend run that fits and drops one that does not.
- [Tabs bar](./tabs-bar.md) lays out the tabs bar and hit-tests the space menu and the first tab.
- [Pixel buffer](./pixel-buffer.md) draws and clips RGBA rectangles, blends alpha, and exposes row-major frame bytes.
