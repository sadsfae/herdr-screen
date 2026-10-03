# Config reference

Every canonical `config.toml` key that herdr-screen reads, with types, defaults,
and allowed values.

The authoritative, generated reference is at
`docs/next/website/src/data/config-reference.json` (produced by
`scripts/config_reference_check.py`). Browse it for the full flat list.

Print the full commented default config at any time:

```bash
herdr-screen --default-config
```

Custom command bindings (`[[keys.command]]`) are user-defined tables and are
not enumerated per key; they are documented in the upstream Herdr
configuration guide. See [Configuration](/docs/configuration/) in the upstream
docs for setup guidance.
