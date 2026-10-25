# Shared by drive.sh and cleanup.sh. Source this file; do not execute it.
# state_dir REPO_ROOT prints the directory that holds driver.pid and evidence.
# When CARGO_TARGET_DIR is set, that directory is the parent, so a private
# target isolates cargo artifacts, the lock, and the evidence together.
# A relative CARGO_TARGET_DIR is resolved from the repository root.
state_dir() {
  if [ -n "${CARGO_TARGET_DIR:-}" ]; then
    case "$CARGO_TARGET_DIR" in
      /*) printf '%s\n' "$CARGO_TARGET_DIR/verify-graphite" ;;
      *) printf '%s\n' "$1/$CARGO_TARGET_DIR/verify-graphite" ;;
    esac
  else
    printf '%s\n' "$1/target/verify-graphite"
  fi
}
