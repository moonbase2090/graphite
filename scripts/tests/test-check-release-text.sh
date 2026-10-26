#!/usr/bin/env bash
# Regression cases for scripts/check-release-text.sh. Each case runs the
# checker in a throwaway repository with HOME isolated from the developer's
# denylist file, and checks the exit code.
set -eu
script="$(cd "$(dirname "$0")/../.." && pwd)/scripts/check-release-text.sh"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

repo="$work/repo"
mkdir -p "$repo" "$work/home"
cd "$repo"
git init -q
git config user.email test@example.invalid
git config user.name test
mkdir scripts
cp "$script" scripts/check-release-text.sh
printf '# Notes\n\nClean release text.\n' > README.md
git add -A
git commit -qm fixture

failures=0
expect() {
  name="$1"; want="$2"; shift 2
  set +e
  env HOME="$work/home" "$@" >"$work/out" 2>&1
  got=$?
  set -e
  if [ "$got" -ne "$want" ]; then
    echo "FAIL $name: exit $got, want $want"; cat "$work/out"; failures=$((failures + 1))
  else
    echo "ok   $name: exit $got"
  fi
}

expect "clean tree passes" 0 bash scripts/check-release-text.sh

printf 'built from /Users/someone/src\n' > "$work/leaky.bin"
expect "single rejected artifact fails" 1 bash scripts/check-release-text.sh "$work/leaky.bin"

mkdir "$work/a" "$work/b"
printf 'built from /Users/someone/src\n' > "$work/a/app"
printf 'clean build output\n' > "$work/b/app"
expect "rejected artifact followed by clean same-basename artifact fails" 1 \
  bash scripts/check-release-text.sh "$work/a/app" "$work/b/app"

expect "trailing blank denylist line keeps a clean tree passing" 0 \
  env RELEASE_DENYLIST="$(printf 'zzz-not-present\n\n')" bash scripts/check-release-text.sh

expect "trailing blank denylist line still rejects a match" 1 \
  env RELEASE_DENYLIST="$(printf 'release-only-secret\n\n')" \
  sh -c 'printf "release-only-secret\n" > README.md && git add -A && git commit -qm leak && bash scripts/check-release-text.sh'
git checkout -q HEAD~1 -- README.md 2>/dev/null || true
git reset -q --hard HEAD~1

expect "malformed denylist expression is a configuration error" 2 \
  env RELEASE_DENYLIST='[' bash scripts/check-release-text.sh

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed"; exit 1
fi
echo "all release-text cases passed"
