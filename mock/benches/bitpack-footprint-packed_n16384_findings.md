# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 349% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (2.72 us) leads bitpack-footprint-packed-naive (12.20 us) by 349%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed shows warm-up / thermal drift (autocorr +0.83)

bitpack-footprint-packed's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (2.72 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.5x the fastest

Fastest bitpack-footprint-packed (2.72 us) to slowest bitpack-footprint-packed-naive (12.20 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 2718.8 ns median
- 1 variant significantly slower than baseline
- Spread: 4.49x (fastest 2718.8 ns, slowest 12199.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 2785ns | 2799ns | 2596ns | 2748ns | 3083ns | base |
| bitpack-footprint-packed-naive | 12307ns | 12273ns | 11308ns | 12005ns | 14212ns | +341.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 2706ns | 2523ns | 2996ns | base | 6.056 |
| bitpack-footprint-packed-naive | 12225ns | 11237ns | 14107ns | +351.85% | 1.340 |

## Performance model

- Peak throughput: **6.494 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 6.026 | 92.8% |
| bitpack-footprint-packed-naive | 1.343 | 20.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 2785ns | 2785ns | base |
| bitpack-footprint-packed-naive | 12307ns | 12307ns | +341.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 2719ns | base | --- | [2527, 2744] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 12199ns | +9209.6ns (+338.7%) | [+8835, +9479]ns | [11364, 12210] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|
| 1 | 2527ns | +347.7% |
| 2 | 2526ns | +345.8% |
| 3 | 2532ns | +344.9% |
| 4 | 2641ns | +326.5% |
| 5 | 2739ns | +324.4% |
| 6 | 2742ns | +311.3% |
| 7 | 2744ns | +310.1% |
| 8 | 2743ns | +342.7% |
| 9 | 2745ns | +345.0% |
| 10 | 2745ns | +363.3% |
| 11 | 2795ns | +336.5% |
| 12 | 2527ns | +382.9% |
| 13 | 2530ns | +387.0% |
| 14 | 2522ns | +384.1% |
| 15 | 2525ns | +383.3% |
| 16 | 2525ns | +403.7% |
| 17 | 2700ns | +351.3% |
| 18 | 2740ns | +310.0% |
| 19 | 2738ns | +310.1% |
| 20 | 2738ns | +310.5% |
| 21 | 2996ns | +307.2% |
| 22 | 2993ns | +307.6% |
| 23 | 2998ns | +307.3% |
| 24 | 2995ns | +309.6% |
| 25 | 2995ns | +307.3% |
| 26 | 2995ns | +315.9% |
| 27 | 2995ns | +345.1% |
| 28 | 2992ns | +345.4% |
| 29 | 2995ns | +345.1% |
| 30 | 2996ns | +326.3% |
| 31 | 2522ns | +410.8% |
| 32 | 2520ns | +413.0% |
| 33 | 2525ns | +754.0% |
| 34 | 2526ns | +346.1% |
| 35 | 2528ns | +344.6% |
| 36 | 2529ns | +344.4% |
| 37 | 2522ns | +352.6% |
| 38 | 2526ns | +344.6% |
| 39 | 2522ns | +345.6% |
| 40 | 2525ns | +360.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | 0.833 | HIGH+ (drift/warm-up) |
| bitpack-footprint-packed-naive | 0.127 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 1.9ns | 2705.6ns | 0.1% |  |
| bitpack-footprint-packed-naive | 3.8ns | 12225.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 2523.1-2995.8 ns)
   2523.1 |########################################
   2546.7 |
   2570.3 |
   2594.0 |
   2617.6 |###
   2641.2 |
   2664.9 |
   2688.5 |###
   2712.2 |
   2735.8 |###########################
   2759.4 |
   2783.1 |###
   2806.7 |
   2830.3 |
   2854.0 |
   2877.6 |
   2901.2 |
   2924.9 |
   2948.5 |
   2972.1 |#####################
  (5 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 11237.5-14107.3 ns)
  11237.5 |################################
  11381.0 |###
  11524.5 |#######
  11668.0 |
  11811.5 |
  11954.9 |
  12098.4 |########################################
  12241.9 |#######
  12385.4 |###
  12528.9 |
  12672.4 |##########
  12815.9 |#######
  12959.4 |
  13102.9 |
  13246.3 |##########
  13389.8 |
  13533.3 |
  13676.8 |
  13820.3 |
  13963.8 |
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-footprint-packed**: autocorrelation=0.83 (measurement drift or warm-up artifact)
