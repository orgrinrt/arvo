# Dispatcher note: no committed bench here has ever checked that its arms agree

**Position:** after `40`. **Author:** the dispatching agent, verifying a finding `40` reported outside
its question. **Standing:** a measurement, not a design finding. Carries no authority over any open
question.

Recorded separately from `40` because it bears on **every** number this panel holds, and a finding
that qualifies all the evidence should not sit inside one file about a different subject.

## The measurement

Across every committed bench artifact in `mock/benches`:

```
awk -F',' 'FNR>1 {t++; if ($13!=0 && $13!="") i++; if ($14!=0) c++; if ($17!=0) d++}
           END {printf "rows=%d instructions=%d cycles=%d digest=%d\n", t, i+0, c+0, d+0}' *.csv
  -> rows=82960 instructions=0 cycles=0 digest=0
```

**214 CSV files, 82,960 rows, and the digest column is zero in every one.** So is instructions, and so
is cycles.

The digest is what would show that two arms computed the same answer. **No committed bench in this
repository has ever cross-checked that its variants agree**, which means every comparison here is a
comparison of timings between arms that were never shown to be doing the same work.

## Why this is the shape of a defect rather than a gap

`22` established that the harness never calls its own validation pass: it injected a one-character bug,
got 400 rows and exit 0. This is that defect's fingerprint in the data, across the whole corpus rather
than one run.

And the failure it permits is not hypothetical. `40` found the instance, and it is worse than reported.

In `precise-container-width-l1_n130103.csv`, mean `algo_ns` per variant:

| variant | mean |
|---|---|
| `warm-container-kernel` | **9.7 ns** |
| `warm-container-headroom` | 8,226.7 ns |
| `warm-container-plusone` | 10,763.1 ns |
| `warm-container-minimum` | 10,824.7 ns |
| `warm-container-native` | 11,236.8 ns |

An arm roughly a thousand times faster than every competitor did not do the work. And the harness's own
findings file does not flag it. It recommends it:

> ### warm-container-kernel dominates: 190167% faster than the next best (warm-container-headroom)
> _Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

**A nineteen-hundred-fold speedup is not a result, and the generator turned it into a recommendation.**
The one column that would have caught it is zero.

## What this does and does not invalidate

**It does not invalidate a comparison whose arms were separately shown to compute the same thing.** `26`
and `27` both attacked their own arms, disassembled them, and reported mechanisms; `27` explicitly
demonstrated fidelity rather than asserting it. Those stand on what their authors did, not on the
harness.

**It does invalidate any comparison resting on the harness having checked.** Nothing in the corpus
carries that check, so a number cited from a findings file without a mechanism behind it is a timing
of unknown work.

**It says nothing about the arithmetic in the probes.** `21` re-ran 51 numbers from the panel's probe
files and 50 reproduced exactly. Probes and benches are different evidence and only one of them is
implicated.

## What is owed

**The sweep is done, and the answer bounds the damage.** Predicate: an arm whose mean `algo_ns` is at
least 50x below the next best in its own file. Across all 214 CSVs it fires **6 times**, and all six
are the same family and the same arm:

| ratio | file | arm |
|---|---|---|
| 844x | `precise-container-width-l1_n130103` | `warm-container-kernel` 9.7 ns vs 8,226.7 |
| 839x | `precise-container-width-l1_n80103` | 12.7 ns vs 10,689.4 |
| 688x | `precise-container-width-l1_n320103` | 7.9 ns vs 5,434.6 |
| 607x | `precise-container-width-l1_n160103` | 13.5 ns vs 8,168.2 |
| 566x | `precise-container-width-l1_n640103` | 9.7 ns vs 5,473.8 |
| 318x | `precise-container-width-l1_n600103` | 25.8 ns vs 8,196.7 |

**And the control is decisive.** The identical arm in the sibling family `warm-container-width-l1` sits
in the pack at 946.7 ns, third of six, against a field of 420 to 8,690. So the arm is not inherently
broken and the harness is not systematically wrong: **one variant, under one configuration, stopped
doing its work**, and nothing in the corpus could tell.

That is the honest size of it. The corpus cannot certify itself, which is the finding; the corpus is
not riddled with dead arms, which is worth saying with the same emphasis.

**The upstream fix**, which is the same one `22` named and which has not moved:
`mockspace-bench-core`'s orchestrator never calls validate and `run_worker_validate` is not
re-exported, so no consumer can reach it. Verified in a clean clone. Deliberately not fixed here,
because widening a one-repo instruction is op's call.

**Nothing in the panel needs re-running because of this note.** The finding is that the corpus cannot
certify itself, not that its numbers are known wrong.
