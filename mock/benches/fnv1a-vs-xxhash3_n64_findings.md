# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fnv1a dominates: 297% faster than the next best (xxhash3)

fnv1a (19 ns) leads xxhash3 (77 ns) by 297%, a clear separation rather than a photo finish. CV 171.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fnv1a is fastest but the noisiest (CV 171.4%)

fnv1a wins on median (19 ns) yet has the highest variance (CV 171.4%), while xxhash3 is the steadiest (CV 37.4%, 77 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fnv1a)

The baseline fnv1a is the fastest (19 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.0x the fastest

Fastest fnv1a (19 ns) to slowest xxhash3 (77 ns): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fnv1a is inconsistent: worst-20% is 3.5x its best-20%

fnv1a's best 20% of batches run at 16 ns but its worst 20% at 57 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fnv1a) is the fastest** at 19.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.97x (fastest 19.4 ns, slowest 77.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 156ns | 146ns | 138ns | 146ns | 201ns | base |
| xxhash3 | 155ns | 146ns | 135ns | 148ns | 196ns | -0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 26ns | 16ns | 57ns | base | 2.453 |
| xxhash3 | 81ns | 67ns | 107ns | +208.85% | 0.794 |

## Performance model

- Peak throughput: **3.891 Gops/s** (fnv1a; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 3.299 | 84.8% |
| xxhash3 | 0.830 | 21.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 156ns | 156ns | base |
| xxhash3 | 155ns | 155ns | -0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 19ns | base | --- | [18, 20] | --- | --- | --- | --- |
| xxhash3 | 77ns | +56.7ns (+292.3%) | [+54, +61]ns | [73, 79] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 18ns | +318.9% |
| 2 | 15ns | +430.5% |
| 3 | 19ns | +286.5% |
| 4 | 17ns | +355.6% |
| 5 | 20ns | +281.9% |
| 6 | 20ns | +295.4% |
| 7 | 21ns | +300.9% |
| 8 | 20ns | +318.9% |
| 9 | 16ns | +393.8% |
| 10 | 18ns | +342.9% |
| 11 | 18ns | +298.4% |
| 12 | 19ns | +298.9% |
| 13 | 18ns | +283.4% |
| 14 | 20ns | +250.0% |
| 15 | 18ns | +292.6% |
| 16 | 20ns | +271.9% |
| 17 | 18ns | +333.0% |
| 18 | 16ns | +383.3% |
| 19 | 16ns | +338.0% |
| 20 | 19ns | +277.6% |
| 21 | 22ns | +207.6% |
| 22 | 18ns | +302.8% |
| 23 | 20ns | +214.5% |
| 24 | 18ns | +302.8% |
| 25 | 20ns | +239.5% |
| 26 | 18ns | +283.8% |
| 27 | 20ns | +276.0% |
| 28 | 18ns | +300.6% |
| 29 | 213ns | -70.1% |
| 30 | 110ns | -38.5% |
| 31 | 17ns | +1429.3% |
| 32 | 21ns | +312.5% |
| 33 | 22ns | +281.0% |
| 34 | 20ns | +308.5% |
| 35 | 20ns | +339.5% |
| 36 | 20ns | +318.5% |
| 37 | 20ns | +348.0% |
| 38 | 24ns | +240.9% |
| 39 | 21ns | +290.4% |
| 40 | 17ns | +428.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.345 | moderate+ |
| xxhash3 | 0.019 | ok |

**Consistency summary:**

- **xxhash3**: won 2/40, lost 38/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 2.7ns | 26.1ns | 10.4% | HIGH |
| xxhash3 | 2.6ns | 80.6ns | 3.3% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 16.5-56.9 ns)
     16.5 |##################################
     18.5 |########################################
     20.5 |#############
     22.5 |##
     24.5 |
     26.6 |
     28.6 |
     30.6 |
     32.6 |
     34.6 |
     36.7 |
     38.7 |
     40.7 |
     42.7 |
     44.7 |
     46.8 |
     48.8 |
     50.8 |
     52.8 |
     54.8 |
  (4 below, 2 above range)

xxhash3 (n=40, range 67.0-107.5 ns)
     67.0 |#################################
     69.0 |####################
     71.1 |########################################
     73.1 |####################
     75.1 |######
     77.1 |########################################
     79.2 |#############
     81.2 |##########################
     83.2 |####################
     85.2 |######
     87.3 |#############
     89.3 |######
     91.3 |
     93.3 |
     95.3 |
     97.4 |
     99.4 |
    101.4 |
    103.4 |
    105.5 |
  (2 below, 1 above range)

```

## Diagnostics

- **fnv1a**: CV=127.5% (high variance, measurements may be unstable)
- **fnv1a**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fnv1a**: bridge=12.9% of algo (FFI overhead may distort results)
- **xxhash3**: CV=35.7% (high variance, measurements may be unstable)
