# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d32 dominates: 29% faster than the next best (bitpack-carrier-packed)

bitpack-carrier-d32 (1.91 us) leads bitpack-carrier-packed (2.47 us) by 29%, a clear separation rather than a photo finish. CV 103.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-d32 beats baseline by 20% (significant)

bitpack-carrier-d32 is -617 ns (20%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d32 is fastest but the noisiest (CV 103.2%)

bitpack-carrier-d32 wins on median (1.91 us) yet has the highest variance (CV 103.2%), while bitpack-carrier-d64 is the steadiest (CV 43.2%, 3.65 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-carrier-d16 shows alternating (throttle bounce) (autocorr -0.75)

bitpack-carrier-d16's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.74 us) is smaller than the fastest variant's own run-to-run std-dev (1.97 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-carrier-d64 is inconsistent: worst-20% is 3.9x its best-20%

bitpack-carrier-d64's best 20% of batches run at 1.31 us but its worst 20% at 5.18 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-carrier-d32** at 1908.8 ns median (-37.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.91x (fastest 1908.8 ns, slowest 3650.9 ns)

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
| Counter frequency | 2112721357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3749ns | 3114ns | 2547ns | 2953ns | 7338ns | base |
| bitpack-carrier-d16-control | 4108ns | 3114ns | 2620ns | 3146ns | 8479ns | +9.57% |
| bitpack-carrier-d32 | 3668ns | 1981ns | 1952ns | 3225ns | 6711ns | -2.17% |
| bitpack-carrier-d64 | 3270ns | 3842ns | 1383ns | 3157ns | 5497ns | -12.77% |
| bitpack-carrier-packed | 3959ns | 2530ns | 2182ns | 3176ns | 8083ns | +5.59% |
| bitpack-carrier-packed-simd | 3839ns | 2531ns | 2070ns | 3113ns | 7785ns | +2.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3662ns | 2496ns | 7133ns | base | 4.474 |
| bitpack-carrier-d16-control | 4012ns | 2565ns | 8251ns | +9.58% | 4.083 |
| bitpack-carrier-d32 | 3531ns | 1882ns | 6443ns | -3.57% | 4.640 |
| bitpack-carrier-d64 | 3098ns | 1314ns | 5183ns | -15.39% | 5.288 |
| bitpack-carrier-packed | 3849ns | 2128ns | 7828ns | +5.11% | 4.257 |
| bitpack-carrier-packed-simd | 3733ns | 2018ns | 7542ns | +1.94% | 4.389 |

## Performance model

- Peak throughput: **12.467 Gops/s** (bitpack-carrier-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 5.371 | 43.1% |
| bitpack-carrier-d16-control | 5.371 | 43.1% |
| bitpack-carrier-d32 | 8.583 | 68.8% |
| bitpack-carrier-d64 | 4.488 | 36.0% |
| bitpack-carrier-packed | 6.643 | 53.3% |
| bitpack-carrier-packed-simd | 6.641 | 53.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 3749ns | 3749ns | base |
| bitpack-carrier-d16-control | 4108ns | 4108ns | +9.57% |
| bitpack-carrier-d32 | 3668ns | 3668ns | -2.17% |
| bitpack-carrier-d64 | 3270ns | 3270ns | -12.77% |
| bitpack-carrier-packed | 3959ns | 3959ns | +5.59% |
| bitpack-carrier-packed-simd | 3839ns | 3839ns | +2.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3051ns | base | --- | [2507, 3051] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 3051ns | no significant difference | [-0, +12]ns | [3050, 3062] | no | 0.4296 | 0.4296 | 0 |
| bitpack-carrier-d32 | 1909ns | -616.5ns (-20.2%) | [-1147, -593]ns | [1902, 5226] | YES | 0.0107 | 0.0064 | 0 |
| bitpack-carrier-d64 | 3651ns | no significant difference | [-1187, +708]ns | [1453, 4031] | no | 0.3352 | 0.2682 | 0 |
| bitpack-carrier-packed | 2466ns | -483.6ns (-15.9%) | [-579, -221]ns | [2466, 2477] | YES | 0.0002 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 2467ns | -488.5ns (-16.0%) | [-582, -478]ns | [2466, 2710] | YES | 0.0017 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2496ns | -0.0% | -23.5% | -47.0% | -19.0% | -19.0% |
| 1 | 2 | 3057ns | -0.0% | -37.7% | +23.0% | -19.3% | -19.1% |
| 1 | 3 | 2501ns | +22.0% | +74.7% | +53.6% | -1.4% | -18.9% |
| 1 | 4 | 3051ns | +0.0% | -12.5% | +0.2% | -19.0% | -18.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.751 | HIGH- (thermal bounce) |
| bitpack-carrier-d16-control | -0.092 | ok |
| bitpack-carrier-d32 | -0.189 | ok |
| bitpack-carrier-d64 | -0.141 | ok |
| bitpack-carrier-packed | -0.080 | ok |
| bitpack-carrier-packed-simd | -0.747 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 0/4, lost 1/4
- **bitpack-carrier-d32**: won 3/4, lost 1/4
- **bitpack-carrier-d64**: won 1/4, lost 3/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 28.9ns | 3661.8ns | 0.8% |  |
| bitpack-carrier-d16-control | 31.7ns | 4012.5ns | 0.8% |  |
| bitpack-carrier-d32 | 39.2ns | 3531.0ns | 1.1% |  |
| bitpack-carrier-d64 | 48.2ns | 3098.2ns | 1.6% |  |
| bitpack-carrier-packed | 37.7ns | 3848.9ns | 1.0% |  |
| bitpack-carrier-packed-simd | 36.3ns | 3732.9ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 2495.9-7133.4 ns)
   2495.9 |##########################
   2727.8 |
   2959.7 |########################################
   3191.6 |
   3423.4 |
   3655.3 |
   3887.2 |##
   4119.1 |
   4350.9 |
   4582.8 |
   4814.7 |
   5046.5 |
   5278.4 |
   5510.3 |
   5742.2 |
   5974.0 |
   6205.9 |
   6437.8 |##
   6669.6 |
   6901.5 |
  (3 below, 5 above range)

bitpack-carrier-d16-control (n=40, range 2565.4-8251.4 ns)
   2565.4 |
   2849.7 |########################################
   3134.0 |
   3418.3 |
   3702.6 |#
   3986.9 |
   4271.2 |
   4555.5 |
   4839.8 |#
   5124.1 |
   5408.4 |
   5692.7 |
   5977.0 |
   6261.3 |
   6545.6 |
   6829.9 |###
   7114.2 |
   7398.5 |#
   7682.8 |
   7967.1 |#
  (7 below, 3 above range)

bitpack-carrier-d32 (n=40, range 1882.5-6442.8 ns)
   1882.5 |########################################
   2110.5 |##
   2338.5 |##
   2566.5 |
   2794.5 |
   3022.5 |
   3250.6 |##
   3478.6 |##
   3706.6 |
   3934.6 |
   4162.6 |
   4390.6 |
   4618.6 |
   4846.7 |
   5074.7 |#####
   5302.7 |#######
   5530.7 |##
   5758.7 |#####
   5986.7 |#####
   6214.7 |##
  (5 below, 4 above range)

bitpack-carrier-d64 (n=40, range 1314.2-5183.3 ns)
   1314.2 |########################################
   1507.6 |#######
   1701.1 |
   1894.5 |
   2088.0 |
   2281.5 |###
   2474.9 |###
   2668.4 |
   2861.8 |
   3055.3 |
   3248.7 |
   3442.2 |
   3635.6 |##############
   3829.1 |##############
   4022.6 |##############
   4216.0 |
   4409.5 |##############
   4602.9 |###
   4796.4 |###
   4989.8 |
  (3 below, 4 above range)

bitpack-carrier-packed (n=40, range 2127.7-7828.5 ns)
   2127.7 |
   2412.8 |########################################
   2697.8 |
   2982.9 |
   3267.9 |
   3552.9 |
   3838.0 |
   4123.0 |
   4408.0 |
   4693.1 |
   4978.1 |
   5263.2 |#
   5548.2 |
   5833.2 |
   6118.3 |#
   6403.3 |###
   6688.3 |#
   6973.4 |#
   7258.4 |###
   7543.5 |
  (6 below, 4 above range)

bitpack-carrier-packed-simd (n=40, range 2018.1-7542.1 ns)
   2018.1 |#########################
   2294.3 |########################################
   2570.5 |
   2846.7 |##
   3122.9 |
   3399.1 |
   3675.3 |
   3951.5 |##
   4227.7 |##
   4503.9 |
   4780.1 |
   5056.3 |
   5332.5 |##
   5608.7 |
   5884.9 |
   6161.1 |##
   6437.3 |##
   6713.5 |##
   6989.7 |#####
   7265.9 |#####
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-carrier-d16**: CV=55.5% (high variance, measurements may be unstable)
- **bitpack-carrier-d16-control**: CV=57.4% (high variance, measurements may be unstable)
- **bitpack-carrier-d16-control**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **bitpack-carrier-d32**: CV=55.8% (high variance, measurements may be unstable)
- **bitpack-carrier-d32**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **bitpack-carrier-d64**: CV=50.9% (high variance, measurements may be unstable)
- **bitpack-carrier-d64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-carrier-packed**: CV=60.4% (high variance, measurements may be unstable)
- **bitpack-carrier-packed**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-carrier-packed-simd**: CV=59.4% (high variance, measurements may be unstable)
- **bitpack-carrier-packed-simd**: worst_20/best_20 = 3.7x (possible bimodal distribution)
