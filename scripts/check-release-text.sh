#!/bin/bash
# Fail if public release text or artifacts reveal private infrastructure.
# Usage: scripts/check-release-text.sh [ARTIFACT...]
# Also scans $RELEASE_TITLE, $RELEASE_BODY and $RELEASE_TAG_MESSAGE if set.
# Extra patterns (one ERE per line) come from the optional file
# ~/.config/moonbase/release-denylist.txt and the optional RELEASE_DENYLIST
# environment variable. Missing sources are skipped. Matched text is never
# printed, only file:line.
set -eu
cd "$(git rev-parse --show-toplevel)"

patterns=(
  'DESKTOP-[A-Z0-9]+'
  '\.local\b'
  '/Users/'
  '/home/[a-z]'
  'C:\\Users'
  '-----BEGIN [A-Z ]*PRIVATE KEY-----'
  '\b(ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{20,}'
  'github_pat_[A-Za-z0-9_]{20,}'
  '\b10\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\b'
  '\b192\.168\.[0-9]{1,3}\.[0-9]{1,3}\b'
  '\b172\.(1[6-9]|2[0-9]|3[01])\.[0-9]{1,3}\.[0-9]{1,3}\b'
)
email_re='[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}'
allowed_email='322824348+mb2090@users.noreply.github.com'

add_patterns() {
  while IFS= read -r line; do
    if [ -n "$line" ]; then
      patterns+=("$line")
    fi
  done
  return 0
}
# Reject malformed expressions up front: grep exits 2 on an invalid pattern.
validate_patterns() {
  for p in "${patterns[@]}"; do
    rc=0
    grep -E -e "$p" /dev/null >/dev/null 2>&1 || rc=$?
    if [ "$rc" -eq 2 ]; then
      echo "release-text: invalid denylist expression (file or RELEASE_DENYLIST)" >&2
      exit 2
    fi
  done
}
denylist_file="${HOME:-}/.config/moonbase/release-denylist.txt"
[ -f "$denylist_file" ] && add_patterns < "$denylist_file"
[ -n "${RELEASE_DENYLIST:-}" ] && add_patterns <<< "$RELEASE_DENYLIST"
validate_patterns

scratch=$(mktemp -d)
index=0
trap 'rm -rf "$scratch"' EXIT

targets=()
while IFS= read -r f; do
  case "$f" in
    CHANGELOG*|RELEASE*|README*|docs/*|*.md) targets+=("$f") ;;
  esac
done < <(git ls-files)
for a in "$@"; do
  [ -f "$a" ] || { echo "missing artifact: $a" >&2; exit 2; }
  # Binaries and archives: scan printable strings.
  # One scratch file per input, so inputs with the same basename never overwrite each other.
  index=$((index + 1))
  scan="$scratch/artifact-$index.strings"
  strings -a "$a" > "$scan"
  targets+=("$scan")
done
for v in RELEASE_TITLE RELEASE_BODY RELEASE_TAG_MESSAGE; do
  if [ -n "${!v:-}" ]; then
    printf '%s\n' "${!v}" > "$scratch/$v"
    targets+=("$scratch/$v")
  fi
done

status=0
for f in "${targets[@]}"; do
  [ -f "$f" ] || continue
  for p in "${patterns[@]}"; do
    if hits=$(grep -nE -e "$p" "$f" 2>/dev/null | cut -d: -f1); [ -n "$hits" ]; then
      for n in $hits; do echo "${f#"$scratch"/}:$n: matches a release-text rule"; done
      status=1
    fi
  done
  while IFS= read -r hit; do
    n=${hit%%:*}; text=${hit#*:}
    for e in $(grep -oE -e "$email_re" <<< "$text"); do
      if [ "$e" != "$allowed_email" ]; then
        echo "${f#"$scratch"/}:$n: disallowed email address"; status=1
      fi
    done
  done < <(grep -nE -e "$email_re" "$f" 2>/dev/null || true)
done
exit $status
