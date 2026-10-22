## Development setup

This repository ships a pre-commit hook in `.githooks/` that blocks commits
containing unformatted staged files. Enable it once per clone:

```sh
git config core.hooksPath .githooks
```

(`scripts/setup-hooks.sh` does the same.) The hook only runs when relevant
files are staged. If it fails, run `cargo fmt --all`, stage the result, and commit again.
