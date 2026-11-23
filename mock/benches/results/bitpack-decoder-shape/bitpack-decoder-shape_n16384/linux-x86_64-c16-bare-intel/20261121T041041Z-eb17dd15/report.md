# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (8.55 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-windowed at 2.00 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-windowed beats baseline by 77% (significant)

bitpack-plan-windowed is -6.55 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.3x slower than the field

bitpack-plan-naive (8.55 us) is 4.3x the fastest (2.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-plan-windowed is fastest but the noisiest (CV 73.7%)

bitpack-plan-windowed wins on median (2.00 us) yet has the highest variance (CV 73.7%), while bitpack-plan-naive is the steadiest (CV 1.0%, 8.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (bitpack-plan-windowed, bitpack-plan-simd) are a dead heat (<1%)

bitpack-plan-windowed (2.00 us) and bitpack-plan-simd (2.00 us) differ by 0.00%, inside the noise, even though the wider field spreads 327.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-plan-windowed, bitpack-plan-simd, bitpack-plan-native} vs {bitpack-plan-naive} (243% apart)

The field splits into a fast tier {bitpack-plan-windowed, bitpack-plan-simd, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 243% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest bitpack-plan-windowed (2.00 us) to slowest bitpack-plan-naive (8.55 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-plan-windowed is inconsistent: worst-20% is 2.8x its best-20%

bitpack-plan-windowed's best 20% of batches run at 2.00 us but its worst 20% at 5.61 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-plan-windowed** at 2000.9 ns median (-76.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.28x (fastest 2000.9 ns, slowest 8555.0 ns)

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
| Counter frequency | 2112800587 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 8626ns | 8610ns | 8601ns | 8611ns | 8697ns | base |
| bitpack-plan-native | 3001ns | 2548ns | 2547ns | 2550ns | 4809ns | -65.21% |
| bitpack-plan-simd | 2661ns | 2052ns | 2051ns | 2056ns | 5085ns | -69.15% |
| bitpack-plan-windowed | 2858ns | 2052ns | 2051ns | 2155ns | 5777ns | -66.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 8567ns | 8546ns | 8627ns | base | 1.912 |
| bitpack-plan-native | 2936ns | 2496ns | 4688ns | -65.73% | 5.580 |
| bitpack-plan-simd | 2585ns | 2000ns | 4913ns | -69.82% | 6.337 |
| bitpack-plan-windowed | 2781ns | 2000ns | 5606ns | -67.54% | 5.892 |

## Performance model

- Peak throughput: **8.192 Gops/s** (bitpack-plan-simd; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.915 | 23.4% |
| bitpack-plan-native | 6.564 | 80.1% |
| bitpack-plan-simd | 8.188 | 99.9% |
| bitpack-plan-windowed | 8.188 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 8626ns | 8626ns | base |
| bitpack-plan-native | 3001ns | 3001ns | -65.21% |
| bitpack-plan-simd | 2661ns | 2661ns | -69.15% |
| bitpack-plan-windowed | 2858ns | 2858ns | -66.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 8555ns | base | --- | [8554, 8556] | --- | --- | --- | --- |
| bitpack-plan-native | 2496ns | -6057.4ns (-70.8%) | [-6060, -6049]ns | [2496, 2501] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 2001ns | -6549.1ns (-76.6%) | [-6553, -6543]ns | [2001, 2011] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 2001ns | -6552.3ns (-76.6%) | [-6554, -6546]ns | [2001, 2011] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 8556ns | -70.8% | -76.6% | -76.6% |
| 1 | 2 | 8557ns | -70.8% | -76.6% | -76.6% |
| 1 | 3 | 8555ns | -70.8% | -76.5% | -76.6% |
| 1 | 4 | 8552ns | -70.8% | -76.6% | -76.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | 0.181 | ok |
| bitpack-plan-native | -0.078 | ok |
| bitpack-plan-simd | -0.428 | moderate- |
| bitpack-plan-windowed | -0.398 | moderate- |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 19.6ns | 8567.5ns | 0.2% |  |
| bitpack-plan-native | 22.6ns | 2936.1ns | 0.8% |  |
| bitpack-plan-simd | 25.0ns | 2585.4ns | 1.0% |  |
| bitpack-plan-windowed | 26.7ns | 2780.8ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 8546.0-8626.9 ns)
   8546.0 |##
   8550.0 |################
   8554.1 |########################################
   8558.1 |########
   8562.1 |
   8566.2 |##
   8570.2 |##
   8574.3 |
   8578.3 |
   8582.4 |
   8586.4 |
   8590.5 |
   8594.5 |
   8598.6 |
   8602.6 |
   8606.6 |
   8610.7 |
   8614.7 |
   8618.8 |
   8622.8 |
  (5 below, 1 above range)

bitpack-plan-native (n=40, range 2496.0-4688.4 ns)
   2496.0 |########################################
   2605.6 |
   2715.2 |
   2824.8 |
   2934.5 |
   3044.1 |#
   3153.7 |
   3263.3 |
   3372.9 |
   3482.6 |
   3592.2 |
   3701.8 |
   3811.4 |
   3921.0 |
   4030.7 |
   4140.3 |
   4249.9 |
   4359.5 |
   4469.1 |
   4578.7 |
  (2 below, 4 above range)

bitpack-plan-simd (n=40, range 1999.9-4913.2 ns)
   1999.9 |########################################
   2145.6 |
   2291.2 |#
   2436.9 |
   2582.5 |
   2728.2 |#
   2873.9 |
   3019.5 |
   3165.2 |
   3310.9 |
   3456.5 |
   3602.2 |
   3747.9 |
   3893.5 |
   4039.2 |
   4184.9 |#
   4330.5 |
   4476.2 |
   4621.9 |
   4767.5 |
  (2 below, 5 above range)

bitpack-plan-windowed (n=40, range 2000.5-5606.1 ns)
   2000.5 |########################################
   2180.7 |
   2361.0 |
   2541.3 |
   2721.6 |
   2901.9 |
   3082.2 |
   3262.4 |
   3442.7 |
   3623.0 |
   3803.3 |
   3983.6 |
   4163.9 |##
   4344.1 |
   4524.4 |
   4704.7 |
   4885.0 |
   5065.3 |
   5245.6 |
   5425.8 |####
  (3 below, 4 above range)

```

## Diagnostics

- **bitpack-plan-native**: CV=43.3% (high variance, measurements may be unstable)
- **bitpack-plan-simd**: CV=52.1% (high variance, measurements may be unstable)
- **bitpack-plan-windowed**: CV=53.0% (high variance, measurements may be unstable)
