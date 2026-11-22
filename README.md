# kit-test

The test-framework kit — `test`/`property`/`benchmark`/`fuzz`/`snapshot`/`assert` plus the deterministic rand-stub backend (`handl-kit-test`).

`kit.hnl` at the repo root is the kit.hnl v1 curation manifest (proposal
P-004) for the in-language test surface: `{ name, description, license,
libs }` only — pure curation, no plugins/entry/glue. `libs` entries are
lib package names (the `lib-*` repo name minus the `lib-` prefix); each
package's `pkg.hnl` `deps` pull its contracts and transitive backends.
This manifest supersedes the Rust crate's role as the kit declaration —
the crate remains as the delivery binding until the rehoming sequenced
in P-004 lands.

## Layout

```text
kit.hnl              the kit.hnl v1 curation manifest
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
