# Whether two 64-bit sets meet, through the bit tier and on the host integer

2 variants, 40 samples per variant.
Baseline: **bit_set_host**

## Highlights

Baseline for all deltas below: **bit_set_host**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bit_set_host) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bit_set_host has the worst median (76 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bit_set_tier at 76 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 76 ns to slowest 76 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (6 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bit_set_tier** at 75.7 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 75.7 ns, slowest 75.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Whether two 64-bit sets meet, through the bit tier and on the host integer |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x18e9dce03ae59e33 |
| Counter frequency | 2112843413 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bit_set_host | 367ns | 328ns | 309ns | 341ns | 502ns | base |
| bit_set_tier | 370ns | 329ns | 311ns | 345ns | 504ns | +0.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bit_set_host | 77ns | 70ns | 86ns | base | 3.325 |
| bit_set_tier | 78ns | 71ns | 85ns | +1.76% | 3.267 |

## Performance model

- Peak throughput: **3.679 Gops/s** (bit_set_host; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bit_set_host | 3.377 | 91.8% |
| bit_set_tier | 3.382 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bit_set_host | 367ns | 367ns | base |
| bit_set_tier | 370ns | 370ns | +0.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bit_set_host | 76ns | base | --- | [75, 76] | --- | --- | --- | --- |
| bit_set_tier | 76ns | no significant difference | [-1, +0]ns | [75, 84] | no | 0.5224 | 0.5224 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bit_set_host | bit_set_tier |
|---|---|---|---|
| 1 | 1 | 85ns | -0.2% |
| 1 | 2 | 76ns | -1.3% |
| 1 | 3 | 69ns | +7.4% |
| 1 | 4 | 77ns | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bit_set_host | -0.027 | ok |
| bit_set_tier | -0.087 | ok |

**Consistency summary:**

- **bit_set_tier**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bit_set_host | 91.5ns | 77.0ns | 118.9% | HIGH |
| bit_set_tier | 93.4ns | 78.4ns | 119.2% | HIGH |

## Distribution (algo ns)

```
bit_set_host (n=40, range 69.6-85.6 ns)
     69.6 |
     70.4 |##
     71.2 |
     72.0 |
     72.8 |
     73.6 |
     74.4 |#############
     75.2 |########################################
     76.0 |#####
     76.8 |
     77.6 |##
     78.4 |
     79.2 |##
     80.0 |
     80.8 |
     81.6 |
     82.4 |
     83.2 |
     84.0 |##
     84.8 |################
  (5 below, 3 above range)

bit_set_tier (n=40, range 71.4-85.3 ns)
     71.4 |
     72.1 |
     72.8 |
     73.5 |
     74.2 |###################################
     74.9 |######################
     75.6 |##########################
     76.3 |
     77.0 |
     77.7 |
     78.4 |
     79.1 |####
     79.8 |
     80.5 |####
     81.2 |
     81.9 |
     82.5 |
     83.2 |########
     83.9 |####
     84.6 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **bit_set_host**: bridge=118.5% of algo (FFI overhead may distort results)
- **bit_set_tier**: bridge=118.9% of algo (FFI overhead may distort results)
