# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 24% (significant)

warm-container-native is -753 ns (24%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.78)

warm-container-native's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-kernel, warm-container-headroom, warm-container-native, warm-container-lanes-deferred} vs {warm-container-plusone} (33% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel, warm-container-headroom, warm-container-native, warm-container-lanes-deferred} and a slow tier {warm-container-plusone} with a 33% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-native is inconsistent: worst-20% is 4.4x its best-20%

warm-container-native's best 20% of batches run at 986 ns but its worst 20% at 4.38 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 2908.9 ns median (-7.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.61x (fastest 2908.9 ns, slowest 4681.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112787121 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3599ns | 3304ns | 1526ns | 3350ns | 6415ns | base |
| warm-container-kernel | 3203ns | 3184ns | 2342ns | 3267ns | 3868ns | -11.00% |
| warm-container-lanes-deferred | 3330ns | 3748ns | 1768ns | 3615ns | 4036ns | -7.46% |
| warm-container-minimum | 3069ns | 3093ns | 2535ns | 3063ns | 3621ns | -14.71% |
| warm-container-native | 3195ns | 3694ns | 1049ns | 3405ns | 4708ns | -11.23% |
| warm-container-plusone | 4058ns | 4898ns | 1488ns | 4157ns | 6330ns | +12.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3426ns | 1458ns | 6078ns | base | 9.565 |
| warm-container-kernel | 3001ns | 2202ns | 3595ns | -12.41% | 10.920 |
| warm-container-lanes-deferred | 3121ns | 1661ns | 3756ns | -8.90% | 10.499 |
| warm-container-minimum | 2874ns | 2383ns | 3360ns | -16.11% | 11.402 |
| warm-container-native | 2987ns | 986ns | 4379ns | -12.80% | 10.969 |
| warm-container-plusone | 3864ns | 1422ns | 6003ns | +12.78% | 8.481 |

## Performance model

- Peak throughput: **33.250 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 10.375 | 31.2% |
| warm-container-kernel | 10.950 | 32.9% |
| warm-container-lanes-deferred | 9.292 | 27.9% |
| warm-container-minimum | 11.265 | 33.9% |
| warm-container-native | 9.441 | 28.4% |
| warm-container-plusone | 7.000 | 21.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3599ns | 3599ns | base |
| warm-container-kernel | 3203ns | 3203ns | -11.00% |
| warm-container-lanes-deferred | 3330ns | 3330ns | -7.46% |
| warm-container-minimum | 3069ns | 3069ns | -14.71% |
| warm-container-native | 3195ns | 3195ns | -11.23% |
| warm-container-plusone | 4058ns | 4058ns | +12.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3158ns | base | --- | [1732, 4361] | --- | --- | --- | --- |
| warm-container-kernel | 2993ns | no significant difference | [-1278, +1103]ns | [2943, 3234] | no | 0.7948 | 0.6358 | 0 |
| warm-container-lanes-deferred | 3526ns | no significant difference | [-771, +661]ns | [3222, 3555] | no | 0.2564 | 0.1539 | 0 |
| warm-container-minimum | 2909ns | no significant difference | [-1279, +820]ns | [2744, 2986] | no | 0.8746 | 0.8746 | 0 |
| warm-container-native | 3471ns | -752.8ns (-23.8%) | [-888, -538]ns | [2954, 3555] | YES | 0.0161 | 0.0064 | 0 |
| warm-container-plusone | 4681ns | +56.7ns (+1.8%) | [+1, +524]ns | [2129, 5060] | YES | 0.0161 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3921ns | -30.4% | -18.0% | -29.9% | -17.8% | +15.6% |
| 1 | 2 | 3077ns | +1.1% | +15.6% | -2.9% | +15.1% | +73.8% |
| 1 | 3 | 1738ns | +96.5% | +104.6% | +55.4% | +76.3% | +164.3% |
| 1 | 4 | 1992ns | +50.8% | +70.8% | +55.2% | +79.4% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.251 | moderate+ |
| warm-container-kernel | -0.084 | ok |
| warm-container-lanes-deferred | -0.194 | ok |
| warm-container-minimum | -0.671 | HIGH- (thermal bounce) |
| warm-container-native | -0.775 | HIGH- (thermal bounce) |
| warm-container-plusone | 0.004 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 1/4, lost 3/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 1/4, lost 3/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 47.8ns | 3426.0ns | 1.4% |  |
| warm-container-kernel | 72.4ns | 3000.6ns | 2.4% |  |
| warm-container-lanes-deferred | 75.5ns | 3121.1ns | 2.4% |  |
| warm-container-minimum | 68.3ns | 2874.0ns | 2.4% |  |
| warm-container-native | 72.7ns | 2987.4ns | 2.4% |  |
| warm-container-plusone | 53.9ns | 3863.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1457.9-6078.4 ns)
   1457.9 |
   1688.9 |########################################
   1919.9 |
   2150.9 |###
   2382.0 |###
   2613.0 |
   2844.0 |
   3075.1 |
   3306.1 |
   3537.1 |###
   3768.1 |#######
   3999.2 |#######
   4230.2 |##########
   4461.2 |
   4692.3 |
   4923.3 |
   5154.3 |##########
   5385.3 |
   5616.4 |##############
   5847.4 |
  (7 below, 5 above range)

warm-container-kernel (n=40, range 2201.9-3595.3 ns)
   2201.9 |
   2271.6 |
   2341.3 |
   2410.9 |
   2480.6 |##########
   2550.3 |#####
   2619.9 |
   2689.6 |####################
   2759.3 |#####
   2829.0 |#####
   2898.6 |########################################
   2968.3 |###############
   3038.0 |
   3107.6 |#####
   3177.3 |#########################
   3247.0 |##########
   3316.6 |
   3386.3 |##########
   3456.0 |
   3525.7 |####################
  (2 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1661.2-3755.6 ns)
   1661.2 |
   1766.0 |
   1870.7 |
   1975.4 |
   2080.1 |
   2184.8 |##########
   2289.6 |
   2394.3 |
   2499.0 |
   2603.7 |
   2708.4 |
   2813.2 |
   2917.9 |######
   3022.6 |######
   3127.3 |#############
   3232.0 |######
   3336.8 |###
   3441.5 |################
   3546.2 |########################################
   3650.9 |###
  (4 below, 4 above range)

warm-container-minimum (n=40, range 2382.9-3360.0 ns)
   2382.9 |
   2431.7 |
   2480.6 |######
   2529.4 |########################################
   2578.3 |
   2627.1 |#############
   2676.0 |######
   2724.8 |#################################
   2773.7 |#############
   2822.6 |######
   2871.4 |######
   2920.3 |#################################
   2969.1 |##########################
   3018.0 |#############
   3066.8 |
   3115.7 |
   3164.6 |######
   3213.4 |#############
   3262.3 |####################
   3311.1 |
  (1 below, 3 above range)

warm-container-native (n=40, range 985.5-4378.6 ns)
    985.5 |#########################
   1155.2 |
   1324.8 |
   1494.5 |
   1664.1 |
   1833.8 |
   2003.4 |#####
   2173.1 |#####
   2342.7 |
   2512.4 |
   2682.1 |#####
   2851.7 |###############
   3021.4 |
   3191.0 |####################
   3360.7 |##########
   3530.3 |########################################
   3700.0 |##########
   3869.6 |###############
   4039.3 |##########
   4208.9 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 1421.5-6003.4 ns)
   1421.5 |#############
   1650.6 |########################################
   1879.7 |
   2108.8 |
   2337.9 |######
   2567.0 |
   2796.1 |
   3025.2 |
   3254.3 |######
   3483.4 |
   3712.5 |
   3941.5 |
   4170.6 |######
   4399.7 |####################
   4628.8 |##########################
   4857.9 |#############
   5087.0 |####################
   5316.1 |####################
   5545.2 |####################
   5774.3 |######
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=54.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=26.9% (high variance, measurements may be unstable)
- **warm-container-native**: CV=40.8% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-plusone**: CV=47.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.2x (possible bimodal distribution)
