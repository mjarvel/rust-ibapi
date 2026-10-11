# Upstream reconciliation and native checkpoint 102, 2026-10-12

## Source and contribution provenance

Fork main starts at `c0e8953c46292fe3bdf45cc8ea9fe50d45d183ce` and integrates
pristine upstream `73fbbc2c350a299d032988b107ae399094a1f3f2`, package 5.0.0.
`main-upstream` advances by fast-forward only in
`G:\repos\rust-ibapi-worktree-main-upstream`. The fixed dated upstream base,
original contribution worktrees and numerical 100/101 remain unchanged.

- PR #966 merged at `813d632bd84e3a291ef3b3228ec14e76889bceb2`. Follow-ups
  #980 (`9572625e31015d2af871a6787d5760548c21ce63`) and #982
  (`6397adf7d45c0be3eb4050791be43c590872ffa5`) distinguish clean shutdown from
  a broken write, close both blocking socket directions, and retain terminal
  closure through reconnect races. Later async SocketState consolidation and
  its review correction are retained. Transport implementation/fixtures are
  identical to current upstream; our obsolete parallel close implementation
  and duplicate tests are removed.
- PR #967 merged at `7246151db50bb3d75bfb1da4c6e6c60b6d019660`. Finite
  populated/empty fetches require HistoricalDataEnd, reject unexpected frame
  types and premature EOF, and retain ordinary ConnectionReset retry.
  The local single-attempt helper shares these maintainer checks.

Full chronological records, original submitted patches, final PR heads and
maintainer changes live in `G:\repos\rust-library1\docs\patches\rust-ibapi`:
001-physical-disconnect.merged.md and 002-historical-completion.merged.md.
The original PR worktree diffs pass reverse-application checks against their
fixed original heads. Maintainer-reported live tests are external evidence;
all checks in this reconciliation are offline.

## Retained fork contracts and conflict resolution

Our extra APIs are separate from those upstream PRs:

- Sync/async cancel_order_untracked validates the ID and sends one cancellation
  without installing an order subscription or diverting observation traffic.
  It follows current upstream encoding; the obsolete below-floor version check
  is removed because the connection floor is now 213.
- Sync/async fetch_once never retries a reset. snapshot_once preserves notices,
  optional bar values and End/range metadata and never retries. These deliberately
  authorized fork APIs are exceptions to the upstream general retrying one-shot
  convention; removing that policy would break wrapper recovery semantics.
- The new stream-only buffer_limit is rejected before a write by all finite
  entry points, including local fetch_once/snapshot_once. Scalar WhatToShow and
  current validation/encoder signatures are adopted privately.
- HistoricalSnapshotUpdate End remains a delivered envelope. It is not a native
  decoder EndOfStream error; decoder registration tests preserve that distinction.

No new Windows transport fix is bundled into this reconciliation. Native API
additions are covered by the retained strict-history, notice/value preservation,
ID-validation, sender-routing and finite-buffer-limit regressions.

## Offline Windows validation

Pinned Rust 1.95.0, debug profiles, cached dependencies, --offline --locked
(after deliberate offline re-resolution of the ignored native Cargo.lock).

| Check | Default async | Sync only | All features |
| --- | --- | --- | --- |
| Unit tests | 1712 passed; 5 failures | 1745 passed; 1 failure | 2241 passed; 6 failures |
| Doctests | 266 passed; 8 ignored | 273 passed; 6 ignored | 396 passed; 8 ignored |
| All-target Clippy, warnings denied | Passed | Passed | Passed |
| Rustdoc, warnings denied | Passed | Passed | Passed |
| Debug example builds | Passed | Passed | Not required |

Both integration test packages compile and pass strict Clippy; they are never
executed against a broker. Package formatting passes. The rule validator is run
directly through Git Bash because just is unavailable on this machine; sandboxed
Git Bash cannot create its signal pipe, so that read-only check runs outside the
sandbox. It passes: 36 nodes, all links and edges resolve (rules-check.log).

Logs live in ignored `sandbox/reconciliation-2026-10-12`. The authoritative
async unit/doc reruns are default-unit-final.log/default-doc-tests-final.log.
A pristine-source check initially shared a Cargo target and package identity;
source freshness was explicitly forced before those final native reruns to avoid
reusing its test artifact. The pristine blocking check uses a separate target.
The first default unit count in validation.json is superseded by the final log.

All failing test names remain active:

- test_reconnect_clears_a_broken_connection
- test_write_abandoned_before_it_starts_sends_nothing
- test_write_cut_off_mid_frame_breaks_the_connection
- test_physical_close_overrides_a_broken_connection
- test_physical_close_wakes_a_backpressured_write
- test_close_wakes_a_blocked_read_and_is_terminal (blocking)

The five async failures reproduce on an untouched archive of upstream 73fbbc2c:
3 focused I/O tests pass and 5 fail. Large loopback writes complete instead of
remaining pending, so setup fails before the intended backpressure assertions.
This confirms the fixture failure, not a proven production write defect.
The untouched-upstream isolated blocking test also fails (0 passed; 1 failed,
about 1.01 seconds): Shutdown is returned, but elapsed time reaches the configured
read timeout. Whether socket wakeup or fixture synchronization causes the delay
needs investigation. Numbered drafts 003 and 004 keep these distinct. Async
retained-owner EOF/read wakeup/terminal-state and blocking physical teardown/race
checks pass; the overall native unit suite is explicitly not all green.

## Freeze, publication and adoption boundary

After reconciliation validation and commit, native 102 is created at
`G:\repos\rust-ibapi-worktree-102` and is immediately immutable. Its exact final
hash is appended on library main in the centralized rust-ibapi changelog and
checkpoint record; the frozen source is never edited to insert its own hash.
Native main and the existing origin/main-upstream mirror may be published to the
existing fork origin as authorized. Native 102 stays local. Library main alone
adopts native 102; existing library/client numerical versions are untouched.
The wrapper's production loopback/regression checks are recorded separately.

No broker session, orders, app launch, client build/migration, native IBX change,
new PR, branch deletion or force push is included. Cleanup is a documented manual
recommendation after dependency/ignored-file audits, not an agent deletion.
