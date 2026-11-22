# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 477% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (2.03 us) leads bitpack-footprint-packed-naive (11.71 us) by 477%, a clear separation rather than a photo finish. CV 122.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed is fastest but the noisiest (CV 122.4%)

bitpack-footprint-packed wins on median (2.03 us) yet has the highest variance (CV 122.4%), while bitpack-footprint-packed-naive is the steadiest (CV 20.4%, 11.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (2.03 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.8x the fastest

Fastest bitpack-footprint-packed (2.03 us) to slowest bitpack-footprint-packed-naive (11.71 us): 5.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-packed is inconsistent: worst-20% is 4.0x its best-20%

bitpack-footprint-packed's best 20% of batches run at 2.00 us but its worst 20% at 7.99 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 2027.8 ns median
- 1 variant significantly slower than baseline
- Spread: 5.77x (fastest 2027.8 ns, slowest 11710.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112837746 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 3846ns | 2079ns | 2055ns | 2974ns | 8252ns | base |
| bitpack-footprint-packed-naive | 12138ns | 11782ns | 10434ns | 11629ns | 15368ns | +215.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 3738ns | 2001ns | 7989ns | base | 4.384 |
| bitpack-footprint-packed-naive | 12057ns | 10372ns | 15254ns | +222.60% | 1.359 |

## Performance model

- Peak throughput: **8.187 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 8.079 | 98.7% |
| bitpack-footprint-packed-naive | 1.399 | 17.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 3846ns | 3846ns | base |
| bitpack-footprint-packed-naive | 12138ns | 12138ns | +215.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 2028ns | base | --- | [2018, 2472] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 11710ns | +8386.1ns (+413.5%) | [+8357, +9463]ns | [10392, 12679] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 2016ns | +414.6% |
| 1 | 2 | 2018ns | +414.1% |
| 1 | 3 | 2472ns | +418.9% |
| 1 | 4 | 2027ns | +525.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.402 | moderate- |
| bitpack-footprint-packed-naive | 0.234 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 35.9ns | 3737.5ns | 1.0% |  |
| bitpack-footprint-packed-naive | 23.5ns | 12057.2ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 2001.1-7989.3 ns)
   2001.1 |########################################
   2300.5 |##############
   2600.0 |
   2899.4 |
   3198.8 |
   3498.2 |
   3797.6 |
   4097.0 |
   4396.4 |
   4695.8 |##
   4995.2 |##
   5294.6 |##
   5594.1 |
   5893.5 |
   6192.9 |
   6492.3 |####
   6791.7 |####
   7091.1 |##
   7390.5 |##
   7689.9 |
  (4 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 10372.1-15254.1 ns)
  10372.1 |#####################################
  10616.2 |##
  10860.3 |
  11104.4 |
  11348.5 |
  11592.6 |
  11836.7 |
  12080.8 |
  12324.9 |
  12569.0 |########################################
  12813.1 |##
  13057.2 |
  13301.3 |
  13545.4 |
  13789.5 |
  14033.6 |
  14277.7 |
  14521.8 |
  14765.9 |##
  15010.0 |##
  (5 below, 2 above range)

```

## Diagnostics

- **bitpack-footprint-packed**: CV=66.4% (high variance, measurements may be unstable)
- **bitpack-footprint-packed**: worst_20/best_20 = 4.0x (possible bimodal distribution)
