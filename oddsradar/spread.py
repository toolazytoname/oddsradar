"""Implied-probability spread. Millionths integer scale (1_000_000 = 100%)."""

from __future__ import annotations

from dataclasses import dataclass

PROB_SCALE = 1_000_000  # millionths


@dataclass(frozen=True)
class Quote:
    event_id: str
    venue: str
    market_id: str
    yes: int  # implied P(yes) in millionths, 0..PROB_SCALE


@dataclass(frozen=True)
class SpreadAlert:
    event_id: str
    spread: int
    threshold: int
    quotes: tuple[Quote, ...]

    def to_public_dict(self) -> dict:
        return {
            "kind": "spread",
            "event_id": self.event_id,
            "spread_millionths": self.spread,
            "threshold_millionths": self.threshold,
            "venues": {q.venue: q.yes for q in self.quotes},
        }


def parse_prob(text: str) -> int:
    """Parse 0-1 decimal or already-integer millionths string."""
    s = text.strip()
    if "." in s:
        whole, frac = s.split(".", 1)
        if not whole:
            whole = "0"
        if whole.startswith("-") or not whole.isdigit() or not frac.isdigit():
            raise ValueError(f"bad prob: {text}")
        frac = (frac + "000000")[:6]
        val = int(whole) * PROB_SCALE + int(frac)
    else:
        if not s.isdigit():
            raise ValueError(f"bad prob: {text}")
        val = int(s)
        # bare 0-1 without dot is millionths already if > 1, else whole probability
        if val <= 1:
            val *= PROB_SCALE
    if val < 0 or val > PROB_SCALE:
        raise ValueError(f"prob out of range: {text}")
    return val


def spread_millionths(quotes: list[Quote]) -> int:
    if len(quotes) < 2:
        raise ValueError("need at least two venue quotes")
    ys = [q.yes for q in quotes]
    return max(ys) - min(ys)


def alert_if_wide(quotes: list[Quote], threshold: int) -> SpreadAlert | None:
    if len({q.event_id for q in quotes}) != 1:
        raise ValueError("quotes must share event_id")
    spr = spread_millionths(quotes)
    if spr <= threshold:
        return None
    return SpreadAlert(event_id=quotes[0].event_id, spread=spr, threshold=threshold, quotes=tuple(quotes))
