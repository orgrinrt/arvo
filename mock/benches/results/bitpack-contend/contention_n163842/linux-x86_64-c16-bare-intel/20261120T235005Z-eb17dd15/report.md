# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 beats baseline by 34% (significant)

bitpack-contend-d64 is -659 ns (34%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 shows alternating (throttle bounce) (autocorr -0.79)

bitpack-contend-d64's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.52 us) is smaller than the fastest variant's own run-to-run std-dev (1.68 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-contend-d64 is inconsistent: worst-20% is 5.3x its best-20%

bitpack-contend-d64's best 20% of batches run at 896 ns but its worst 20% at 4.72 us (5.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-packed-simd** at 1638.3 ns median (-15.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.93x (fastest 1638.3 ns, slowest 3159.4 ns)

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
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3109ns | 2016ns | 1756ns | 2396ns | 6600ns | base |
| bitpack-contend-d16-control | 2664ns | 1772ns | 1454ns | 1876ns | 6239ns | -14.31% |
| bitpack-contend-d32 | 2837ns | 3315ns | 1185ns | 2767ns | 4701ns | -8.74% |
| bitpack-contend-d64 | 2803ns | 2507ns | 964ns | 2672ns | 5034ns | -9.85% |
| bitpack-contend-packed | 2865ns | 1861ns | 1243ns | 2686ns | 5021ns | -7.86% |
| bitpack-contend-packed-simd | 3034ns | 1690ns | 1338ns | 2734ns | 5629ns | -2.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 3001ns | 1701ns | 6338ns | base | 5.460 |
| bitpack-contend-d16-control | 2567ns | 1403ns | 5989ns | -14.45% | 6.382 |
| bitpack-contend-d32 | 2699ns | 1131ns | 4454ns | -10.04% | 6.070 |
| bitpack-contend-d64 | 2623ns | 896ns | 4718ns | -12.57% | 6.245 |
| bitpack-contend-packed | 2744ns | 1190ns | 4786ns | -8.54% | 5.970 |
| bitpack-contend-packed-simd | 2906ns | 1280ns | 5379ns | -3.16% | 5.639 |

## Performance model

- Peak throughput: **18.281 Gops/s** (bitpack-contend-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 8.420 | 46.1% |
| bitpack-contend-d16-control | 9.586 | 52.4% |
| bitpack-contend-d32 | 5.186 | 28.4% |
| bitpack-contend-d64 | 6.895 | 37.7% |
| bitpack-contend-packed | 9.170 | 50.2% |
| bitpack-contend-packed-simd | 10.001 | 54.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 3109ns | 3109ns | base |
| bitpack-contend-d16-control | 2664ns | 2664ns | -14.31% |
| bitpack-contend-d32 | 2837ns | 2837ns | -8.74% |
| bitpack-contend-d64 | 2803ns | 2803ns | -9.85% |
| bitpack-contend-packed | 2865ns | 2865ns | -7.86% |
| bitpack-contend-packed-simd | 3034ns | 3034ns | -2.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 1946ns | base | --- | [1723, 2100] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 1709ns | -294.6ns (-15.1%) | [-676, -136]ns | [1570, 1802] | YES | 0.0009 | 0.0002 | 0 |
| bitpack-contend-d32 | 3159ns | -502.6ns (-25.8%) | [-624, -384]ns | [1310, 3796] | YES | 0.0011 | 0.0007 | 0 |
| bitpack-contend-d64 | 2376ns | -659.4ns (-33.9%) | [-889, -57]ns | [1498, 3611] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-contend-packed | 1787ns | -370.5ns (-19.0%) | [-778, -281]ns | [1432, 4175] | YES | 0.0028 | 0.0022 | 0 |
| bitpack-contend-packed-simd | 1638ns | -310.8ns (-16.0%) | [-324, -226]ns | [1480, 3978] | YES | 0.0011 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1724ns | -0.6% | -29.1% | +4.4% | -17.0% | -14.1% |
| 1 | 2 | 2100ns | -32.1% | +66.6% | +65.4% | +66.0% | +71.0% |
| 1 | 3 | 1705ns | -17.6% | +133.1% | -45.2% | +58.2% | +56.0% |
| 1 | 4 | 1946ns | -2.5% | -28.4% | +62.6% | +44.6% | -15.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.783 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | -0.238 | moderate- |
| bitpack-contend-d32 | -0.247 | moderate- |
| bitpack-contend-d64 | -0.795 | HIGH- (thermal bounce) |
| bitpack-contend-packed | -0.424 | moderate- |
| bitpack-contend-packed-simd | -0.312 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 4/4, lost 0/4
- **bitpack-contend-d32**: won 2/4, lost 2/4
- **bitpack-contend-d64**: won 1/4, lost 3/4
- **bitpack-contend-packed**: won 1/4, lost 3/4
- **bitpack-contend-packed-simd**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 36.0ns | 3000.6ns | 1.2% |  |
| bitpack-contend-d16-control | 32.2ns | 2567.1ns | 1.3% |  |
| bitpack-contend-d32 | 46.8ns | 2699.3ns | 1.7% |  |
| bitpack-contend-d64 | 48.3ns | 2623.4ns | 1.8% |  |
| bitpack-contend-packed | 41.0ns | 2744.3ns | 1.5% |  |
| bitpack-contend-packed-simd | 43.4ns | 2905.7ns | 1.5% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1701.0-6337.7 ns)
   1701.0 |########################################
   1932.8 |########################
   2164.6 |
   2396.5 |
   2628.3 |
   2860.2 |
   3092.0 |
   3323.8 |##
   3555.7 |
   3787.5 |
   4019.3 |
   4251.2 |##
   4483.0 |
   4714.9 |
   4946.7 |#####
   5178.5 |#####
   5410.4 |
   5642.2 |##
   5874.1 |
   6105.9 |
  (4 below, 5 above range)

bitpack-contend-d16-control (n=40, range 1403.1-5989.4 ns)
   1403.1 |#################################
   1632.4 |########################################
   1861.7 |##########
   2091.1 |###
   2320.4 |
   2549.7 |
   2779.0 |
   3008.3 |###
   3237.6 |
   3467.0 |###
   3696.3 |
   3925.6 |
   4154.9 |###
   4384.2 |
   4613.5 |
   4842.9 |
   5072.2 |
   5301.5 |###
   5530.8 |######
   5760.1 |###
  (4 below, 3 above range)

bitpack-contend-d32 (n=40, range 1131.4-4454.2 ns)
   1131.4 |########################################
   1297.5 |#################
   1463.7 |#####
   1629.8 |#####
   1796.0 |#####
   1962.1 |
   2128.2 |
   2294.4 |
   2460.5 |
   2626.7 |
   2792.8 |
   2959.0 |#####
   3125.1 |#####
   3291.2 |
   3457.4 |#################
   3623.5 |###########
   3789.7 |#####
   3955.8 |##################################
   4121.9 |#################
   4288.1 |#####
  (6 below, 3 above range)

bitpack-contend-d64 (n=40, range 896.2-4718.3 ns)
    896.2 |##########################
   1087.3 |#################################
   1278.4 |
   1469.5 |
   1660.6 |########################################
   1851.7 |
   2042.8 |
   2233.9 |
   2425.0 |
   2616.1 |
   2807.2 |######
   2998.3 |######
   3189.4 |######
   3380.6 |####################
   3571.7 |####################
   3762.8 |#############
   3953.9 |#############
   4145.0 |#############
   4336.1 |#############
   4527.2 |
  (5 below, 3 above range)

bitpack-contend-packed (n=40, range 1190.5-4785.6 ns)
   1190.5 |###############
   1370.2 |########################################
   1550.0 |###############
   1729.7 |#####
   1909.5 |
   2089.3 |#####
   2269.0 |
   2448.8 |
   2628.5 |
   2808.3 |
   2988.1 |#####
   3167.8 |#####
   3347.6 |
   3527.3 |
   3707.1 |
   3886.8 |#####
   4066.6 |##########
   4246.4 |##############################
   4426.1 |#########################
   4605.9 |
  (6 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 1280.1-5378.6 ns)
   1280.1 |########################################
   1485.1 |############################
   1690.0 |
   1894.9 |
   2099.8 |
   2304.8 |####
   2509.7 |
   2714.6 |
   2919.5 |####
   3124.5 |
   3329.4 |####
   3534.3 |
   3739.2 |########
   3944.2 |########
   4149.1 |
   4354.0 |########
   4558.9 |####
   4763.9 |####
   4968.8 |############
   5173.7 |########
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=63.0% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=70.6% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **bitpack-contend-d32**: CV=52.1% (high variance, measurements may be unstable)
- **bitpack-contend-d32**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-contend-d64**: CV=57.6% (high variance, measurements may be unstable)
- **bitpack-contend-d64**: worst_20/best_20 = 5.3x (possible bimodal distribution)
- **bitpack-contend-packed**: CV=55.8% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=57.9% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 4.2x (possible bimodal distribution)
