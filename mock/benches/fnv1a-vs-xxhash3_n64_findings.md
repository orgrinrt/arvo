# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fnv1a dominates: 298% faster than the next best (xxhash3)

fnv1a (18 ns) leads xxhash3 (71 ns) by 298%, a clear separation rather than a photo finish. CV 12.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fnv1a is fastest but the noisiest (CV 12.5%)

fnv1a wins on median (18 ns) yet has the highest variance (CV 12.5%), while xxhash3 is the steadiest (CV 5.8%, 71 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fnv1a)

The baseline fnv1a is the fastest (18 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.0x the fastest

Fastest fnv1a (18 ns) to slowest xxhash3 (71 ns): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fnv1a) is the fastest** at 17.9 ns median
- 1 variant significantly slower than baseline
- Spread: 3.98x (fastest 17.9 ns, slowest 71.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 138ns | 139ns | 132ns | 139ns | 143ns | base |
| xxhash3 | 141ns | 141ns | 134ns | 141ns | 148ns | +1.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 18ns | 14ns | 20ns | base | 3.629 |
| xxhash3 | 72ns | 67ns | 79ns | +308.29% | 0.889 |

## Performance model

- Peak throughput: **4.567 Gops/s** (fnv1a; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 3.575 | 78.3% |
| xxhash3 | 0.899 | 19.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 138ns | 138ns | base |
| xxhash3 | 141ns | 141ns | +1.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 18ns | base | --- | [18, 19] | --- | --- | --- | --- |
| xxhash3 | 71ns | +53.5ns (+298.9%) | [+52, +55]ns | [70, 72] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 18ns | +297.8% |
| 2 | 20ns | +254.0% |
| 3 | 16ns | +306.3% |
| 4 | 18ns | +295.4% |
| 5 | 19ns | +284.4% |
| 6 | 20ns | +250.5% |
| 7 | 20ns | +304.1% |
| 8 | 19ns | +325.5% |
| 9 | 16ns | +327.2% |
| 10 | 18ns | +297.7% |
| 11 | 20ns | +260.5% |
| 12 | 18ns | +286.6% |
| 13 | 18ns | +285.7% |
| 14 | 18ns | +288.0% |
| 15 | 19ns | +281.4% |
| 16 | 17ns | +331.6% |
| 17 | 17ns | +360.2% |
| 18 | 19ns | +309.9% |
| 19 | 19ns | +289.9% |
| 20 | 19ns | +277.6% |
| 21 | 20ns | +256.0% |
| 22 | 19ns | +249.5% |
| 23 | 19ns | +266.7% |
| 24 | 20ns | +258.5% |
| 25 | 18ns | +306.9% |
| 26 | 19ns | +260.4% |
| 27 | 19ns | +301.6% |
| 28 | 19ns | +284.4% |
| 29 | 15ns | +351.9% |
| 30 | 22ns | +205.4% |
| 31 | 15ns | +369.3% |
| 32 | 17ns | +329.3% |
| 33 | 18ns | +272.6% |
| 34 | 14ns | +413.0% |
| 35 | 12ns | +562.4% |
| 36 | 15ns | +411.0% |
| 37 | 15ns | +430.5% |
| 38 | 18ns | +327.9% |
| 39 | 13ns | +423.3% |
| 40 | 13ns | +445.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.318 | moderate+ |
| xxhash3 | 0.326 | moderate+ |

**Consistency summary:**

- **xxhash3**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 2.1ns | 17.6ns | 11.7% | HIGH |
| xxhash3 | 2.0ns | 72.0ns | 2.8% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 14.0-20.1 ns)
     14.0 |
     14.3 |#####
     14.6 |
     14.9 |#####
     15.2 |##########
     15.5 |##########
     15.8 |
     16.1 |
     16.4 |#####
     16.7 |
     17.0 |##########
     17.3 |#########################
     17.6 |###############
     17.9 |
     18.2 |#####
     18.6 |###############
     18.9 |
     19.2 |########################################
     19.5 |##########
     19.8 |####################
  (4 below, 1 above range)

xxhash3 (n=40, range 67.0-78.9 ns)
     67.0 |#################################
     67.6 |######
     68.2 |######
     68.8 |####################
     69.4 |#############
     70.0 |#############
     70.6 |#############
     71.2 |########################################
     71.8 |######
     72.3 |#############
     72.9 |######
     73.5 |####################
     74.1 |######
     74.7 |
     75.3 |
     75.9 |
     76.5 |######
     77.1 |######
     77.7 |######
     78.3 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **fnv1a**: bridge=11.7% of algo (FFI overhead may distort results)
