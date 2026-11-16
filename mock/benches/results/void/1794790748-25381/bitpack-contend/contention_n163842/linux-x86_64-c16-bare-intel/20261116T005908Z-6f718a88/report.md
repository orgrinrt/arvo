# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d32 dominates: 22% faster than the next best (bitpack-contend-d16)

bitpack-contend-d32 (1.46 us) leads bitpack-contend-d16 (1.79 us) by 22%, a clear separation rather than a photo finish. CV 114.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-packed-simd is an outlier: 2.4x slower than the field

bitpack-contend-packed-simd (3.58 us) is 2.4x the fastest (1.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d32 is fastest but the noisiest (CV 114.9%)

bitpack-contend-d32 wins on median (1.46 us) yet has the highest variance (CV 114.9%), while bitpack-contend-packed-simd is the steadiest (CV 35.5%, 3.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-d32 shows alternating (throttle bounce) (autocorr -0.61)

bitpack-contend-d32's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-contend-d32 is inconsistent: worst-20% is 4.5x its best-20%

bitpack-contend-d32's best 20% of batches run at 1.14 us but its worst 20% at 5.12 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d32** at 1464.4 ns median (-18.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.44x (fastest 1464.4 ns, slowest 3577.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749291 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 2986ns | 1853ns | 1460ns | 2577ns | 5736ns | base |
| bitpack-contend-d16-control | 3195ns | 2141ns | 1511ns | 2830ns | 5972ns | +7.01% |
| bitpack-contend-d32 | 2827ns | 1526ns | 1193ns | 2518ns | 5386ns | -5.33% |
| bitpack-contend-d64 | 2854ns | 3087ns | 1091ns | 2838ns | 4667ns | -4.40% |
| bitpack-contend-packed | 3302ns | 3452ns | 1422ns | 3068ns | 5881ns | +10.59% |
| bitpack-contend-packed-simd | 3168ns | 3745ns | 1397ns | 3268ns | 4640ns | +6.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 2873ns | 1408ns | 5491ns | base | 5.702 |
| bitpack-contend-d16-control | 3077ns | 1457ns | 5720ns | +7.08% | 5.325 |
| bitpack-contend-d32 | 2700ns | 1137ns | 5117ns | -6.03% | 6.068 |
| bitpack-contend-d64 | 2666ns | 1018ns | 4387ns | -7.20% | 6.145 |
| bitpack-contend-packed | 3159ns | 1358ns | 5633ns | +9.97% | 5.186 |
| bitpack-contend-packed-simd | 3026ns | 1337ns | 4410ns | +5.31% | 5.415 |

## Performance model

- Peak throughput: **16.093 Gops/s** (bitpack-contend-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 9.165 | 57.0% |
| bitpack-contend-d16-control | 7.841 | 48.7% |
| bitpack-contend-d32 | 11.188 | 69.5% |
| bitpack-contend-d64 | 5.709 | 35.5% |
| bitpack-contend-packed | 4.964 | 30.8% |
| bitpack-contend-packed-simd | 4.579 | 28.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 2986ns | 2986ns | base |
| bitpack-contend-d16-control | 3195ns | 3195ns | +7.01% |
| bitpack-contend-d32 | 2827ns | 2827ns | -5.33% |
| bitpack-contend-d64 | 2854ns | 2854ns | -4.40% |
| bitpack-contend-packed | 3302ns | 3302ns | +10.59% |
| bitpack-contend-packed-simd | 3168ns | 3168ns | +6.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1788ns | base | --- | [1707, 3167] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 2089ns | no significant difference | [-44, +314]ns | [1753, 4423] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-d32 | 1464ns | -315.3ns (-17.6%) | [-393, -136]ns | [1330, 3954] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 2870ns | -502.2ns (-28.1%) | [-607, -47]ns | [2339, 3176] | YES (adj: no) | 0.1345 | 0.0807 | 0 |
| bitpack-contend-packed | 3300ns | no significant difference | [-359, +204]ns | [1619, 4165] | no | 0.3352 | 0.2682 | 0 |
| bitpack-contend-packed-simd | 3578ns | no significant difference | [-365, +43]ns | [1715, 3800] | no | 0.1345 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2138ns | -34.9% | +0.8% | +31.1% | +69.1% | +65.6% |
| 1 | 2 | 2024ns | +120.4% | -11.5% | +89.9% | +46.7% | +99.3% |
| 1 | 3 | 2140ns | -18.7% | +79.1% | +45.7% | +73.6% | +70.7% |
| 1 | 4 | 1415ns | +48.1% | -21.8% | -21.4% | +14.4% | +73.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.191 | ok |
| bitpack-contend-d16-control | -0.565 | HIGH- (thermal bounce) |
| bitpack-contend-d32 | -0.611 | HIGH- (thermal bounce) |
| bitpack-contend-d64 | -0.025 | ok |
| bitpack-contend-packed | -0.364 | moderate- |
| bitpack-contend-packed-simd | -0.007 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 2/4
- **bitpack-contend-d32**: won 2/4, lost 2/4
- **bitpack-contend-d64**: won 1/4, lost 3/4
- **bitpack-contend-packed**: won 0/4, lost 4/4
- **bitpack-contend-packed-simd**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 38.1ns | 2873.2ns | 1.3% |  |
| bitpack-contend-d16-control | 40.5ns | 3076.6ns | 1.3% |  |
| bitpack-contend-d32 | 42.2ns | 2699.9ns | 1.6% |  |
| bitpack-contend-d64 | 51.2ns | 2666.2ns | 1.9% |  |
| bitpack-contend-packed | 47.9ns | 3159.5ns | 1.5% |  |
| bitpack-contend-packed-simd | 49.3ns | 3025.8ns | 1.6% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1408.0-5490.7 ns)
   1408.0 |########################################
   1612.1 |#############################
   1816.3 |#######
   2020.4 |
   2224.5 |#######
   2428.7 |
   2632.8 |
   2836.9 |###
   3041.1 |
   3245.2 |
   3449.3 |###
   3653.5 |
   3857.6 |
   4061.7 |
   4265.9 |
   4470.0 |#######
   4674.1 |
   4878.3 |##########
   5082.4 |##########
   5286.5 |#######
  (2 below, 3 above range)

bitpack-contend-d16-control (n=40, range 1456.7-5720.4 ns)
   1456.7 |####
   1669.9 |########################################
   1883.1 |##########################
   2096.2 |########
   2309.4 |
   2522.6 |
   2735.8 |
   2949.0 |
   3162.2 |
   3375.4 |
   3588.5 |####
   3801.7 |
   4014.9 |
   4228.1 |####
   4441.3 |######################
   4654.5 |#############
   4867.7 |
   5080.8 |
   5294.0 |########
   5507.2 |
  (6 below, 4 above range)

bitpack-contend-d32 (n=40, range 1136.6-5116.6 ns)
   1136.6 |########################################
   1335.6 |######################
   1534.6 |########
   1733.6 |
   1932.6 |
   2131.6 |####
   2330.6 |
   2529.6 |####
   2728.6 |
   2927.6 |
   3126.6 |
   3325.6 |
   3524.6 |####
   3723.6 |
   3922.6 |#############
   4121.6 |####
   4320.6 |####
   4519.6 |
   4718.6 |######################
   4917.6 |#################
  (6 below, 1 above range)

bitpack-contend-d64 (n=40, range 1018.1-4386.8 ns)
   1018.1 |########################################
   1186.5 |##############################
   1355.0 |####################
   1523.4 |
   1691.8 |
   1860.3 |
   2028.7 |##########
   2197.1 |
   2365.6 |
   2534.0 |##############################
   2702.4 |##############################
   2870.9 |########################################
   3039.3 |##############################
   3207.7 |##########
   3376.2 |##############################
   3544.6 |##########
   3713.0 |##########
   3881.5 |####################
   4049.9 |##########
   4218.4 |
  (4 below, 4 above range)

bitpack-contend-packed (n=40, range 1357.8-5632.7 ns)
   1357.8 |########################################
   1571.6 |########################################
   1785.3 |
   1999.1 |
   2212.8 |
   2426.6 |
   2640.3 |#####
   2854.0 |
   3067.8 |#####
   3281.5 |#####
   3495.3 |#####
   3709.0 |##########
   3922.8 |##########
   4136.5 |###############
   4350.2 |####################
   4564.0 |#####
   4777.7 |##########
   4991.5 |#####
   5205.2 |##########
   5419.0 |
  (2 below, 1 above range)

bitpack-contend-packed-simd (n=40, range 1337.1-4410.5 ns)
   1337.1 |########################################
   1490.7 |#############
   1644.4 |
   1798.1 |####
   1951.8 |
   2105.4 |
   2259.1 |
   2412.8 |
   2566.4 |
   2720.1 |
   2873.8 |
   3027.4 |
   3181.1 |
   3334.8 |####
   3488.5 |######################
   3642.1 |#################
   3795.8 |########
   3949.5 |#############
   4103.1 |########
   4256.8 |##########################
  (2 below, 2 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=58.9% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=55.8% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-contend-d32**: CV=62.3% (high variance, measurements may be unstable)
- **bitpack-contend-d32**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **bitpack-contend-d64**: CV=46.5% (high variance, measurements may be unstable)
- **bitpack-contend-d64**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **bitpack-contend-packed**: CV=58.9% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=41.9% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.3x (possible bimodal distribution)
