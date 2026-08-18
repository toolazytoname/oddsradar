from __future__ import annotations

import csv
import json
from collections import defaultdict
from pathlib import Path

from oddsradar.spread import Quote, alert_if_wide, parse_prob


def load_map(path: Path) -> list[dict]:
    rows = []
    with path.open(newline="", encoding="utf-8") as fh:
        for row in csv.DictReader(fh):
            rows.append(row)
    return rows


def quotes_from_fixture(map_path: Path, quotes_path: Path) -> dict[str, list[Quote]]:
    mapping = load_map(map_path)
    key = {(r["venue"], r["market_id"]): r["event_id"] for r in mapping}
    raw = json.loads(quotes_path.read_text(encoding="utf-8"))
    by_event: dict[str, list[Quote]] = defaultdict(list)
    for q in raw["quotes"]:
        event_id = key.get((q["venue"], q["market_id"]))
        if event_id is None:
            continue
        by_event[event_id].append(
            Quote(
                event_id=event_id,
                venue=q["venue"],
                market_id=q["market_id"],
                yes=parse_prob(str(q["yes"])),
            )
        )
    return dict(by_event)


def compare(map_path: Path, quotes_path: Path, threshold: int) -> list[dict]:
    out = []
    for event_id, qs in quotes_from_fixture(map_path, quotes_path).items():
        if len(qs) < 2:
            continue
        alert = alert_if_wide(qs, threshold)
        if alert:
            out.append(alert.to_public_dict())
        else:
            from oddsradar.spread import spread_millionths

            out.append(
                {
                    "kind": "ok",
                    "event_id": event_id,
                    "spread_millionths": spread_millionths(qs),
                    "threshold_millionths": threshold,
                    "venues": {q.venue: q.yes for q in qs},
                }
            )
    return out
