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
