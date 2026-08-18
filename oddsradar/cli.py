from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from oddsradar.mapping import compare
from oddsradar.secrets import forbidden_fields


def cmd_doctor(args) -> int:
    cfg = json.loads(Path(args.config).read_text(encoding="utf-8"))
    hits = forbidden_fields(cfg)
    if hits:
        print(f"doctor: forbidden secret field(s): {', '.join(hits)}", file=sys.stderr)
        return 2
    print(f"ok map={cfg.get('map')} threshold={cfg.get('threshold_millionths')}")
    return 0


def cmd_compare(args) -> int:
    cfg = json.loads(Path(args.config).read_text(encoding="utf-8"))
    hits = forbidden_fields(cfg)
    if hits:
        print(f"doctor: forbidden secret field(s): {', '.join(hits)}", file=sys.stderr)
        return 2
    threshold = int(args.threshold or cfg.get("threshold_millionths", 50_000))
    rows = compare(Path(args.map), Path(args.quotes), threshold)
    out_path = Path(args.out) if args.out else None
    text = "".join(json.dumps(r, sort_keys=True) + "\n" for r in rows)
    print(text, end="")
    if out_path:
        out_path.write_text(text, encoding="utf-8")
    alerts = [r for r in rows if r["kind"] == "spread"]
    if args.notify_file and alerts:
        p = Path(args.notify_file)
        with p.open("a", encoding="utf-8") as fh:
            for r in alerts:
                fh.write(json.dumps(r, sort_keys=True) + "\n")
        print(f"notify file:{p}")
    return 0


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="oddsradar")
    sub = p.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("doctor")
    s.add_argument("--config", required=True)
    s.set_defaults(func=cmd_doctor)
    s = sub.add_parser("compare")
    s.add_argument("--config", required=True)
    s.add_argument("--map", required=True)
    s.add_argument("--quotes", required=True)
    s.add_argument("--threshold", type=int, default=None)
    s.add_argument("--out", default=None)
    s.add_argument("--notify-file", default=None)
    s.set_defaults(func=cmd_compare)
    return p


def main(argv=None) -> int:
    args = build_parser().parse_args(argv)
    return args.func(args)
