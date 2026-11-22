# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-contend-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-contend-d16 has the worst median (3.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-contend-d16-control at 2.33 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-contend-d16-control is fastest but the noisiest (CV 63.4%)

bitpack-contend-d16-control wins on median (2.33 us) yet has the highest variance (CV 63.4%), while bitpack-contend-d64 is the steadiest (CV 31.3%, 2.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-d16 shows alternating (throttle bounce) (autocorr -0.67)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (987 ns) is smaller than the fastest variant's own run-to-run std-dev (1.48 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-contend-packed is inconsistent: worst-20% is 4.1x its best-20%

bitpack-contend-packed's best 20% of batches run at 1.04 us but its worst 20% at 4.24 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d16-control** at 2328.5 ns median (-29.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.42x (fastest 2328.5 ns, slowest 3315.6 ns)

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
| bitpack-contend-d16 | 2820ns | 3509ns | 1231ns | 2761ns | 4588ns | base |
| bitpack-contend-d16-control | 2809ns | 2457ns | 1291ns | 2561ns | 5072ns | -0.40% |
| bitpack-contend-d32 | 2714ns | 3224ns | 1098ns | 2762ns | 4189ns | -3.76% |
| bitpack-contend-d64 | 2616ns | 2876ns | 1090ns | 2776ns | 3662ns | -7.24% |
| bitpack-contend-packed | 2901ns | 3241ns | 1094ns | 2975ns | 4488ns | +2.87% |
| bitpack-contend-packed-simd | 2599ns | 2632ns | 1097ns | 2484ns | 4447ns | -7.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 2670ns | 1172ns | 4327ns | base | 6.137 |
| bitpack-contend-d16-control | 2667ns | 1230ns | 4817ns | -0.08% | 6.142 |
| bitpack-contend-d32 | 2549ns | 1038ns | 3921ns | -4.51% | 6.427 |
| bitpack-contend-d64 | 2430ns | 1030ns | 3394ns | -8.96% | 6.741 |
| bitpack-contend-packed | 2742ns | 1037ns | 4240ns | +2.72% | 5.975 |
| bitpack-contend-packed-simd | 2435ns | 1029ns | 4144ns | -8.79% | 6.728 |

## Performance model

- Peak throughput: **15.922 Gops/s** (bitpack-contend-packed-simd; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 4.941 | 31.0% |
| bitpack-contend-d16-control | 7.036 | 44.2% |
| bitpack-contend-d32 | 5.432 | 34.1% |
| bitpack-contend-d64 | 6.104 | 38.3% |
| bitpack-contend-packed | 5.341 | 33.5% |
| bitpack-contend-packed-simd | 6.618 | 41.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 2820ns | 2820ns | base |
| bitpack-contend-d16-control | 2809ns | 2809ns | -0.40% |
| bitpack-contend-d32 | 2714ns | 2714ns | -3.76% |
| bitpack-contend-d64 | 2616ns | 2616ns | -7.24% |
| bitpack-contend-packed | 2901ns | 2901ns | +2.87% |
| bitpack-contend-packed-simd | 2599ns | 2599ns | -7.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3316ns | base | --- | [1288, 3659] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 2328ns | no significant difference | [-1, +158]ns | [1298, 3672] | no | 0.1009 | 0.0807 | 0 |
| bitpack-contend-d32 | 3016ns | -251.9ns (-7.6%) | [-328, -107]ns | [1548, 3378] | YES | 0.0009 | 0.0002 | 0 |
| bitpack-contend-d64 | 2684ns | -156.7ns (-4.7%) | [-802, -58]ns | [2494, 2771] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| bitpack-contend-packed | 3068ns | -145.8ns (-4.4%) | [-252, -50]ns | [2534, 3417] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| bitpack-contend-packed-simd | 2476ns | no significant difference | [-721, +146]ns | [1233, 3057] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3316ns | +9.0% | -14.7% | -21.7% | -10.0% | -68.1% |
| 1 | 2 | 1209ns | +54.5% | +94.7% | +139.5% | +100.2% | +153.9% |
| 1 | 3 | 3861ns | -68.2% | -39.7% | -72.3% | -11.3% | -37.4% |
| 1 | 4 | 2703ns | +19.0% | +12.7% | +0.0% | +16.6% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.669 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | -0.225 | moderate- |
| bitpack-contend-d32 | -0.241 | moderate- |
| bitpack-contend-d64 | -0.492 | moderate- |
| bitpack-contend-packed | -0.322 | moderate- |
| bitpack-contend-packed-simd | -0.366 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 2/4, lost 2/4
- **bitpack-contend-d64**: won 2/4, lost 1/4
- **bitpack-contend-packed**: won 2/4, lost 2/4
- **bitpack-contend-packed-simd**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 51.9ns | 2669.6ns | 1.9% |  |
| bitpack-contend-d16-control | 48.7ns | 2667.4ns | 1.8% |  |
| bitpack-contend-d32 | 56.2ns | 2549.3ns | 2.2% |  |
| bitpack-contend-d64 | 62.9ns | 2430.5ns | 2.6% |  |
| bitpack-contend-packed | 57.4ns | 2742.3ns | 2.1% |  |
| bitpack-contend-packed-simd | 55.8ns | 2435.0ns | 2.3% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 1172.2-4327.5 ns)
   1172.2 |########################################
   1330.0 |###
   1487.7 |
   1645.5 |
   1803.2 |###
   1961.0 |
   2118.8 |###
   2276.5 |
   2434.3 |
   2592.1 |
   2749.8 |
   2907.6 |
   3065.4 |
   3223.1 |##########
   3380.9 |###
   3538.7 |#############
   3696.4 |##########
   3854.2 |#############
   4012.0 |###
   4169.7 |###
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 1230.4-4816.8 ns)
   1230.4 |########################################
   1409.7 |
   1589.1 |
   1768.4 |
   1947.7 |
   2127.0 |##
   2306.3 |#####
   2485.7 |
   2665.0 |
   2844.3 |
   3023.6 |##
   3202.9 |##
   3382.3 |##
   3561.6 |###########
   3740.9 |
   3920.2 |########
   4099.5 |
   4278.9 |#####
   4458.2 |##
   4637.5 |#####
  (5 below, 3 above range)

bitpack-contend-d32 (n=40, range 1037.6-3921.4 ns)
   1037.6 |########################################
   1181.8 |
   1326.0 |
   1470.2 |
   1614.4 |
   1758.5 |
   1902.7 |####
   2046.9 |
   2191.1 |
   2335.3 |
   2479.5 |####
   2623.7 |
   2767.9 |############
   2912.0 |############
   3056.2 |############
   3200.4 |
   3344.6 |############
   3488.8 |################
   3633.0 |################
   3777.2 |####
  (4 below, 3 above range)

bitpack-contend-d64 (n=40, range 1030.3-3394.4 ns)
   1030.3 |########################################
   1148.5 |
   1266.7 |
   1384.9 |
   1503.1 |
   1621.3 |
   1739.5 |
   1857.7 |#####
   1975.9 |
   2094.1 |
   2212.3 |
   2330.5 |##########
   2448.7 |#########################
   2566.9 |###############
   2685.1 |###################################
   2803.3 |#########################
   2921.5 |###############
   3039.7 |
   3157.9 |#####
   3276.1 |
  (1 below, 4 above range)

bitpack-contend-packed (n=40, range 1036.5-4240.1 ns)
   1036.5 |########################################
   1196.7 |
   1356.9 |
   1517.1 |
   1677.3 |####
   1837.4 |
   1997.6 |
   2157.8 |
   2318.0 |####
   2478.2 |
   2638.3 |########
   2798.5 |#############
   2958.7 |########
   3118.9 |########
   3279.1 |######################
   3439.2 |####
   3599.4 |
   3759.6 |########
   3919.8 |########
   4080.0 |#################
  (3 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 1029.0-4143.6 ns)
   1029.0 |########################################
   1184.8 |########
   1340.5 |
   1496.2 |
   1652.0 |
   1807.7 |
   1963.4 |
   2119.1 |
   2274.9 |####################
   2430.6 |####
   2586.3 |########
   2742.1 |####
   2897.8 |########
   3053.5 |############
   3209.2 |
   3365.0 |########
   3520.7 |####
   3676.4 |####
   3832.2 |############
   3987.9 |####
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=50.0% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=55.3% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-contend-d32**: CV=46.1% (high variance, measurements may be unstable)
- **bitpack-contend-d32**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bitpack-contend-d64**: CV=34.6% (high variance, measurements may be unstable)
- **bitpack-contend-d64**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-packed**: CV=45.0% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=49.3% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 4.0x (possible bimodal distribution)
