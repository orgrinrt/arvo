# Sketch: live-count CSR transpose, subtopic A

**Round:** 202605291942 (csr-live-counts)
**Date:** 2026-05-29
**Status:** WORKS (bug reproduced, fix validated)
**Tracks:** task #634

## Hypothesis

arvo's packed `Csr::with_transpose` (last row runs to `cap_size(NNZ)`, scatter
loop runs over `0..cap_size(NNZ)`) miscounts when fed a capacity-with-slack
matrix where `live_nnz < cap_size(NNZ)` and the tail `col_idx[live_nnz..]` holds
the constructor default `NodeId(0)`. Every slack-tail slot is counted as an edge
into node 0, so node 0 gains spurious predecessors. The proposed live-count
variant (scatter and row bounds use `live_nnz` / `live_rows`) reads no slack and
produces the correct reverse adjacency.

## Method

The corruption lives in the loop bounds, not in arvo's type machinery, so the
probe models arvo `csr.rs`'s `row_end` last-row rule plus the count-prefix
portion of `with_transpose` in plain `usize`, with no arvo dependency. Graph:
node 0 to node 1, node 1 to node 0. Caps `ROWS = 4`, `NNZ = 8`. Live: 2 rows,
2 edges. Correct in-degree (predecessor count) per node is `[1, 1, 0, 0]`.

```rust
const ROWS_CAP: usize = 4;
const NNZ_CAP: usize = 8;

// row_ptr = [0, 1, 0, 0]; col_idx = [1, 0, 0, 0, 0, 0, 0, 0] (tail default 0)
// live_rows = 2, live_nnz = 2

// PACKED: count over k in 0..NNZ_CAP (= arvo's with_transpose today)
fn transpose_packed(col_idx: &[usize; NNZ_CAP]) -> [usize; ROWS_CAP] {
    let mut counts = [0usize; ROWS_CAP];
    let mut k = 0;
    while k < NNZ_CAP {            // <-- counts the slack tail
        let col = col_idx[k];
        if col < ROWS_CAP { counts[col] += 1; }
        k += 1;
    }
    counts
}

// LIVE-COUNT: count over k in 0..live_nnz
fn transpose_live(col_idx: &[usize; NNZ_CAP], live_nnz: usize) -> [usize; ROWS_CAP] {
    let mut counts = [0usize; ROWS_CAP];
    let mut k = 0;
    while k < live_nnz {          // <-- slack never read
        let col = col_idx[k];
        if col < ROWS_CAP { counts[col] += 1; }
        k += 1;
    }
    counts
}
```

Compiled with `rustc --edition 2024` (no nightly features needed; plain `usize`).

## Outcome

```
packed in-degree (predecessor count) per node: [7, 1, 0, 0]
live   in-degree (predecessor count) per node: [1, 1, 0, 0]
expected correct in-degree: [1, 1, 0, 0]
```

- PACKED node 0 in-degree = 7: the real predecessor (node 1) plus the
  `NNZ_CAP - live_nnz = 6` default-`NodeId(0)` slack-tail slots, every one
  miscounted as an edge into node 0. The pollution reaches a real node, so it
  is not contained to the phantom tail rows.
- LIVE node 0 in-degree = 1: correct. The full result `[1, 1, 0, 0]` is the
  clean reverse adjacency.

Hypothesis CONFIRMED. The packed transpose corrupts real-node predecessor sets
on a slack input; bounding the scatter by `live_nnz` removes it.

## What this unblocks

- The doc CL for round 202605291942 can lock the live-count parameterisation
  (changes 1 to 5 in the topic) on validated ground.
- The same bound move applies to the algorithm seed loops (`0..node_count()`
  rather than `0..cap_size(N)`); subtopic B verifies that during SRC by running
  the packed-path suite green plus a slack-input test.
- hilavitkutin C1a then builds a `Csr` with `live_rows = unit_count`,
  `live_nnz = edge_count`, and the slack tail is never iterated.

## Note on the standalone probe source

The runnable probe lives at `/tmp/csr_live_counts_probe.rs` during the round
(adds the assertions and the print harness around the two functions above). It
is not committed as a separate `.rs` because the logic is fully captured in the
fenced blocks here and the probe carries no arvo dependency worth preserving as
a build target. The findings text plus git history is the audit trail.
