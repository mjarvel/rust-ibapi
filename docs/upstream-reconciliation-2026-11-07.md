# Upstream reconciliation and native checkpoint 105, 2026-11-07

## Source

- `main-upstream` fast-forwarded `37a7e9681472047b4d4558c275b1ffef3afcf7f7`
  (5.2.0) → `01714527` (122 commits). Upstream `main` is now `6.0.0-dev`:
  releases 5.3.0–5.6.0 plus the start of the breaking 6.0 series. 5.7.0 was
  cut from upstream's `release/5.x` branch and is not in this lineage; its
  fixes (#1172, #1177, #1179) are on `main` too.
- Temporary branch `reconcile-upstream-2026-11-07` from fork `main` 104
  (`7378f3731e19dec0fb5861590dd942ab0113f255`), worktree
  `G:\repos\rust-ibapi-worktree-reconcile-upstream-2026-11-07`.

## Conflict resolution

- `AGENTS.md`, `README.md`, `CHANGELOG.md`: upstream text adopted; the fork
  routing line, fork API paragraph and the fork changelog section are kept.
- Historical data (`builder/data.rs`, `sync.rs`, `async.rs`): upstream now
  carries a `BarRequest`, a builder-owned request id, a one-shot deadline and
  a typed `HistoricalDataFrame`, and adds its own `subscribe()` (sent once,
  caller-owned). Fork `fetch_once` and `snapshot_once` are kept and rebuilt on
  those internals: they use the builder's request id, `fetch_once` reads frames
  through upstream's `next_frame` (so it honours `one_shot_timeout` when set
  and, like `fetch`, cancels at TWS when it fails before the window frame), and
  neither ever resends. `snapshot_once` keeps `HistoricalSnapshotUpdate` with
  optional wire values, which `subscribe()` does not offer.
- Fork tests follow upstream renames (`CommissionAndFeesReport`,
  `CommissionsAndFeesReport`, `commission_and_fees`). The strict-history tests
  now count `RequestHistoricalData` writes instead of all writes, because the
  early-failure cancel (#1094) is a second, intended write; "never replayed"
  still means exactly one request.
- `docs/rules/workflow/local-fork-workflow.md` drops its `memory:` key, which
  upstream's rule validator now rejects (#1165).

## Offline Windows validation

Rust 1.95.0 (pinned), shared target `G:\repos\rust_target`.

| Check | Default async | Sync only | All features |
| --- | --- | --- | --- |
| Unit tests | 1833 passed; 6 failed | 1862 passed; 1 failed | 2438 passed; 7 failed |
| Doctests | 325 passed; 8 ignored | 332 passed; 6 ignored | 463 passed; 8 ignored |
| Clippy all targets, `-D warnings` | Passed | Passed | Passed |
| Rustdoc, `-D warnings` | Passed | Passed | Passed |
| Example builds | Passed | Passed | – |

`cargo fmt --check` passes; both integration crates build their tests (not
run: no broker). `tools/check-rules-graph.sh`: 38 nodes, all links resolve.

The 7 failures are the known Windows loopback fixture failures plus one new
upstream test of the same fixture family
(`transport::async::io::tests::sever_fails_a_stuck_write_until_reconnect`).
All 7 fail identically on an untouched archive of `main-upstream` 01714527
(433 passed; 7 failed in `transport::`). Logs are in ignored
`sandbox/reconciliation-2026-11-07`.

## Freeze and publication

Fork `main` is fast-forwarded to the reconcile merge, tagged `105`, and pushed
with `main-upstream` and the tag to our origin. No numerical branch or worktree
is created; nothing is pushed upstream. Consumer pins are unchanged: adopting
105 in `rust_broker_tws` is a separate task (6.0 API changes reach its internal
mapping code; see its `docs/rust-ibapi-changelog.md`).
