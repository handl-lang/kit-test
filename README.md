# kit-test

The test-framework kit — `test`/`property`/`benchmark`/`fuzz`/`snapshot`/`assert` plus the deterministic rand-stub backend (`handl-kit-test`).

## Layout

```text
crates/kit-test/    the handl-kit-test crate
```

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate; `ci.sh` is the gate entry. Sibling deps are git
dependencies (`branch = "main"`); the workspace-root `[patch]` tables resolve
them from sibling checkouts for local development.

## License

MIT.
