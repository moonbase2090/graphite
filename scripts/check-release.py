#!/usr/bin/env python3
"""Validate release metadata without changing files, tags, or releases."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import sys

VERSION = r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*)?"


def check(root, tag):
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--manifest-path", str(root / "Cargo.toml")], text=True
    ))
    members = set(metadata["workspace_members"])
    packages = [p for p in metadata["packages"] if p["id"] in members]
    versions = {p["version"] for p in packages}
    if len(versions) != 1:
        raise ValueError("workspace package versions differ")
    version = versions.pop()
    if not re.fullmatch(VERSION, version):
        raise ValueError("workspace version is not a supported semantic version")
    if tag is not None and tag != f"v{version}":
        raise ValueError(f"tag {tag!r} does not match workspace version v{version}")
    changelog = (root / "CHANGELOG.md").read_text()
    heading = version if tag else "Unreleased"
    section = re.search(r"^## \[" + re.escape(heading) + r"\][^\n]*\n(.*?)(?=^## |\Z)", changelog, re.M | re.S)
    if not section or not re.search(r"^\s*- \S", section[1], re.M):
        raise ValueError(f"CHANGELOG.md needs a nonempty [{heading}] section")
    for package in sorted(packages, key=lambda p: p["name"]):
        print(f"package={package['name']} version={version}")
    print(f"release-check: {'tag metadata ready' if tag else 'dry-run passed; release entry and signed tag still required'}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parent.parent)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--tag", help="tag being checked, for example v0.1.0")
    group.add_argument("--dry-run", action="store_true", help="check workspace versions and Unreleased notes")
    args = parser.parse_args()
    try:
        check(args.root, args.tag)
    except (ValueError, OSError, subprocess.CalledProcessError, json.JSONDecodeError) as error:
        print(f"release-check: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
