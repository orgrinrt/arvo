# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d32 dominates: 29% faster than the next best (bitpack-carrier-packed)

bitpack-carrier-d32 (1.90 us) leads bitpack-carrier-packed (2.44 us) by 29%, a clear separation rather than a photo finish. CV 105.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-d32 beats baseline by 24% (significant)

bitpack-carrier-d32 is -611 ns (24%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d32 is fastest but the noisiest (CV 105.7%)

bitpack-carrier-d32 wins on median (1.90 us) yet has the highest variance (CV 105.7%), while bitpack-carrier-d64 is the steadiest (CV 43.9%, 3.73 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-carrier-packed shows alternating (throttle bounce) (autocorr -0.51)

bitpack-carrier-packed's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.83 us) is smaller than the fastest variant's own run-to-run std-dev (2.00 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-carrier-d64 is inconsistent: worst-20% is 3.9x its best-20%

bitpack-carrier-d64's best 20% of batches run at 1.34 us but its worst 20% at 5.27 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-carrier-d32** at 1895.8 ns median (-24.4% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.97x (fastest 1895.8 ns, slowest 3730.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112762950 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3724ns | 2558ns | 2547ns | 2750ns | 7826ns | base |
| bitpack-carrier-d16-control | 4998ns | 3126ns | 3113ns | 4286ns | 9017ns | +34.18% |
| bitpack-carrier-d32 | 3668ns | 1962ns | 1951ns | 3219ns | 6730ns | -1.53% |
| bitpack-carrier-d64 | 3439ns | 3914ns | 1402ns | 3397ns | 5600ns | -7.68% |
| bitpack-carrier-packed | 3550ns | 2509ns | 2051ns | 2704ns | 7589ns | -4.67% |
| bitpack-carrier-packed-simd | 3922ns | 2519ns | 2221ns | 3291ns | 7516ns | +5.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3640ns | 2496ns | 7620ns | base | 4.501 |
| bitpack-carrier-d16-control | 4886ns | 3051ns | 8779ns | +34.22% | 3.353 |
| bitpack-carrier-d32 | 3532ns | 1885ns | 6460ns | -2.96% | 4.638 |
| bitpack-carrier-d64 | 3263ns | 1336ns | 5273ns | -10.36% | 5.021 |
| bitpack-carrier-packed | 3450ns | 2000ns | 7362ns | -5.22% | 4.749 |
| bitpack-carrier-packed-simd | 3814ns | 2166ns | 7282ns | +4.78% | 4.296 |

## Performance model

- Peak throughput: **12.268 Gops/s** (bitpack-carrier-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.536 | 53.3% |
| bitpack-carrier-d16-control | 5.350 | 43.6% |
| bitpack-carrier-d32 | 8.642 | 70.4% |
| bitpack-carrier-d64 | 4.392 | 35.8% |
| bitpack-carrier-packed | 6.702 | 54.6% |
| bitpack-carrier-packed-simd | 6.670 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 3724ns | 3724ns | base |
| bitpack-carrier-d16-control | 4998ns | 4998ns | +34.18% |
| bitpack-carrier-d32 | 3668ns | 3668ns | -1.53% |
| bitpack-carrier-d64 | 3439ns | 3439ns | -7.68% |
| bitpack-carrier-packed | 3550ns | 3550ns | -4.67% |
| bitpack-carrier-packed-simd | 3922ns | 3922ns | +5.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 2507ns | base | --- | [2496, 3051] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 3063ns | +555.0ns (+22.1%) | [+544, +1048]ns | [3051, 5718] | YES | 0.0003 | 0.0001 | 2 |
| bitpack-carrier-d32 | 1896ns | -610.9ns (-24.4%) | [-616, -601]ns | [1885, 5067] | YES | 0.0080 | 0.0064 | 0 |
| bitpack-carrier-d64 | 3730ns | no significant difference | [-1159, +466]ns | [1729, 4541] | no | 0.2682 | 0.2682 | 0 |
| bitpack-carrier-packed | 2445ns | -495.8ns (-19.8%) | [-550, -486]ns | [2228, 2457] | YES | 0.0002 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 2456ns | -61.6ns (-2.5%) | [-496, -45]ns | [2445, 3182] | YES | 0.0080 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2496ns | +161.0% | +37.1% | -2.0% | -1.6% | -7.5% |
| 1 | 2 | 3057ns | -0.2% | -38.2% | +37.0% | -20.0% | -19.8% |
| 1 | 3 | 2496ns | +22.5% | -23.6% | +34.9% | -19.9% | -1.6% |
| 1 | 4 | 2501ns | +22.0% | -24.4% | +27.2% | +3.4% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.424 | moderate- |
| bitpack-carrier-d16-control | -0.084 | ok |
| bitpack-carrier-d32 | -0.089 | ok |
| bitpack-carrier-d64 | -0.457 | moderate- |
| bitpack-carrier-packed | -0.512 | HIGH- (thermal bounce) |
| bitpack-carrier-packed-simd | -0.058 | ok |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 1/4, lost 3/4
- **bitpack-carrier-d32**: won 3/4, lost 1/4
- **bitpack-carrier-d64**: won 1/4, lost 3/4
- **bitpack-carrier-packed**: won 3/4, lost 1/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 28.4ns | 3640.0ns | 0.8% |  |
| bitpack-carrier-d16-control | 38.1ns | 4885.8ns | 0.8% |  |
| bitpack-carrier-d32 | 38.7ns | 3532.3ns | 1.1% |  |
| bitpack-carrier-d64 | 50.2ns | 3262.8ns | 1.5% |  |
| bitpack-carrier-packed | 33.7ns | 3450.0ns | 1.0% |  |
| bitpack-carrier-packed-simd | 37.1ns | 3814.1ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 2495.8-7620.4 ns)
   2495.8 |########################################
   2752.1 |
   3008.3 |################
   3264.5 |##
   3520.8 |
   3777.0 |
   4033.2 |
   4289.4 |
   4545.7 |
   4801.9 |
   5058.1 |
   5314.4 |
   5570.6 |##
   5826.8 |
   6083.0 |
   6339.3 |
   6595.5 |
   6851.7 |
   7108.0 |
   7364.2 |
  (4 below, 6 above range)

bitpack-carrier-d16-control (n=40, range 3050.7-8778.5 ns)
   3050.7 |########################################
   3337.1 |
   3623.5 |
   3909.9 |
   4196.3 |
   4482.7 |
   4769.1 |##
   5055.5 |##
   5341.8 |
   5628.2 |####
   5914.6 |
   6201.0 |##
   6487.4 |####
   6773.8 |##
   7060.2 |#######
   7346.6 |##
   7633.0 |
   7919.3 |##
   8205.7 |
   8492.1 |
  (6 below, 4 above range)

bitpack-carrier-d32 (n=40, range 1884.9-6460.5 ns)
   1884.9 |########################################
   2113.7 |
   2342.5 |
   2571.2 |
   2800.0 |
   3028.8 |##
   3257.6 |
   3486.4 |
   3715.1 |
   3943.9 |
   4172.7 |
   4401.5 |
   4630.2 |
   4859.0 |####
   5087.8 |####
   5316.6 |##
   5545.4 |##
   5774.1 |######
   6002.9 |####
   6231.7 |####
  (5 below, 3 above range)

bitpack-carrier-d64 (n=40, range 1335.5-5272.6 ns)
   1335.5 |########################################
   1532.4 |#################
   1729.2 |####
   1926.1 |####
   2122.9 |
   2319.8 |####
   2516.7 |
   2713.5 |
   2910.4 |
   3107.2 |####
   3304.1 |####
   3500.9 |
   3697.8 |
   3894.6 |
   4091.5 |#############
   4288.4 |########
   4485.2 |#################
   4682.1 |########
   4878.9 |#################
   5075.8 |########
  (2 below, 3 above range)

bitpack-carrier-packed (n=40, range 2000.1-7361.9 ns)
   2000.1 |###############################
   2268.2 |########################################
   2536.3 |
   2804.4 |
   3072.5 |##
   3340.6 |
   3608.6 |##
   3876.7 |##
   4144.8 |
   4412.9 |
   4681.0 |
   4949.1 |
   5217.2 |
   5485.3 |
   5753.4 |##
   6021.5 |##
   6289.5 |##
   6557.6 |##
   6825.7 |
   7093.8 |##
  (3 below, 4 above range)

bitpack-carrier-packed-simd (n=40, range 2166.1-7282.0 ns)
   2166.1 |
   2421.9 |########################################
   2677.7 |
   2933.5 |
   3189.3 |
   3445.1 |
   3700.9 |#
   3956.7 |
   4212.5 |
   4468.3 |#
   4724.1 |
   4979.9 |
   5235.7 |
   5491.5 |#
   5747.3 |#
   6003.1 |
   6258.9 |#
   6514.7 |#####
   6770.4 |#####
   7026.2 |#
  (5 below, 2 above range)

```

## Diagnostics

- **bitpack-carrier-d16**: CV=60.1% (high variance, measurements may be unstable)
- **bitpack-carrier-d16**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **bitpack-carrier-d16-control**: CV=49.2% (high variance, measurements may be unstable)
- **bitpack-carrier-d32**: CV=56.7% (high variance, measurements may be unstable)
- **bitpack-carrier-d32**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bitpack-carrier-d64**: CV=50.2% (high variance, measurements may be unstable)
- **bitpack-carrier-d64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-carrier-packed**: CV=61.1% (high variance, measurements may be unstable)
- **bitpack-carrier-packed**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-carrier-packed-simd**: CV=55.0% (high variance, measurements may be unstable)
- **bitpack-carrier-packed-simd**: worst_20/best_20 = 3.4x (possible bimodal distribution)
