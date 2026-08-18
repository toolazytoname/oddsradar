from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from oddsradar.spread import Quote, alert_if_wide, parse_prob, spread_millionths  # noqa: E402

BIN = [sys.executable, "-m", "oddsradar"]


def run(args, cwd=None):
    import os

    return subprocess.run(
        BIN + args,
        cwd=cwd or ROOT,
        capture_output=True,
        text=True,
        env={**os.environ, "PYTHONPATH": str(ROOT)},
    )


class TestSpreadMath(unittest.TestCase):
    def test_parse_and_spread(self):
        a = Quote("e", "pm", "1", parse_prob("0.62"))
        b = Quote("e", "kx", "2", parse_prob("0.50"))
        self.assertEqual(a.yes, 620_000)
        self.assertEqual(spread_millionths([a, b]), 120_000)
        alert = alert_if_wide([a, b], 50_000)
        self.assertIsNotNone(alert)
        self.assertIsNone(alert_if_wide([a, Quote("e", "kx", "2", parse_prob("0.619"))], 50_000))

    def test_one_quote_rejected(self):
        with self.assertRaises(ValueError):
            spread_millionths([Quote("e", "pm", "1", 1)])


class TestCLI(unittest.TestCase):
    def test_wide_alerts_tight_does_not(self):
        r = run(
            [
                "compare",
                "--config",
                str(ROOT / "fixtures/config.ok.json"),
                "--map",
                str(ROOT / "fixtures/markets.csv"),
                "--quotes",
                str(ROOT / "fixtures/quotes_wide.json"),
            ]
        )
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn('"kind": "spread"', r.stdout)
        self.assertIn("btc-100k", r.stdout)
        # fed is tight
        lines = [json.loads(x) for x in r.stdout.strip().splitlines() if x.startswith("{")]
        fed = next(x for x in lines if x["event_id"] == "fed-cut")
        self.assertEqual(fed["kind"], "ok")

        r2 = run(
            [
                "compare",
                "--config",
                str(ROOT / "fixtures/config.ok.json"),
                "--map",
                str(ROOT / "fixtures/markets.csv"),
                "--quotes",
                str(ROOT / "fixtures/quotes_tight.json"),
            ]
        )
        self.assertEqual(r2.returncode, 0, r2.stderr)
        self.assertNotIn('"kind": "spread"', r2.stdout)

    def test_doctor_secret(self):
        r = run(["doctor", "--config", str(ROOT / "fixtures/config.secret.json")])
        self.assertNotEqual(r.returncode, 0)
        self.assertNotIn("PLANT-SECRET-DO-NOT-LOG", r.stdout + r.stderr)

    def test_notify_file(self):
        with tempfile.TemporaryDirectory() as td:
            td = Path(td)
            r = run(
                [
                    "compare",
                    "--config",
                    str(ROOT / "fixtures/config.ok.json"),
                    "--map",
                    str(ROOT / "fixtures/markets.csv"),
                    "--quotes",
                    str(ROOT / "fixtures/quotes_wide.json"),
                    "--notify-file",
                    str(td / "a.jsonl"),
                ]
            )
            self.assertEqual(r.returncode, 0, r.stderr)
            body = (td / "a.jsonl").read_text()
            self.assertIn("btc-100k", body)
            self.assertNotIn("fed-cut", body)


if __name__ == "__main__":
    unittest.main()
