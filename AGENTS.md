# kit-test

The test-framework kit — a **manifest-only** repo. `kit.hnl` at the root
is the kit.hnl v1 curation manifest (proposal P-004) for the
in-language test surface: `{ name, description, license, libs }` only —
pure curation, no plugins/entry/glue.

## Layout

| path | role |
| --- | --- |
| `kit.hnl` | the kit.hnl v1 curation manifest — the whole repo |
| `workflows/` | bupha gate wiring |

## Contract

- `libs` entries are lib package names (the `lib-*` repo name minus the
  `lib-` prefix); each package's `pkg.hnl` `deps` pull its contracts and
  transitive backends. Contracts whose backends declare no `deps`
  (`rand-stub`, `posix-stub`) are listed explicitly alongside their stub.
- The Rust delivery-binding crate (`handl-kit-test`) moved to
  [`handl-lang/tool-test`](https://github.com/handl-lang/tool-test)
  (`crates/kit-test`) — it is test-framework machinery, not a kit.
  Consumers depend on it as
  `handl-kit-test = { git = "https://github.com/handl-lang/tool-test" }`.

## Verification

Manifest-only repo — no build.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
