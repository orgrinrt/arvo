# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 392% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (548.24 us) leads bitpack-footprint-packed-naive (2.70 ms) by 392%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed shows alternating (throttle bounce) (autocorr -0.61)

bitpack-footprint-packed's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (548.24 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.9x the fastest

Fastest bitpack-footprint-packed (548.24 us) to slowest bitpack-footprint-packed-naive (2.70 ms): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 548237.9 ns median
- 1 variant significantly slower than baseline
- Spread: 4.92x (fastest 548237.9 ns, slowest 2698120.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 547133ns | 548586ns | 517461ns | 548951ns | 571351ns | base |
| bitpack-footprint-packed-naive | 2710688ns | 2698371ns | 2695752ns | 2700858ns | 2755117ns | +395.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 546885ns | 517288ns | 571111ns | base | 7.669 |
| bitpack-footprint-packed-naive | 2710394ns | 2695463ns | 2754705ns | +395.61% | 1.547 |

## Performance model

- Peak throughput: **8.108 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.651 | 94.4% |
| bitpack-footprint-packed-naive | 1.555 | 19.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 547133ns | 547133ns | base |
| bitpack-footprint-packed-naive | 2710688ns | 2710688ns | +395.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 548238ns | base | --- | [531111, 566195] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 2698121ns | +2165855.4ns (+395.1%) | [+2132620, +2167239]ns | [2697289, 2702770] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 516178ns | +425.3% |
| 1 | 2 | 565933ns | +376.7% |
| 1 | 3 | 531071ns | +407.9% |
| 1 | 4 | 567868ns | +374.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.612 | HIGH- (thermal bounce) |
| bitpack-footprint-packed-naive | -0.053 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 48.7ns | 546885.4ns | 0.0% |  |
| bitpack-footprint-packed-naive | 85.4ns | 2710394.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 517287.8-571110.9 ns)
  517287.8 |
  519978.9 |###
  522670.1 |###
  525361.2 |
  528052.4 |###
  530743.5 |####################################
  533434.7 |
  536125.9 |
  538817.0 |
  541508.2 |
  544199.3 |
  546890.5 |
  549581.6 |
  552272.8 |
  554963.9 |
  557655.1 |
  560346.3 |
  563037.4 |################
  565728.6 |########################################
  568419.7 |######
  (6 below, 1 above range)

bitpack-footprint-packed-naive (n=40, range 2695463.2-2754704.5 ns)
  2695463.2 |########################################
  2698425.3 |########
  2701387.4 |#############
  2704349.4 |
  2707311.5 |##########
  2710273.6 |#####
  2713235.6 |##
  2716197.7 |
  2719159.8 |##
  2722121.8 |
  2725083.9 |
  2728046.0 |##
  2731008.0 |
  2733970.1 |##
  2736932.1 |
  2739894.2 |
  2742856.3 |
  2745818.3 |
  2748780.4 |
  2751742.5 |
  (5 below, 2 above range)

```
