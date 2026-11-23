# Whether two 64-bit sets meet, through the bit tier and on the host integer

2 variants, 40 samples per variant.
Baseline: **bit_set_host**

## Highlights

Baseline for all deltas below: **bit_set_host**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bit_set_host) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bit_set_host has the worst median (61 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bit_set_tier at 46 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bit_set_tier dominates: 32% faster than the next best (bit_set_host)

bit_set_tier (46 ns) leads bit_set_host (61 ns) by 32%, a clear separation rather than a photo finish. CV 24.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bit_set_tier beats baseline by 25% (significant)

bit_set_tier is -15 ns (25%) faster than baseline bit_set_host, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bit_set_host is inconsistent: worst-20% is 1.9x its best-20%

bit_set_host's best 20% of batches run at 49 ns but its worst 20% at 94 ns (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bit_set_tier** at 46.1 ns median (-24.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.32x (fastest 46.1 ns, slowest 60.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Whether two 64-bit sets meet, through the bit tier and on the host integer |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x18e9dcd2ca520f33 |
| Counter frequency | 2112843413 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bit_set_host | 469ns | 415ns | 286ns | 407ns | 837ns | base |
| bit_set_tier | 306ns | 274ns | 230ns | 268ns | 497ns | -34.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bit_set_host | 69ns | 49ns | 94ns | base | 0.929 |
| bit_set_tier | 49ns | 40ns | 67ns | -28.76% | 1.304 |

## Performance model

- Peak throughput: **1.585 Gops/s** (bit_set_tier; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bit_set_host | 1.054 | 66.5% |
| bit_set_tier | 1.388 | 87.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bit_set_host | 469ns | 469ns | base |
| bit_set_tier | 306ns | 306ns | -34.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bit_set_host | 61ns | base | --- | [60, 80] | --- | --- | --- | --- |
| bit_set_tier | 46ns | -15.2ns (-25.1%) | [-36, -11]ns | [44, 48] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bit_set_host | bit_set_tier |
|---|---|---|---|
| 1 | 1 | 94ns | -56.8% |
| 1 | 2 | 60ns | -19.0% |
| 1 | 3 | 81ns | -45.6% |
| 1 | 4 | 49ns | +28.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bit_set_host | -0.464 | moderate- |
| bit_set_tier | -0.233 | moderate- |

**Consistency summary:**

- **bit_set_tier**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bit_set_host | 113.5ns | 68.9ns | 164.8% | HIGH |
| bit_set_tier | 81.0ns | 49.1ns | 165.0% | HIGH |

## Distribution (algo ns)

```
bit_set_host (n=40, range 49.5-93.9 ns)
     49.5 |
     51.7 |########################################
     53.9 |######
     56.1 |
     58.3 |########################################
     60.6 |####################
     62.8 |
     65.0 |
     67.2 |
     69.4 |
     71.7 |
     73.9 |######
     76.1 |
     78.3 |#################################
     80.5 |#################################
     82.8 |
     85.0 |
     87.2 |
     89.4 |
     91.7 |##########################
  (6 below, 3 above range)

bit_set_tier (n=40, range 40.4-67.3 ns)
     40.4 |##########################
     41.7 |####
     43.1 |########################################
     44.4 |####
     45.8 |
     47.1 |######################
     48.4 |########################################
     49.8 |####
     51.1 |
     52.5 |
     53.8 |
     55.2 |
     56.5 |
     57.9 |
     59.2 |
     60.5 |
     61.9 |
     63.2 |
     64.6 |
     65.9 |
  (3 below, 5 above range)

```

## Diagnostics

- **bit_set_host**: CV=24.5% (high variance, measurements may be unstable)
- **bit_set_host**: bridge=167.1% of algo (FFI overhead may distort results)
- **bit_set_tier**: CV=23.4% (high variance, measurements may be unstable)
- **bit_set_tier**: bridge=167.5% of algo (FFI overhead may distort results)
