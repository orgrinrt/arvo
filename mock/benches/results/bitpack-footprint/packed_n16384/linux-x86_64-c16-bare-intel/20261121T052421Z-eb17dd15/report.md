# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 372% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (2.45 us) leads bitpack-footprint-packed-naive (11.55 us) by 372%, a clear separation rather than a photo finish. CV 101.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed is fastest but the noisiest (CV 101.8%)

bitpack-footprint-packed wins on median (2.45 us) yet has the highest variance (CV 101.8%), while bitpack-footprint-packed-naive is the steadiest (CV 10.0%, 11.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (2.45 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.7x the fastest

Fastest bitpack-footprint-packed (2.45 us) to slowest bitpack-footprint-packed-naive (11.55 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-packed is inconsistent: worst-20% is 4.0x its best-20%

bitpack-footprint-packed's best 20% of batches run at 2.06 us but its worst 20% at 8.15 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 2445.6 ns median
- 1 variant significantly slower than baseline
- Spread: 4.72x (fastest 2445.6 ns, slowest 11545.4 ns)

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
| Counter frequency | 2112746100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 4064ns | 2508ns | 2111ns | 3264ns | 8415ns | base |
| bitpack-footprint-packed-naive | 11586ns | 11629ns | 10419ns | 11585ns | 12758ns | +185.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 3951ns | 2059ns | 8150ns | base | 4.147 |
| bitpack-footprint-packed-naive | 11523ns | 10366ns | 12682ns | +191.66% | 1.422 |

## Performance model

- Peak throughput: **7.959 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 6.700 | 84.2% |
| bitpack-footprint-packed-naive | 1.419 | 17.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 4064ns | 4064ns | base |
| bitpack-footprint-packed-naive | 11586ns | 11586ns | +185.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 2446ns | base | --- | [2444, 2458] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 11545ns | +7922.6ns (+324.0%) | [+7911, +8794]ns | [10369, 12671] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 2445ns | +324.0% |
| 1 | 2 | 2458ns | +321.9% |
| 1 | 3 | 2451ns | +417.0% |
| 1 | 4 | 2005ns | +531.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.078 | ok |
| bitpack-footprint-packed-naive | 0.250 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 37.6ns | 3950.8ns | 1.0% |  |
| bitpack-footprint-packed-naive | 21.4ns | 11522.7ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 2058.6-8150.1 ns)
   2058.6 |
   2363.1 |########################################
   2667.7 |
   2972.3 |##
   3276.9 |
   3581.5 |
   3886.0 |
   4190.6 |
   4495.2 |
   4799.8 |
   5104.4 |
   5408.9 |
   5713.5 |
   6018.1 |##
   6322.7 |##
   6627.3 |
   6931.8 |##
   7236.4 |##
   7541.0 |########
   7845.6 |##
  (7 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 10366.0-12682.1 ns)
  10366.0 |########################################
  10481.8 |
  10597.6 |
  10713.4 |
  10829.2 |
  10945.0 |
  11060.8 |
  11176.6 |
  11292.4 |
  11408.2 |
  11524.0 |
  11639.8 |
  11755.6 |
  11871.4 |
  11987.2 |
  12103.0 |
  12218.8 |
  12334.7 |
  12450.5 |
  12566.3 |########################################
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-footprint-packed**: CV=63.0% (high variance, measurements may be unstable)
- **bitpack-footprint-packed**: worst_20/best_20 = 4.0x (possible bimodal distribution)
