# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (bitpack-contend-pipe4, bitpack-contend-pipe2) are a dead heat (<1%)

bitpack-contend-pipe4 (2.47 us) and bitpack-contend-pipe2 (2.48 us) differ by 0.28%, inside the noise, even though the wider field spreads 23.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-pipe2 shows alternating (throttle bounce) (autocorr -0.59)

bitpack-contend-pipe2's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (584 ns) is smaller than the fastest variant's own run-to-run std-dev (2.16 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-contend-pipe4 is inconsistent: worst-20% is 3.5x its best-20%

bitpack-contend-pipe4's best 20% of batches run at 2.02 us but its worst 20% at 7.05 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### bitpack-contend-d16-control's edge over baseline is significant but tiny (1 ns, 0.04%)

bitpack-contend-d16-control differs from baseline bitpack-contend-d16 by 1 ns (0.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 2468.4 ns median (-1.6% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.24x (fastest 2468.4 ns, slowest 3052.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112755232 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3951ns | 2560ns | 2548ns | 2954ns | 8344ns | base |
| bitpack-contend-d16-control | 4024ns | 3116ns | 2549ns | 3011ns | 8536ns | +1.84% |
| bitpack-contend-packed-simd | 4499ns | 2910ns | 2358ns | 4155ns | 7668ns | +13.86% |
| bitpack-contend-pipe2 | 3997ns | 2539ns | 2069ns | 3602ns | 7110ns | +1.16% |
| bitpack-contend-pipe4 | 3963ns | 2531ns | 2071ns | 3485ns | 7289ns | +0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 3861ns | 2497ns | 8127ns | base | 4.243 |
| bitpack-contend-d16-control | 3931ns | 2497ns | 8307ns | +1.80% | 4.168 |
| bitpack-contend-packed-simd | 4359ns | 2300ns | 7432ns | +12.90% | 3.758 |
| bitpack-contend-pipe2 | 3887ns | 2017ns | 6882ns | +0.67% | 4.215 |
| bitpack-contend-pipe4 | 3853ns | 2019ns | 7052ns | -0.20% | 4.252 |

## Performance model

- Peak throughput: **8.122 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.533 | 80.4% |
| bitpack-contend-d16-control | 5.367 | 66.1% |
| bitpack-contend-packed-simd | 5.858 | 72.1% |
| bitpack-contend-pipe2 | 6.619 | 81.5% |
| bitpack-contend-pipe4 | 6.637 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 3951ns | 3951ns | base |
| bitpack-contend-d16-control | 4024ns | 4024ns | +1.84% |
| bitpack-contend-packed-simd | 4499ns | 4499ns | +13.86% |
| bitpack-contend-pipe2 | 3997ns | 3997ns | +1.16% |
| bitpack-contend-pipe4 | 3963ns | 3963ns | +0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 2508ns | base | --- | [2497, 3052] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 3053ns | +1.0ns (+0.0%) | [+0, +13]ns | [2508, 3064] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-contend-packed-simd | 2797ns | -29.0ns (-1.2%) | [-237, -22]ns | [2480, 5950] | YES | 0.0086 | 0.0064 | 0 |
| bitpack-contend-pipe2 | 2475ns | -473.8ns (-18.9%) | [-483, -39]ns | [2465, 5203] | YES | 0.0007 | 0.0002 | 0 |
| bitpack-contend-pipe4 | 2468ns | -477.3ns (-19.0%) | [-478, -29]ns | [2029, 5489] | YES | 0.0014 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 2502ns | +0.1% | +171.9% | -1.2% | +26.0% |
| 1 | 2 | 2497ns | +0.0% | -0.7% | +32.8% | +53.7% |
| 1 | 3 | 3058ns | +0.0% | -5.5% | -16.9% | -33.8% |
| 1 | 4 | 2497ns | +22.6% | -0.9% | +7.3% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.424 | moderate- |
| bitpack-contend-d16-control | 0.247 | moderate+ |
| bitpack-contend-packed-simd | -0.142 | ok |
| bitpack-contend-pipe2 | -0.586 | HIGH- (thermal bounce) |
| bitpack-contend-pipe4 | -0.108 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 0/4, lost 2/4
- **bitpack-contend-packed-simd**: won 3/4, lost 1/4
- **bitpack-contend-pipe2**: won 2/4, lost 2/4
- **bitpack-contend-pipe4**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 30.8ns | 3861.3ns | 0.8% |  |
| bitpack-contend-d16-control | 30.9ns | 3930.8ns | 0.8% |  |
| bitpack-contend-packed-simd | 47.5ns | 4359.3ns | 1.1% |  |
| bitpack-contend-pipe2 | 37.6ns | 3887.0ns | 1.0% |  |
| bitpack-contend-pipe4 | 37.1ns | 3853.4ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 2496.9-8126.8 ns)
   2496.9 |########################################
   2778.4 |#########
   3059.9 |#####
   3341.4 |
   3622.9 |
   3904.4 |
   4185.9 |
   4467.3 |#
   4748.8 |
   5030.3 |
   5311.8 |#
   5593.3 |
   5874.8 |
   6156.3 |
   6437.8 |
   6719.3 |
   7000.8 |#
   7282.3 |#
   7563.8 |#
   7845.3 |#
  (1 below, 4 above range)

bitpack-contend-d16-control (n=40, range 2497.2-8307.1 ns)
   2497.2 |#################################
   2787.6 |########################################
   3078.1 |######
   3368.6 |###
   3659.1 |
   3949.6 |###
   4240.1 |
   4530.6 |
   4821.1 |###
   5111.6 |###
   5402.1 |
   5692.6 |
   5983.1 |
   6273.6 |
   6564.1 |
   6854.6 |
   7145.1 |
   7435.6 |
   7726.1 |###
   8016.6 |###
  (4 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 2299.8-7432.0 ns)
   2299.8 |########################################
   2556.4 |##################
   2813.0 |###
   3069.6 |
   3326.2 |
   3582.8 |
   3839.4 |
   4096.0 |
   4352.6 |
   4609.2 |
   4865.9 |
   5122.5 |
   5379.1 |###
   5635.7 |######
   5892.3 |###
   6148.9 |###
   6405.5 |
   6662.1 |###############
   6918.7 |######
   7175.3 |
  (3 below, 5 above range)

bitpack-contend-pipe2 (n=40, range 2017.3-6882.0 ns)
   2017.3 |##########################
   2260.5 |########################################
   2503.7 |###
   2747.0 |
   2990.2 |
   3233.5 |###
   3476.7 |###
   3719.9 |
   3963.2 |
   4206.4 |
   4449.6 |###
   4692.9 |
   4936.1 |
   5179.4 |
   5422.6 |
   5665.8 |######
   5909.1 |######
   6152.3 |#############
   6395.6 |
   6638.8 |##########
  (2 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 2019.0-7052.3 ns)
   2019.0 |########################################
   2270.7 |########################
   2522.3 |
   2774.0 |
   3025.7 |
   3277.3 |
   3529.0 |
   3780.7 |
   4032.3 |###
   4284.0 |
   4535.7 |
   4787.3 |
   5039.0 |
   5290.7 |###
   5542.3 |###
   5794.0 |######
   6045.7 |######
   6297.3 |#########
   6549.0 |######
   6800.7 |###
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=57.6% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=61.2% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=50.4% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **bitpack-contend-pipe2**: CV=52.1% (high variance, measurements may be unstable)
- **bitpack-contend-pipe2**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bitpack-contend-pipe4**: CV=56.0% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: worst_20/best_20 = 3.5x (possible bimodal distribution)
