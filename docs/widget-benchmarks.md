# Measure chrome rendering

Run an optimized correctness pass, then the five-round gate:

```sh
cargo bench --locked -p graphite-tui --bench chrome -- --test
python3 scripts/bench-widgets.py
```

Each scene uses the reviewed widget snapshot fixture. Before timing, the
harness checks the exact text snapshot, groups adjacent equal cell styles into
spans, prepares a plain ratatui Paragraph, and verifies that it produces exactly
the same symbols and styles as the Graphite widget. Both sides reuse allocated
buffers and reset them inside the timed region. Layout, string construction,
color resolution and baseline span grouping stay outside timing. The control
is borrowed during rendering, so it does not clone its text each frame.

The control measures a frozen styled Paragraph containing the same rendered
cells. It is a fixed ratatui reference, not a claim about every possible
handwritten rendering implementation. Text correctness is independently checked
against the committed snapshots; baseline styles are copied during setup.

There is one pair per widget and a six-widget frame pair. The frame is the
library caller's buffer-render path. Terminal encoding, damage tracking, I/O,
font rendering and event latency are outside this crate and are not measured.
Those consumer-level guardrails belong in the consumer benchmarks.

The runner alternates the first side across five processes, retains each
Criterion mean and confidence interval, and prints the median and range of the
five per-process means. A pair passes only when the largest Graphite mean is at
most 1.10 times the smallest control mean. A pair fails when the smallest
Graphite mean exceeds 1.10 times the largest control mean. Overlapping outcomes
are inconclusive and exit 2, so a noisy run cannot silently pass. Failures exit
1. Both widgets and the combined frame must pass.

Logs, per-round estimates and `summary.tsv` remain in `target/widget-proof`.
Use `--out` to keep another run. Record compiler, machine load, run count and
ranges with any result. Profile a separate run before attributing a difference
to a particular code path. The CI job repeats the correctness pass and gate on
Rust 1.85.0. Its timing result is specific to that runner.

To prove the gate rejects real extra rendering work, run:

```sh
python3 scripts/bench-widgets.py --group frame --negative-control --out target/widget-negative
```

This repeats Graphite drawing 16 times per iteration while the control still
renders once. It must exit 1 with FAIL. Do not report those injected timings as
performance results. The ordinary runner explicitly disables the injection.
