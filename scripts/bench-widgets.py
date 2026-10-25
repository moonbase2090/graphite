#!/usr/bin/env python3
"""Run five alternating Criterion rounds and apply a conservative 10% gate."""
import argparse
import json
import os
from pathlib import Path
import statistics
import subprocess
import sys

GROUPS = ["tabs", "side_rail", "sidebar", "pane", "legend", "picker", "frame"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, default=Path("target/widget-proof"))
    parser.add_argument("--group", choices=GROUPS, help="measure one pair, for example frame")
    parser.add_argument("--negative-control", action="store_true", help="repeat Graphite rendering 16 times to prove the gate rejects real extra work")
    args = parser.parse_args()
    groups = [args.group] if args.group else GROUPS
    args.out.mkdir(parents=True, exist_ok=True)
    root = Path(__file__).resolve().parent.parent
    values = {name: {side: [] for side in ["graphite", "plain"]} for name in groups}
    for run in range(5):
        env = os.environ.copy()
        env["GRAPHITE_BENCH_ORDER"] = "graphite-first" if run % 2 == 0 else "plain-first"
        env["GRAPHITE_BENCH_NEGATIVE_CONTROL"] = "1" if args.negative_control else "0"
        log = args.out / f"round-{run + 1}.log"
        with log.open("w") as stream:
            subprocess.run(["cargo", "bench", "--locked", "-p", "graphite-tui", "--bench", "chrome", "--", *([args.group] if args.group else [])], cwd=root, env=env, stdout=stream, stderr=subprocess.STDOUT, check=True)
        estimates = {}
        target = Path(env.get("CARGO_TARGET_DIR", root / "target"))
        if not target.is_absolute():
            target = root / target
        for name in groups:
            estimates[name] = {}
            for side in ["graphite", "plain"]:
                data = json.loads((target / "criterion" / name / side / "new/estimates.json").read_text())
                value = data["mean"]["point_estimate"]
                values[name][side].append(value)
                estimates[name][side] = data
        (args.out / f"round-{run + 1}.json").write_text(json.dumps(estimates, indent=2))
        print(f"round {run + 1}/5 completed ({env['GRAPHITE_BENCH_ORDER']})", flush=True)
    report = ["widget\tgraphite_median_ns\tgraphite_range_ns\tplain_median_ns\tplain_range_ns\tratio\tverdict"]
    verdicts = []
    for name in groups:
        g, p = values[name]["graphite"], values[name]["plain"]
        gm, pm = statistics.median(g), statistics.median(p)
        verdict = "PASS" if max(g) <= 1.1 * min(p) else "FAIL" if min(g) > 1.1 * max(p) else "INCONCLUSIVE"
        verdicts.append(verdict)
        report.append(f"{name}\t{gm:.1f}\t{min(g):.1f}..{max(g):.1f}\t{pm:.1f}\t{min(p):.1f}..{max(p):.1f}\t{gm/pm:.3f}\t{verdict}")
    text = "\n".join(report) + "\n"
    (args.out / "summary.tsv").write_text(text)
    print(text, end="")
    return 1 if "FAIL" in verdicts else 2 if "INCONCLUSIVE" in verdicts else 0


if __name__ == "__main__":
    sys.exit(main())
