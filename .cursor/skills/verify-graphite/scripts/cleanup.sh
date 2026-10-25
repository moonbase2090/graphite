#!/bin/sh
# Stop a verification drive this run started. Evidence files stay in the state directory.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/../../../.." && pwd)
cd "$root"
script_dir=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
. "$script_dir/state.sh"
state=$(state_dir "$root")
pidfile=$state/driver.pid
if [ ! -f "$pidfile" ]; then
  printf 'cleanup: no driver pid file\n'
  exit 0
fi
pid=$(cat "$pidfile" 2>/dev/null || true)
case "$pid" in
  ''|*[!0-9]*)
    printf 'cleanup: pid file does not hold a pid\n' >&2
    exit 1
    ;;
esac
if kill -0 "$pid" 2>/dev/null; then
  pgid=$(ps -o pgid= -p "$pid" 2>/dev/null | tr -d ' ' || true)
  if [ "$pgid" = "$pid" ]; then
    kill -TERM -"$pid" 2>/dev/null || true
  else
    kill -TERM "$pid" 2>/dev/null || true
  fi
  i=0
  while [ "$i" -lt 50 ] && kill -0 "$pid" 2>/dev/null; do
    i=$((i + 1))
    sleep 0.1
  done
  if kill -0 "$pid" 2>/dev/null; then
    if [ "$pgid" = "$pid" ]; then
      kill -KILL -"$pid" 2>/dev/null || true
    else
      kill -KILL "$pid" 2>/dev/null || true
    fi
  fi
fi
rm -f "$pidfile"
printf 'cleanup: removed driver pid file; evidence under %s was left in place\n' "$state"
