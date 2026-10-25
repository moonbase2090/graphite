---
name: verify-graphite
description: "Drive graphite-core, the Graphite layout and color library, through its public Rust API and capture stdout proof. Use when proving ellipsize, theme tokens, pane status, legend packing, or tabs-bar layout."
---

# Verify Graphite

Graphite is a library. Callers pass measurements and rectangles in and read layout, color, and truncated text back. There is no window, server, or account. Verification builds `graphite-core` and runs one feature driver in its own `cargo` process.

## Launch

From the repository root:

```sh
.cursor/skills/verify-graphite/scripts/doctor.sh
.cursor/skills/verify-graphite/scripts/drive.sh <feature>
```

`<feature>` is `ellipsize`, `theme-tokens`, `pane-status`, `legend-keys`, or `tabs-bar`.

Ready means `drive.sh` exits 0 and writes the evidence file. There is no port and no ready log line. The first run compiles the library; later runs reuse the cargo target directory.

The state directory holds `driver.pid` and `<feature>/drive.txt`. With `CARGO_TARGET_DIR` unset it is `target/verify-graphite/` in the repository. When `CARGO_TARGET_DIR` is set, cargo artifacts and the state directory both live under that path: `$CARGO_TARGET_DIR/verify-graphite/`. A relative value is resolved from the repository root. Cleanup must be started with the same `CARGO_TARGET_DIR` or it will not see that drive.

Do not start a second drive in the same state directory while `driver.pid` names a live process. `drive.sh` refuses that itself. A side-by-side run sets a different `CARGO_TARGET_DIR` for each drive.

Teardown is `.cursor/skills/verify-graphite/scripts/cleanup.sh`. It signals the process group of the cargo process recorded in `driver.pid`, which includes the compiler or driver cargo started. It does not signal a process by name. It does not delete proof files.

## Workspace CI verification

After changing build or CI configuration, run the same checks for each workspace
feature configuration with Rust 1.85.0:

```sh
cargo fmt --all -- --check
for feature_args in '' '--features motion' '--no-default-features'; do
    cargo clippy --workspace --locked --all-targets $feature_args -- -D warnings
    cargo test --workspace --locked $feature_args
    RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked $feature_args
    cargo check --workspace --all-targets --locked $feature_args
done
```

Run this block in a POSIX shell so each feature argument expands into separate
words. CI repeats these checks on Linux and macOS. The default jobs keep the
existing `Checks (ubuntu-latest)` and `Checks (macos-latest)` names for branch
protection. Feature jobs include the feature configuration in their names.
Record the PR's green CI run link as cross-platform proof.

## Doctor

Run `.cursor/skills/verify-graphite/scripts/doctor.sh` before the first drive, and again after a drive fails.

It is read-only. Success prints one line:

```text
doctor: ok rustc=rustc <version> package=graphite-core rust-version=1.85
```

Failure means `rustc` is older than 1.85, `Cargo.toml` does not pin `rust-version = "1.85"`, or `cargo metadata --no-deps` does not report the package `graphite-core`. Do not drive that checkout.

## Drive

The harness is `.cursor/skills/verify-graphite/scripts/drive.sh`. It runs the public API in `.cursor/skills/verify-graphite/driver` and copies stdout to the evidence file, then prints that file.

The driver supplies the text metrics a host would supply:

- `ellipsize` and `legend-keys` measure one cell per Unicode scalar.
- `pane-status` and `tabs-bar` measure cells times the font size in pixels.

Use the feature file for the exact arguments and the stdout lines that prove the feature. A passing exit code alone is not proof.

## Evidence

Each successful drive leaves `<state>/<feature>/drive.txt`. The default state directory is `target/verify-graphite/`, and `target/` is gitignored. Cleanup must leave these files in place.

Proof standards:

- Call the public functions named in the feature file. Do not satisfy the check by editing a unit test or by printing a constant the driver did not compute.
- The evidence file contains both the action inputs (feature name, and the labels or sizes on the stdout lines) and the resulting values.
- Compare the file with the lines in the feature file. A mismatch is a failed drive.
- Graphite does not write a database or send messages. The side effect is the evidence file itself. After cleanup, the file must still be present and non-empty.

## Cleanup

```sh
.cursor/skills/verify-graphite/scripts/cleanup.sh
```

That reads `driver.pid` in the state directory, signals that process group, and removes the pid file. The recorded pid is cargo, started in its own process group, not the shell that launched it. Signaling the group stops cargo and the compiler or driver still running under it. Cleanup does not remove `<state>/<feature>/drive.txt`. It does not kill processes by name.

## Helpers

- `.cursor/skills/verify-graphite/scripts/doctor.sh` — read-only checkout check. No arguments.
- `.cursor/skills/verify-graphite/scripts/drive.sh <feature>` — build if needed, run one feature, write and print `<state>/<feature>/drive.txt`.
- `.cursor/skills/verify-graphite/scripts/cleanup.sh` — stop the recorded cargo process group and keep evidence. Uses the same `CARGO_TARGET_DIR` rules as `drive.sh`.
- `.cursor/skills/verify-graphite/scripts/state.sh` — sourced by `drive.sh` and `cleanup.sh` to resolve the state directory. Do not run it directly.

`doctor.sh`, `drive.sh`, and `cleanup.sh` are executable. Run them from any working directory; they locate the repository from their own path. `state.sh` is sourced and is not a command.

Keep this skill honest with `/maintain-verification-skill` when a change alters how Graphite is built, measured, or proven.

## Terminal palettes

With the opt-in `terminal-palette` feature, run:

```sh
cargo test --locked --features terminal-palette
cargo run --quiet --locked --example terminal_palettes --features terminal-palette > target/terminal-palettes.md
diff -u docs/terminal-palettes.md target/terminal-palettes.md
```

The generated table must match the reviewed reference exactly. Spot-check fixed
xterm endpoints with the palette unit tests. Slots 0 through 15 depend on the
host palette and are excluded from nearest-color search; ANSI-16 uses semantic
slots. Dark has 34 entries and light has 35 because its optional active-tab
outline is present. Run the same commands from a clean local VM for behavior
proof, and record the exact commit and output in the PR.

## Release metadata checks

Run `python3 -m unittest discover -s scripts/tests -p test_release.py` and
`python3 scripts/check-release.py --dry-run`. The fixture tests must show five
passes, including rejecting a mismatched tag, absent release notes, and mixed
workspace versions. The repository dry-run must list workspace package versions
and state that a release entry and signed tag are still required. Before an
approved release, run `python3 scripts/check-release.py --tag vX.Y.Z` against the
prepared release commit. It must reject tags whose version differs from the
workspace or whose changelog section is absent. These commands perform no
release writes. The tag workflow repeats the check and workspace tests/docs.

## Release text check

`scripts/check-release-text.sh` fails when release text or artifacts reveal
private infrastructure. Run it with bash from the repository root, optionally
passing built artifacts as arguments. Exit codes: 0 clean, 1 a rule matched
(output names file and line, never the matched text), 2 a denylist expression
is malformed or an artifact is missing.

Run the regression cases with `bash scripts/tests/test-check-release-text.sh`.
They must print `all release-text cases passed`. The cases cover a clean tree,
a rejected artifact alone, a rejected artifact followed by a clean artifact with
the same basename, a trailing blank denylist line with and without a match, and
a malformed expression.

## Interaction states

Run `cargo test --locked --features states` and
`cargo run --quiet --locked --example state_styles --features states`.
The example prints one line per depth and state. Expected modifiers:
`Focus` is `thick=true marker=true bold=true` at every depth, with
`reverse=false` at TrueColor and Ansi256 and `reverse=true` at Ansi16.
`Selected` is `thick=false marker=false bold=false`, with `reverse=true` only
at Ansi16. `SelectedFocus` adds `marker=true`. `Normal`, `Hover`, `Disabled`,
and `Error` print all four flags `false` at every depth. Run it from a clean
local VM and record the exact commit and stdout. The `states` feature is off
by default.

## TOML theme files

Run `cargo test --locked --features serde` and
`cargo run --quiet --locked --example theme_toml --features serde`.
The example must print `dark: 35 lines, round trip equal`,
`light: 36 lines, round trip equal`, and
``bad hex: `ground` must be a color like "#1A2B3C", found "#10121"``.
Run it from a clean local VM and record the exact commit and stdout. The
`serde` feature is off by default and pulls in the `toml` crate.

## Widget gallery

The `gallery` example in `graphite-tui` renders every chrome widget (tabs bar,
side rail, sidebar, pane chrome, legend, theme picker) as ANSI text. It has six
modes: `dark` or `light`, each at `truecolor`, `256`, or `16` color depth.

Commands, from the repository root, with Rust 1.85 on `PATH`:

```sh
cargo test --locked -p graphite-tui --example gallery
cargo run --quiet --locked -p graphite-tui --example gallery -- dark 16
cargo run --quiet --locked -p graphite-tui --example gallery -- all
```

Expected output:

- `gallery all` prints six `== <theme> <depth>` headings in this order: `dark truecolor`, `dark 256`, `dark 16`, `light truecolor`, `light 256`, `light 16`. It exits 0.
- Each mode prints the same plain-text tabs-bar line after the escape codes are removed: `  Graphite ▾     │ ● build 2  ×   ● review  needs you  ×   ● notes ×  +`.
- The `gallery` example tests pass: `chip_active` maps to ANSI background 40 and `tab_hover` to 100 in the dark 16-color mode, and truecolor and 256 keep the real values.

Screenshot procedure for review: run the `all` command in a clean local
virtual macOS machine, then capture the host's view of the terminal window.
Record the exact commit, the command, and the image with the review.

## Host-theme contrast properties

Run `cargo test --locked --test theme_properties` and
`cargo run --quiet --locked --example theme_contrast`. The focused regression
must pass, and the example must show attention `[221, 51, 81]` with contrast
at least 4.5. Before the fix, its ink `[19, 19, 19]` had contrast 4.129008.
The property test samples 512 coherent host themes: background channels are
0..60 for dark or 190..255 for light, text has the opposite polarity, and badge
and ANSI colors span the full byte range. It checks every documented text
surface and the accent floor. Existing CI runs this integration test through
`cargo test`.

Do not describe this as a guarantee for independently arbitrary surfaces. A
shared ink cannot meet 4.5 on black, white and mid-gray simultaneously. The
solver preserves best-effort behavior for incompatible backgrounds; the
property generator states its admissible domain instead of discarding failures.
Repeat the public example and property tests from a clean local VM and retain
the exact head and output as behavior proof.
