# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (8.56 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-simd at 2.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-simd beats baseline by 76% (significant)

bitpack-plan-simd is -6.54 us (76%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.2x slower than the field

bitpack-plan-naive (8.56 us) is 4.2x the fastest (2.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-plan-simd, bitpack-plan-windowed) are a dead heat (<1%)

bitpack-plan-simd (2.01 us) and bitpack-plan-windowed (2.02 us) differ by 0.22%, inside the noise, even though the wider field spreads 324.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-plan-naive shows alternating (throttle bounce) (autocorr -0.77)

bitpack-plan-naive's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} vs {bitpack-plan-naive} (243% apart)

The field splits into a fast tier {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 243% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.2x the fastest

Fastest bitpack-plan-simd (2.01 us) to slowest bitpack-plan-naive (8.56 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-plan-windowed is inconsistent: worst-20% is 2.8x its best-20%

bitpack-plan-windowed's best 20% of batches run at 2.02 us but its worst 20% at 5.55 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-plan-simd** at 2013.9 ns median (-76.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.25x (fastest 2013.9 ns, slowest 8556.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112822771 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 8624ns | 8622ns | 8612ns | 8622ns | 8641ns | base |
| bitpack-plan-native | 2668ns | 2548ns | 2547ns | 2551ns | 3141ns | -69.06% |
| bitpack-plan-simd | 2664ns | 2067ns | 2032ns | 2065ns | 5095ns | -69.10% |
| bitpack-plan-windowed | 2816ns | 2070ns | 2069ns | 2096ns | 5724ns | -67.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 8556ns | 8549ns | 8560ns | base | 1.915 |
| bitpack-plan-native | 2608ns | 2496ns | 3052ns | -69.51% | 6.281 |
| bitpack-plan-simd | 2590ns | 1978ns | 4938ns | -69.73% | 6.326 |
| bitpack-plan-windowed | 2738ns | 2018ns | 5552ns | -68.00% | 5.983 |

## Performance model

- Peak throughput: **8.285 Gops/s** (bitpack-plan-simd; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.915 | 23.1% |
| bitpack-plan-native | 6.564 | 79.2% |
| bitpack-plan-simd | 8.135 | 98.2% |
| bitpack-plan-windowed | 8.118 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 8624ns | 8624ns | base |
| bitpack-plan-native | 2668ns | 2668ns | -69.06% |
| bitpack-plan-simd | 2664ns | 2664ns | -69.10% |
| bitpack-plan-windowed | 2816ns | 2816ns | -67.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 8556ns | base | --- | [8555, 8558] | --- | --- | --- | --- |
| bitpack-plan-native | 2496ns | -6058.8ns (-70.8%) | [-6059, -6052]ns | [2496, 2496] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 2014ns | -6540.7ns (-76.4%) | [-6551, -6537]ns | [2006, 2017] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 2018ns | -6536.0ns (-76.4%) | [-6537, -6528]ns | [2018, 2023] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 8555ns | -70.8% | -76.4% | -76.4% |
| 1 | 2 | 8558ns | -70.8% | -76.8% | -76.4% |
| 1 | 3 | 8555ns | -70.8% | -76.4% | -76.4% |
| 1 | 4 | 8558ns | -70.8% | -76.5% | -76.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | -0.767 | HIGH- (thermal bounce) |
| bitpack-plan-native | -0.382 | moderate- |
| bitpack-plan-simd | -0.515 | HIGH- (thermal bounce) |
| bitpack-plan-windowed | -0.450 | moderate- |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 19.8ns | 8555.6ns | 0.2% |  |
| bitpack-plan-native | 20.2ns | 2608.5ns | 0.8% |  |
| bitpack-plan-simd | 25.1ns | 2589.8ns | 1.0% |  |
| bitpack-plan-windowed | 26.2ns | 2738.2ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 8549.5-8559.7 ns)
   8549.5 |
   8550.0 |
   8550.5 |
   8551.0 |
   8551.5 |
   8552.0 |
   8552.6 |
   8553.1 |
   8553.6 |
   8554.1 |######
   8554.6 |########################################
   8555.1 |#################################
   8555.6 |####################
   8556.1 |#############
   8556.6 |######
   8557.2 |#################################
   8557.7 |##########################
   8558.2 |
   8558.7 |####################
   8559.2 |######
  (5 below, 4 above range)

bitpack-plan-native (n=40, range 2495.9-3052.3 ns)
   2495.9 |########################################
   2523.7 |
   2551.5 |
   2579.3 |
   2607.2 |
   2635.0 |
   2662.8 |
   2690.6 |
   2718.4 |
   2746.3 |
   2774.1 |
   2801.9 |
   2829.7 |
   2857.6 |
   2885.4 |
   2913.2 |
   2941.0 |
   2968.8 |
   2996.7 |
   3024.5 |
  (3 below, 2 above range)

bitpack-plan-simd (n=40, range 1977.6-4938.2 ns)
   1977.6 |########################################
   2125.6 |
   2273.7 |#
   2421.7 |
   2569.7 |
   2717.8 |
   2865.8 |
   3013.8 |#
   3161.8 |
   3309.9 |
   3457.9 |
   3605.9 |
   3753.9 |
   3902.0 |
   4050.0 |
   4198.0 |#
   4346.1 |
   4494.1 |
   4642.1 |
   4790.1 |
  (3 below, 5 above range)

bitpack-plan-windowed (n=40, range 2017.5-5552.2 ns)
   2017.5 |########################################
   2194.3 |
   2371.0 |#
   2547.7 |
   2724.5 |
   2901.2 |
   3077.9 |
   3254.7 |
   3431.4 |
   3608.1 |#
   3784.9 |
   3961.6 |
   4138.3 |
   4315.1 |
   4491.8 |
   4668.5 |
   4845.3 |
   5022.0 |#
   5198.7 |
   5375.5 |#
  (4 below, 5 above range)

```

## Diagnostics

- **bitpack-plan-native**: CV=21.5% (high variance, measurements may be unstable)
- **bitpack-plan-simd**: CV=51.7% (high variance, measurements may be unstable)
- **bitpack-plan-windowed**: CV=53.2% (high variance, measurements may be unstable)
