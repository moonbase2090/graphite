# Review policy

This policy applies to every pull request, whether a person or an agent wrote it.

## 1. Classify every PR

**Trunk**: the change touches shared code that other parts depend on:
- core libraries and shared modules
- auth, secrets, and permissions
- data models, schemas, and migrations
- CI, release, and signing workflows; build configuration
- public APIs, CLI flags, config formats
- anything with 5 or more dependents

**Leaf**: everything else, such as a single UI screen, docs, a self-contained script, or test-only changes.

A PR that is partly trunk is trunk. When unsure, call it trunk. Label trunk PRs `trunk` and leaf PRs `leaf`.

**In graphite, trunk includes:**
- `src/lib.rs`: the public API, including every `pub use`. A public signature or output change changes Prismattyc at its next pin bump.
- Shared foundations used across modules: `src/geom.rs` (`Rect`), `src/text.rs` (`Face`, `TextMetrics`), `src/tokens.rs` (`Rgb`, `ThemeVariant`, `Tokens`), `src/scale.rs` (design constants and `scale_px`), `src/color.rs` and `src/derive.rs` (theme token derivation).
- Feature modules whose public output Prismattyc draws: `src/pane.rs`, `src/ring.rs`, `src/sidebar.rs`, `src/tabs_bar.rs`, `src/side_rail.rs`, `src/picker.rs`, `src/legend.rs` and `src/translucency.rs`, whenever a `pub` signature, returned geometry, hit result or color changes.
- CI and build configuration: `.github/workflows/ci.yml`, `Cargo.toml` including the pinned `rust-version = "1.85"`, and `Cargo.lock`.

Leaf changes include test-only changes, docs, and a private helper inside one feature module that changes no public output and is covered by that module's tests.

## 2. Proof (every PR)

The PR body has a **Proof** section with real evidence: test output, a CI run link, a screenshot or recording for UI changes, or a before/after for behavior changes. "Tested locally" without output is not proof.

## 3. Leaf PRs

- CI green and proof present.
- One review by anyone other than the author (person or agent).
- Merge once green.

## 4. Trunk PRs

- CI green, and the proof shows the change running, not just compiling.
- Agentic validation: an agent other than the author builds and exercises the change.
- Independent review, preferably by a different model than the author.
- Every finding is fixed or explicitly waived in the PR thread.
- Then merge.

## 5. Feature gating

Graphite is pure layout and color math. New behavior in trunk code ships behind an option that is off by default, so pinned consumers see no change until they opt in, unless it is a pure fix. Turning an option on by default is its own trunk PR, and that PR is trunk.

## 6. Tests

For every test, ask: would it fail if the behavior it names broke? If not, reject it. Reject tests that:

- assert a value against itself, or a constant against the same constant
- compute the expected value with the code under test or a copy of its logic
- mock or stub the unit under test, then assert what the mock returns
- only check that something ran, was called, or didn't panic, with no assertion on the result
- compare against a snapshot or golden file regenerated from current output without review
- still pass when the implementation is deleted or replaced with a stub or default

## 7. Merging

- Merge commits only. No admin overrides.
- Code-scanning threads are resolved only when they are report-only or addressed, never just to unblock a merge.
- Releases and tags need owner approval.
- Bumping Prismattyc's `graphite-core` pin is a separate trunk PR in Prismattyc.
