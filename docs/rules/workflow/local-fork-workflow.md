---
id: local-fork-workflow
title: Route work through this fork's main and preserve frozen consumer pins
cluster: workflow
status: active
triggers:
  - starting work in the local fork or choosing a checkout
  - pulling upstream updates or reconciling them with our patches
  - freezing a checkpoint or changing a consumer dependency
symbols: [main, main-upstream, origin, upstream]
related: [pre-pr-checks, pinned-toolchain]
precedents: []
memory: []
---

Read `G:\repos\info\README.md` before work and follow its branch/worktree rules.
Develop on main or a separate temporary worktree; advance main-upstream only by
fast-forward from upstream. Keep numbered checkpoints immutable immediately.
Verify origin as the publication destination. Change consumer pins only as part
of an authorized adoption task, after validation and a fresh checkpoint.

## Why

This is the user's fork: main integrates our patches, while main-upstream keeps
an unmodified upstream lineage. A merge into main must not silently change the
native source that existing library or application checkpoints consume.

## Precedents

- The user's 2026-10-09 branch migration establishes these local roles. See
  [the reconciliation record](../../upstream-reconciliation-2026-10-09.md) and
  [the root README](../../../README.md#local-fork-workflow). Existing native
  100/101 and the library's adopted 101 path remain unchanged.
