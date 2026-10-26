# Prepare a release

Graphite packages use one workspace version. While the version is 0.x, increment
the minor version for incompatible public API changes and the patch version for
compatible fixes. At 1.x and later, incompatible public API changes increment
the major version. New compatible APIs increment the minor version. Tags have
the form `vX.Y.Z`, optionally followed by a SemVer prerelease suffix.

Only the release PR changes package versions. Update every workspace package,
regenerate Cargo.lock, and move the relevant Unreleased notes into a section
such as `## [0.1.0] - YYYY-MM-DD`. Keep an Unreleased section for future changes.
Write release notes about behavior consumers can observe.

Before preparing the release PR, run:

```sh
python3 scripts/check-release.py --dry-run
```

Before tagging the merged release commit, run the metadata check with its
intended version, followed by the workspace checks in the verification skill:

```sh
python3 scripts/check-release.py --tag v0.1.0
```

A matching version is insufficient without a nonempty changelog section for
that release. The tag workflow repeats this check, runs all-feature workspace
tests on Rust 1.85.0, and builds documentation with warnings denied. It also
checks default-off compilation.

Moonbase approves, signs, pushes the annotated tag, and creates the release.
This workflow validates pushed tags; it does not create tags, publish crates,
or create releases. A green run does not itself authorize a release. Packages
remain unpublished while their manifests say `publish = false`.
