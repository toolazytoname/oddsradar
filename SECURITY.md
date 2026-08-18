# Security

oddsradar is a **read-only** market observer. It must never hold keys that can place bets or transfer funds.

## Rules

- Config may contain public market ids, venue API bases, and notify tokens.
- Forbidden: private keys, venue trading credentials, wallet seeds.
- Notification tokens live in `.env` (`chmod 0600`), never in git.
- No custody, no betting from this process.
- Respect venue API terms. Prefer official APIs over scraping.

## Reporting

Open a private GitHub security advisory, or contact the maintainer on the GitHub profile.
