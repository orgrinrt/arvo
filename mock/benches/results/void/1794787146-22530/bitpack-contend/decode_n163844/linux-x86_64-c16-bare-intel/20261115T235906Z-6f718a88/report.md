# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-control dominates: 45% faster than the next best (bitpack-contend-pipe2)

bitpack-contend-d16-control (1.94 us) leads bitpack-contend-pipe2 (2.82 us) by 45%, a clear separation rather than a photo finish. CV 68.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d16-control is fastest but the noisiest (CV 68.0%)

bitpack-contend-d16-control wins on median (1.94 us) yet has the highest variance (CV 68.0%), while bitpack-contend-packed-simd is the steadiest (CV 31.1%, 2.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (965 ns) is smaller than the fastest variant's own run-to-run std-dev (1.32 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {bitpack-contend-d16-control} vs {bitpack-contend-pipe2, bitpack-contend-d16, bitpack-contend-pipe4, bitpack-contend-packed-simd} (45% apart)

The field splits into a fast tier {bitpack-contend-d16-control} and a slow tier {bitpack-contend-pipe2, bitpack-contend-d16, bitpack-contend-pipe4, bitpack-contend-packed-simd} with a 45% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-pipe4 is inconsistent: worst-20% is 4.0x its best-20%

bitpack-contend-pipe4's best 20% of batches run at 1.01 us but its worst 20% at 4.00 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d16-control** at 1941.6 ns median (-32.5% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.50x (fastest 1941.6 ns, slowest 2906.6 ns)

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
| bitpack-contend-d16 | 2662ns | 3028ns | 1280ns | 2563ns | 4341ns | base |
| bitpack-contend-d16-control | 2749ns | 2052ns | 1351ns | 2622ns | 4526ns | +3.25% |
| bitpack-contend-packed-simd | 2908ns | 3075ns | 1178ns | 3113ns | 4020ns | +9.23% |
| bitpack-contend-pipe2 | 2854ns | 3010ns | 1114ns | 2952ns | 4300ns | +7.21% |
| bitpack-contend-pipe4 | 2685ns | 3090ns | 1065ns | 2707ns | 4241ns | +0.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 2522ns | 1224ns | 4076ns | base | 6.496 |
| bitpack-contend-d16-control | 2605ns | 1290ns | 4258ns | +3.30% | 6.288 |
| bitpack-contend-packed-simd | 2732ns | 1122ns | 3790ns | +8.30% | 5.998 |
| bitpack-contend-pipe2 | 2667ns | 1051ns | 4005ns | +5.75% | 6.142 |
| bitpack-contend-pipe4 | 2515ns | 1010ns | 3996ns | -0.28% | 6.514 |

## Performance model

- Peak throughput: **16.217 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.694 | 35.1% |
| bitpack-contend-d16-control | 8.438 | 52.0% |
| bitpack-contend-packed-simd | 5.637 | 34.8% |
| bitpack-contend-pipe2 | 5.815 | 35.9% |
| bitpack-contend-pipe4 | 5.688 | 35.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 2662ns | 2662ns | base |
| bitpack-contend-d16-control | 2749ns | 2749ns | +3.25% |
| bitpack-contend-packed-simd | 2908ns | 2908ns | +9.23% |
| bitpack-contend-pipe2 | 2854ns | 2854ns | +7.21% |
| bitpack-contend-pipe4 | 2685ns | 2685ns | +0.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 2878ns | base | --- | [1264, 3365] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1942ns | +76.6ns (+2.7%) | [+68, +169]ns | [1334, 3705] | YES | 0.0044 | 0.0022 | 0 |
| bitpack-contend-packed-simd | 2907ns | -176.3ns (-6.1%) | [-431, -43]ns | [2704, 3135] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-contend-pipe2 | 2818ns | -198.3ns (-6.9%) | [-248, -23]ns | [2613, 3152] | YES | 0.0221 | 0.0166 | 0 |
| bitpack-contend-pipe4 | 2880ns | -288.9ns (-10.0%) | [-431, -209]ns | [2115, 2950] | YES | 0.0027 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 3305ns | +10.1% | -12.6% | -9.5% | -17.5% |
| 1 | 2 | 3397ns | -30.9% | -19.6% | -18.1% | -13.2% |
| 1 | 3 | 1227ns | +6.0% | +140.4% | +201.4% | +131.6% |
| 1 | 4 | 1248ns | +5.1% | +162.5% | -16.2% | -19.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.236 | moderate+ |
| bitpack-contend-d16-control | 0.228 | moderate+ |
| bitpack-contend-packed-simd | 0.102 | ok |
| bitpack-contend-pipe2 | -0.388 | moderate- |
| bitpack-contend-pipe4 | -0.070 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-packed-simd**: won 2/4, lost 2/4
- **bitpack-contend-pipe2**: won 3/4, lost 1/4
- **bitpack-contend-pipe4**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 47.7ns | 2522.3ns | 1.9% |  |
| bitpack-contend-d16-control | 50.6ns | 2605.5ns | 1.9% |  |
| bitpack-contend-packed-simd | 61.6ns | 2731.6ns | 2.3% |  |
| bitpack-contend-pipe2 | 65.3ns | 2667.4ns | 2.4% |  |
| bitpack-contend-pipe4 | 59.5ns | 2515.3ns | 2.4% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1224.2-4076.4 ns)
   1224.2 |########################################
   1366.8 |###
   1509.4 |
   1652.0 |
   1794.6 |
   1937.2 |
   2079.9 |###
   2222.5 |
   2365.1 |###
   2507.7 |
   2650.3 |###
   2792.9 |
   2935.5 |###
   3078.2 |###
   3220.8 |#############
   3363.4 |#######################
   3506.0 |######
   3648.6 |###
   3791.2 |
   3933.8 |
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 1290.3-4257.7 ns)
   1290.3 |########################################
   1438.7 |
   1587.0 |
   1735.4 |
   1883.8 |
   2032.1 |
   2180.5 |
   2328.9 |
   2477.2 |##
   2625.6 |
   2774.0 |
   2922.4 |
   3070.7 |
   3219.1 |##
   3367.5 |##
   3515.8 |##
   3664.2 |######
   3812.6 |######
   3960.9 |####
   4109.3 |######
  (2 below, 5 above range)

bitpack-contend-packed-simd (n=40, range 1122.0-3789.6 ns)
   1122.0 |############################
   1255.4 |
   1388.8 |
   1522.2 |
   1655.5 |
   1788.9 |
   1922.3 |
   2055.7 |
   2189.1 |#####
   2322.4 |
   2455.8 |#####
   2589.2 |############################
   2722.6 |#################
   2856.0 |########################################
   2989.3 |#####
   3122.7 |#################
   3256.1 |#####
   3389.5 |###########
   3522.9 |#################
   3656.3 |#####
  (3 below, 4 above range)

bitpack-contend-pipe2 (n=40, range 1050.7-4005.3 ns)
   1050.7 |#################################
   1198.4 |
   1346.2 |
   1493.9 |######
   1641.6 |
   1789.4 |
   1937.1 |
   2084.8 |
   2232.6 |######
   2380.3 |#############
   2528.0 |
   2675.7 |########################################
   2823.5 |####################
   2971.2 |####################
   3118.9 |######
   3266.7 |######
   3414.4 |######
   3562.1 |##########################
   3709.9 |######
   3857.6 |####################
  (5 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 1010.3-3996.2 ns)
   1010.3 |##############################
   1159.6 |
   1308.9 |
   1458.2 |#####
   1607.5 |
   1756.8 |
   1906.1 |#####
   2055.4 |#####
   2204.7 |
   2354.0 |
   2503.3 |
   2652.6 |####################
   2801.9 |########################################
   2951.2 |##########
   3100.4 |#####
   3249.7 |
   3399.0 |#####
   3548.3 |###############
   3697.6 |#####
   3846.9 |#####
  (6 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=47.3% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=50.6% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=33.1% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bitpack-contend-pipe2**: CV=40.3% (high variance, measurements may be unstable)
- **bitpack-contend-pipe2**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bitpack-contend-pipe4**: CV=44.9% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: worst_20/best_20 = 4.0x (possible bimodal distribution)
