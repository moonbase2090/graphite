#!/bin/sh
# Drive one Graphite feature through the public library API.
# usage: drive.sh <ellipsize|theme-tokens|per-surface-text|pane-status|legend-keys|tabs-bar|pixel-buffer>
set -eu
set -m
feature=${1:?usage: drive.sh <feature>}
root=$(CDPATH= cd -- "$(dirname "$0")/../../../.." && pwd)
cd "$root"
script_dir=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
. "$script_dir/state.sh"
case "$feature" in
  ellipsize|theme-tokens|per-surface-text|pane-status|legend-keys|tabs-bar|pixel-buffer) ;;
  *)
    printf 'drive: unknown feature %s\n' "$feature" >&2
    exit 2
    ;;
esac
state=$(state_dir "$root")
pidfile=$state/driver.pid
evidence=$state/$feature/drive.txt
if [ -f "$pidfile" ]; then
  old=$(cat "$pidfile" 2>/dev/null || true)
  if [ -n "$old" ] && kill -0 "$old" 2>/dev/null; then
    printf 'drive: another verification drive is still running as pid %s\n' "$old" >&2
    exit 1
  fi
fi
mkdir -p "$state/$feature"

cargo_pid=
cleaned=0
on_exit() {
  if [ "$cleaned" -eq 1 ]; then
    return 0
  fi
  cleaned=1
  if [ -n "$cargo_pid" ] && kill -0 "$cargo_pid" 2>/dev/null; then
    kill -TERM -"$cargo_pid" 2>/dev/null || kill -TERM "$cargo_pid" 2>/dev/null || true
    wait "$cargo_pid" 2>/dev/null || true
  fi
  rm -f "$pidfile"
}
trap 'on_exit' EXIT
trap 'on_exit; exit 130' INT
trap 'on_exit; exit 143' TERM

cargo run --quiet --manifest-path "$root/.cursor/skills/verify-graphite/driver/Cargo.toml" -- "$feature" \
  > "$evidence" &
cargo_pid=$!
printf '%s\n' "$cargo_pid" > "$pidfile"
status=0
wait "$cargo_pid" || status=$?
cargo_pid=
cat "$evidence"
exit "$status"
