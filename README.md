# kit-test

The test-framework kit — a **manifest-only** repo. `kit.hnl` at the root is
the kit.hnl v1 curation manifest (proposal P-004) for the in-language test
surface: `{ name, description, license, libs }` only — pure curation, no
plugins/entry/glue. `libs` entries are lib package names (the `lib-*` repo
name minus the `lib-` prefix); each package's `pkg.hnl` `deps` pull its
contracts and transitive backends.

The Rust delivery-binding crate (`handl-kit-test`) moved to
[`handl-lang/tool-test`](https://github.com/handl-lang/tool-test)
(`crates/kit-test`) — it is test-framework machinery, not a kit, so it
belongs to the engine family. Consumers depend on it as
`handl-kit-test = { git = "https://github.com/handl-lang/tool-test" }`.

## Layout

```text
kit.hnl    the kit.hnl v1 curation manifest
```

## License

MIT.
