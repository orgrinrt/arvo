# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-headroom dominates: 22% faster than the next best (warm-container-plusone)

warm-container-headroom (1.89 us) leads warm-container-plusone (2.30 us) by 22%, a clear separation rather than a photo finish. CV 114.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-headroom is fastest but the noisiest (CV 114.7%)

warm-container-headroom wins on median (1.89 us) yet has the highest variance (CV 114.7%), while warm-container-lanes-deferred is the steadiest (CV 12.5%, 2.45 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (913 ns) is smaller than the fastest variant's own run-to-run std-dev (2.17 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (warm-container-headroom)

The baseline warm-container-headroom is the fastest (1.89 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### warm-container-plusone is inconsistent: worst-20% is 4.0x its best-20%

warm-container-plusone's best 20% of batches run at 1.88 us but its worst 20% at 7.53 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-container-headroom) is the fastest** at 1888.0 ns median
- 3 variants significantly slower than baseline
- Spread: 1.48x (fastest 1888.0 ns, slowest 2800.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112763739 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3818ns | 1955ns | 1945ns | 3310ns | 7219ns | base |
| warm-container-kernel | 2763ns | 2698ns | 1997ns | 2739ns | 3598ns | -27.64% |
| warm-container-lanes-deferred | 2700ns | 2642ns | 2254ns | 2671ns | 3234ns | -29.28% |
| warm-container-minimum | 3944ns | 2364ns | 2317ns | 2860ns | 8823ns | +3.29% |
| warm-container-native | 3911ns | 2864ns | 2350ns | 2862ns | 8618ns | +2.42% |
| warm-container-plusone | 3907ns | 2388ns | 1947ns | 3243ns | 7856ns | +2.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3676ns | 1878ns | 6918ns | base | 4.456 |
| warm-container-kernel | 2558ns | 1855ns | 3309ns | -30.43% | 6.406 |
| warm-container-lanes-deferred | 2491ns | 2094ns | 2940ns | -32.25% | 6.578 |
| warm-container-minimum | 3846ns | 2265ns | 8571ns | +4.60% | 4.261 |
| warm-container-native | 3812ns | 2298ns | 8365ns | +3.69% | 4.298 |
| warm-container-plusone | 3762ns | 1881ns | 7533ns | +2.32% | 4.355 |

## Performance model

- Peak throughput: **8.832 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.678 | 98.3% |
| warm-container-kernel | 6.537 | 74.0% |
| warm-container-lanes-deferred | 6.679 | 75.6% |
| warm-container-minimum | 7.084 | 80.2% |
| warm-container-native | 5.850 | 66.2% |
| warm-container-plusone | 7.109 | 80.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3818ns | 3818ns | base |
| warm-container-kernel | 2763ns | 2763ns | -27.64% |
| warm-container-lanes-deferred | 2700ns | 2700ns | -29.28% |
| warm-container-minimum | 3944ns | 3944ns | +3.29% |
| warm-container-native | 3911ns | 3911ns | +2.42% |
| warm-container-plusone | 3907ns | 3907ns | +2.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1888ns | base | --- | [1878, 5349] | --- | --- | --- | --- |
| warm-container-kernel | 2506ns | no significant difference | [-2675, +404]ns | [2421, 2674] | no | 1.0000 | 1.0000 | 0 |
| warm-container-lanes-deferred | 2453ns | no significant difference | [-2650, +444]ns | [2416, 2588] | no | 0.7948 | 0.6358 | 0 |
| warm-container-minimum | 2313ns | +427.1ns (+22.6%) | [+389, +887]ns | [2303, 2794] | YES | 0.0027 | 0.0007 | 0 |
| warm-container-native | 2801ns | +672.5ns (+35.6%) | [+407, +916]ns | [2787, 2813] | YES | 0.0107 | 0.0064 | 0 |
| warm-container-plusone | 2305ns | +412.7ns (+21.9%) | [+24, +423]ns | [2297, 3373] | YES | 0.0027 | 0.0011 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1883ns | +33.5% | +48.4% | +48.9% | +21.9% | +22.5% |
| 1 | 2 | 3992ns | -34.9% | -38.6% | -42.3% | -29.7% | -52.5% |
| 1 | 3 | 2861ns | -20.9% | -25.3% | -19.3% | -1.9% | -19.4% |
| 1 | 4 | 1879ns | +64.2% | +30.7% | +22.4% | +49.3% | +49.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.302 | moderate- |
| warm-container-kernel | -0.446 | moderate- |
| warm-container-lanes-deferred | 0.007 | ok |
| warm-container-minimum | -0.084 | ok |
| warm-container-native | -0.086 | ok |
| warm-container-plusone | 0.021 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 2/4, lost 2/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 37.8ns | 3676.4ns | 1.0% |  |
| warm-container-kernel | 74.8ns | 2557.6ns | 2.9% |  |
| warm-container-lanes-deferred | 74.5ns | 2490.8ns | 3.0% |  |
| warm-container-minimum | 32.4ns | 3845.5ns | 0.8% |  |
| warm-container-native | 32.1ns | 3812.3ns | 0.8% |  |
| warm-container-plusone | 38.4ns | 3761.7ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1878.3-6918.4 ns)
   1878.3 |########################################
   2130.3 |
   2382.3 |
   2634.3 |##
   2886.3 |
   3138.3 |
   3390.3 |
   3642.3 |##
   3894.3 |
   4146.3 |##
   4398.3 |
   4650.3 |
   4902.3 |
   5154.3 |####
   5406.3 |####
   5658.3 |####
   5910.4 |##
   6162.4 |######
   6414.4 |
   6666.4 |####
  (4 below, 3 above range)

warm-container-kernel (n=40, range 1855.0-3309.4 ns)
   1855.0 |
   1927.7 |
   2000.4 |
   2073.1 |
   2145.9 |#####
   2218.6 |########################################
   2291.3 |#################
   2364.0 |#####
   2436.7 |##################################
   2509.5 |#################
   2582.2 |
   2654.9 |##################################
   2727.6 |#####
   2800.3 |
   2873.1 |###########
   2945.8 |#####
   3018.5 |###########
   3091.2 |
   3163.9 |
   3236.7 |#####
  (2 below, 4 above range)

warm-container-lanes-deferred (n=40, range 2093.7-2939.7 ns)
   2093.7 |########
   2136.0 |########
   2178.3 |
   2220.6 |
   2262.9 |##########################
   2305.2 |
   2347.5 |####
   2389.8 |
   2432.1 |########################################
   2474.4 |########
   2516.7 |####
   2559.0 |
   2601.3 |
   2643.6 |######################
   2685.9 |
   2728.2 |#############
   2770.5 |####
   2812.8 |
   2855.1 |####
   2897.4 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 2265.3-8570.5 ns)
   2265.3 |########################################
   2580.6 |##################
   2895.8 |
   3211.1 |
   3526.3 |##
   3841.6 |
   4156.9 |
   4472.1 |
   4787.4 |
   5102.6 |##
   5417.9 |
   5733.2 |##
   6048.4 |
   6363.7 |
   6678.9 |
   6994.2 |
   7309.5 |
   7624.7 |##
   7940.0 |##
   8255.3 |####
  (4 below, 4 above range)

warm-container-native (n=40, range 2298.0-8364.9 ns)
   2298.0 |###
   2601.3 |########################################
   2904.7 |
   3208.0 |
   3511.4 |
   3814.7 |#
   4118.0 |
   4421.4 |
   4724.7 |
   5028.1 |#
   5331.4 |
   5634.8 |
   5938.1 |
   6241.5 |
   6544.8 |
   6848.2 |
   7151.5 |
   7454.9 |#
   7758.2 |
   8061.5 |
  (6 below, 5 above range)

warm-container-plusone (n=40, range 1880.8-7532.7 ns)
   1880.8 |##################
   2163.4 |########################################
   2446.0 |
   2728.6 |
   3011.2 |######
   3293.8 |
   3576.4 |######
   3858.9 |
   4141.5 |
   4424.1 |
   4706.7 |
   4989.3 |
   5271.9 |
   5554.5 |
   5837.1 |###
   6119.7 |###
   6402.3 |
   6684.9 |############
   6967.5 |###
   7250.1 |###
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=58.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-kernel**: CV=22.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=64.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-native**: CV=66.5% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=61.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
