# Releasing

Releases and tags need owner approval (see `REVIEW_POLICY.md`).

## Release text

Release text and artifacts must not reference private infrastructure,
hostnames, personal paths, or internal tooling. This covers the changelog,
release notes, README and docs, built artifacts, the release title and body,
and the tag message.

Run `scripts/check-release-text.sh` before publishing. It exits non-zero on
any match and prints only the file and line of a hit, never the matched text.
Pass built artifacts as arguments to scan them too, and set `RELEASE_TITLE`,
`RELEASE_BODY`, and `RELEASE_TAG_MESSAGE` to check release metadata.
