# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 368% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (2.47 us) leads bitpack-footprint-packed-naive (11.55 us) by 368%, a clear separation rather than a photo finish. CV 78.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed is fastest but the noisiest (CV 78.1%)

bitpack-footprint-packed wins on median (2.47 us) yet has the highest variance (CV 78.1%), while bitpack-footprint-packed-naive is the steadiest (CV 10.0%, 11.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (2.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.7x the fastest

Fastest bitpack-footprint-packed (2.47 us) to slowest bitpack-footprint-packed-naive (11.55 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-packed is inconsistent: worst-20% is 3.0x its best-20%

bitpack-footprint-packed's best 20% of batches run at 2.31 us but its worst 20% at 6.96 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 2466.8 ns median
- 1 variant significantly slower than baseline
- Spread: 4.68x (fastest 2466.8 ns, slowest 11548.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 3494ns | 2530ns | 2377ns | 2619ns | 7235ns | base |
| bitpack-footprint-packed-naive | 11597ns | 11619ns | 10425ns | 11598ns | 12767ns | +231.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 3383ns | 2315ns | 6964ns | base | 4.844 |
| bitpack-footprint-packed-naive | 11528ns | 10368ns | 12683ns | +240.78% | 1.421 |

## Performance model

- Peak throughput: **7.078 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 6.642 | 93.8% |
| bitpack-footprint-packed-naive | 1.419 | 20.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 3494ns | 3494ns | base |
| bitpack-footprint-packed-naive | 11597ns | 11597ns | +231.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 2467ns | base | --- | [2462, 2474] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 11548ns | +7953.2ns (+322.4%) | [+7910, +9874]ns | [10372, 12675] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 2466ns | +413.9% |
| 1 | 2 | 2454ns | +322.6% |
| 1 | 3 | 2467ns | +320.3% |
| 1 | 4 | 2800ns | +352.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.062 | ok |
| bitpack-footprint-packed-naive | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 35.6ns | 3382.7ns | 1.1% |  |
| bitpack-footprint-packed-naive | 21.8ns | 11527.5ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 2314.7-6963.9 ns)
   2314.7 |########################################
   2547.1 |
   2779.6 |######
   3012.0 |#
   3244.5 |
   3477.0 |
   3709.4 |
   3941.9 |#
   4174.3 |
   4406.8 |
   4639.3 |
   4871.7 |
   5104.2 |
   5336.7 |
   5569.1 |
   5801.6 |#
   6034.0 |
   6266.5 |###
   6499.0 |
   6731.4 |
  (2 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 10367.9-12683.3 ns)
  10367.9 |########################################
  10483.6 |
  10599.4 |
  10715.2 |
  10831.0 |
  10946.7 |
  11062.5 |
  11178.3 |
  11294.0 |
  11409.8 |
  11525.6 |
  11641.4 |
  11757.1 |
  11872.9 |
  11988.7 |
  12104.4 |
  12220.2 |
  12336.0 |
  12451.7 |
  12567.5 |###############################
  (1 below, 5 above range)

```

## Diagnostics

- **bitpack-footprint-packed**: CV=57.0% (high variance, measurements may be unstable)
- **bitpack-footprint-packed**: worst_20/best_20 = 3.0x (possible bimodal distribution)
