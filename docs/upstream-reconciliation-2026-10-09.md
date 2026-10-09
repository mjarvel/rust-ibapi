# Upstream reconciliation, 2026-10-09

## Scope and revisions

The user authorized pushing the existing socket-close commit on main to the fork,
updating main-upstream, and assessing/reconciling current upstream changes.
Default main pull/push resolve to origin/main. The existing main commit
934aaab4acf6a535eb014216d43efc24f56cffa9 was pushed to origin/main before integration.
The upstream mirror advanced from 0ff82e81df1fee315a84cef897289297a72fdc57 to
3e73f2f1cfac151c10e403a3e7d779272134445f, preserving pristine upstream source.

Reconciliation uses local branch codex/reconcile-upstream-2026-10-09 in
G:\repos\worktree-rust-ibapi-reconcile-2026-10-09. It combines our main/101 baseline
with 61 incoming upstream commits across 78 files, including the 5.0.0 release.
The package version follows upstream's 5.0.0; no existing numbered checkpoint,
consumer dependency, registry version pin in a consumer, or published executable
is changed by this native integration. No broker activity occurs.

## Resolution

The textual merge conflict was in CHANGELOG.md. Upstream's release history is
preserved alongside an explicit local-fork section for all retained native additions:
sender-only cancel_order_untracked, strict non-retrying fetch_once, finite
notice-preserving snapshot_once, and physical TCP disconnect.

An automatic source merge also placed an awaited socket close into upstream's
new synchronous request_shutdown function. Preserve that function's runtime-free
subscription termination, and await physical close in explicit ensure_shutdown
before joining the dispatcher, plus in the dispatcher's terminal cleanup. This
retains retained-client/subscription EOF and best-effort drop semantics without
blocking inside a synchronous drop path. The private AsyncIo close operation,
terminal socket latch and read/write wakeup tests remain unchanged.

Two sender-only cancellation test assertions refer to upstream's new orders
registry rather than the removed order_channels lock. Their intent is unchanged:
no per-order route is created by cancellation, and execution/fee observation is
not diverted. Production history and physical-close regressions remain active.

## Consumer compatibility boundary

An isolated archive of committed rust-library1 was tested against the candidate.
Only that temporary copy's native path/version and protobuf fixture include were
changed to the reconciliation worktree and =5.0.0. Its 102 unit tests and four
integration tests pass on native's pinned Rust 1.95.0, including production timeout
recovery, exactly-once feed restoration, stock/futures requests and order routing.
The real library dev and all frozen consumer worktrees remain untouched.

This is offline compatibility evidence, not version-5 consumer adoption or live
acceptance. Future adoption must create a fresh native checkpoint, change the
library's native version/path/BACKEND_REVISION/probe defaults, validate and freeze
a new library checkpoint, then deliberately update application pins.

## Native verification

The default native unit suite passes 1,664 tests with three pre-existing Windows
fixture failures: test_reconnect_clears_a_broken_connection,
test_write_abandoned_before_it_starts_sends_nothing, and
test_write_cut_off_mid_frame_breaks_the_connection. Their large writes buffer
fully on this machine rather than stalling. They remain active and unchanged.
This is the same limitation recorded for native 101, not a new reconciliation
regression.

All native checks use the pinned Rust 1.95.0, offline cached dependencies and a
shared native target directory. Integration packages are compiled without running
broker tests.

| Check | Default async | Sync only | All features |
|---|---|---|---|
| Unit tests | 1,664 passed; 3 known failures | 1,680 passed | 2,160 passed; 3 known failures |
| Doctests | 253 passed; 8 ignored | 259 passed; 6 ignored | 374 passed; 8 ignored |
| Strict Clippy, all targets | Passed | Passed | Passed |
| Rustdoc with warnings denied | Passed | Passed | Passed |
| Example builds | Passed | Passed | Not required |

Async and sync integration test packages compile successfully and pass strict Clippy.
Formatting and
Git whitespace checks pass. The rule-graph validator passes after adding the
fork's checkout-routing convention to the existing rule index.

The isolated library also passes strict all-target/all-feature Clippy on its
normal stable toolchain (Rust 1.97.1). Checking that library copy with native's
older Rust 1.95.0 instead reports its pre-existing nonminimal_bool warning in
client.rs; no real library source was changed to silence a toolchain-specific
style diagnostic. Its complete test suite passed on 1.95.0 as recorded above.

This reconciliation is an integration state on main, not a new frozen checkpoint
or production acceptance. Only the pre-existing 934aaab commit was published to
origin/main as authorized; the reconciled merge is initially kept local.
