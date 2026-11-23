# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-container-plusone, warm-container-headroom) are a dead heat (<1%)

warm-container-plusone (2.65 us) and warm-container-headroom (2.65 us) differ by 0.01%, inside the noise, even though the wider field spreads 44.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.17 us) is smaller than the fastest variant's own run-to-run std-dev (2.52 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-plusone, warm-container-headroom, warm-container-kernel} vs {warm-container-minimum, warm-container-native} (27% apart)

The field splits into a fast tier {warm-container-plusone, warm-container-headroom, warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-plusone vs stability leader warm-container-kernel (+2% speed for 8.7x steadier)

warm-container-plusone is fastest (2.65 us, CV 95.2%); warm-container-kernel gives up 1.5% median for 8.7x lower variance (CV 11.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-minimum is inconsistent: worst-20% is 4.2x its best-20%

warm-container-minimum's best 20% of batches run at 1.17 us but its worst 20% at 4.87 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 2651.2 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.44x (fastest 2651.2 ns, slowest 3826.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 64 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112747662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4310ns | 2733ns | 2353ns | 3294ns | 9317ns | base |
| warm-container-kernel | 2975ns | 2923ns | 2582ns | 2927ns | 3512ns | -30.98% |
| warm-container-minimum | 3404ns | 3668ns | 1254ns | 3503ns | 5254ns | -21.03% |
| warm-container-native | 3617ns | 4096ns | 1525ns | 3971ns | 4647ns | -16.09% |
| warm-container-plusone | 4143ns | 2733ns | 2235ns | 3283ns | 8632ns | -3.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4172ns | 2283ns | 8988ns | base | 3.927 |
| warm-container-kernel | 2730ns | 2374ns | 3206ns | -34.56% | 6.001 |
| warm-container-minimum | 3169ns | 1168ns | 4866ns | -24.03% | 5.169 |
| warm-container-native | 3369ns | 1424ns | 4305ns | -19.24% | 4.863 |
| warm-container-plusone | 4010ns | 2169ns | 8325ns | -3.88% | 4.086 |

## Performance model

- Peak throughput: **14.028 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.179 | 44.0% |
| warm-container-kernel | 6.086 | 43.4% |
| warm-container-minimum | 4.778 | 34.1% |
| warm-container-native | 4.282 | 30.5% |
| warm-container-plusone | 6.180 | 44.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4310ns | 4310ns | base |
| warm-container-kernel | 2975ns | 2975ns | -30.98% |
| warm-container-minimum | 3404ns | 3404ns | -21.03% |
| warm-container-native | 3617ns | 3617ns | -16.09% |
| warm-container-plusone | 4143ns | 4143ns | -3.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2652ns | base | --- | [2651, 2665] | --- | --- | --- | --- |
| warm-container-kernel | 2692ns | no significant difference | [-356, +100]ns | [2550, 2808] | no | 0.5364 | 0.2682 | 0 |
| warm-container-minimum | 3429ns | no significant difference | [-1483, +697]ns | [3041, 3611] | no | 1.0000 | 1.0000 | 0 |
| warm-container-native | 3826ns | no significant difference | [-1004, +818]ns | [3499, 3922] | no | 1.0000 | 0.8746 | 0 |
| warm-container-plusone | 2651ns | -481.6ns (-18.2%) | [-483, -0]ns | [2416, 2664] | YES (adj: no) | 0.0948 | 0.0237 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2658ns | -7.4% | +43.0% | +27.7% | -0.0% |
| 1 | 2 | 2174ns | +17.2% | +47.0% | +75.0% | +22.2% |
| 1 | 3 | 2657ns | +10.8% | -56.0% | +49.2% | -18.2% |
| 1 | 4 | 2657ns | +3.5% | +34.6% | +44.1% | -18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.417 | moderate- |
| warm-container-kernel | 0.095 | ok |
| warm-container-minimum | -0.314 | moderate- |
| warm-container-native | 0.052 | ok |
| warm-container-plusone | 0.252 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 0/4, lost 4/4
- **warm-container-plusone**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 40.9ns | 4171.7ns | 1.0% |  |
| warm-container-kernel | 73.3ns | 2730.0ns | 2.7% |  |
| warm-container-minimum | 70.2ns | 3169.4ns | 2.2% |  |
| warm-container-native | 75.5ns | 3369.2ns | 2.2% |  |
| warm-container-plusone | 39.0ns | 4009.7ns | 1.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2283.0-8988.2 ns)
   2283.0 |##
   2618.3 |########################################
   2953.5 |
   3288.8 |######
   3624.0 |
   3959.3 |
   4294.6 |
   4629.8 |
   4965.1 |
   5300.3 |
   5635.6 |
   5970.9 |
   6306.1 |
   6641.4 |
   6976.6 |
   7311.9 |
   7647.2 |##
   7982.4 |####
   8317.7 |##
   8652.9 |####
  (6 below, 4 above range)

warm-container-kernel (n=40, range 2374.2-3206.1 ns)
   2374.2 |#############
   2415.8 |####################
   2457.4 |######
   2499.0 |
   2540.6 |########################################
   2582.2 |######
   2623.8 |#############
   2665.4 |######
   2707.0 |#############
   2748.5 |####################
   2790.1 |####################
   2831.7 |#############
   2873.3 |######
   2914.9 |
   2956.5 |
   2998.1 |######
   3039.7 |
   3081.3 |####################
   3122.9 |#############
   3164.5 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 1168.0-4866.1 ns)
   1168.0 |################################
   1352.9 |########
   1537.8 |
   1722.7 |
   1907.6 |########
   2092.5 |
   2277.4 |
   2462.3 |
   2647.2 |########
   2832.1 |################
   3017.0 |########################
   3201.9 |################################
   3386.8 |########################################
   3571.7 |################
   3756.6 |########################################
   3941.5 |
   4126.5 |########################################
   4311.4 |
   4496.3 |
   4681.2 |
  (4 below, 3 above range)

warm-container-native (n=40, range 1424.2-4305.3 ns)
   1424.2 |
   1568.3 |
   1712.3 |####
   1856.4 |
   2000.4 |
   2144.5 |
   2288.5 |
   2432.6 |########
   2576.6 |
   2720.7 |
   2864.7 |
   3008.8 |
   3152.8 |################
   3296.9 |
   3441.0 |############
   3585.0 |####
   3729.1 |####################################
   3873.1 |####
   4017.2 |########
   4161.2 |########################################
  (6 below, 1 above range)

warm-container-plusone (n=40, range 2168.9-8325.1 ns)
   2168.9 |#################################
   2476.7 |########################################
   2784.5 |###
   3092.3 |
   3400.1 |
   3707.9 |
   4015.8 |
   4323.6 |###
   4631.4 |
   4939.2 |
   5247.0 |
   5554.8 |
   5862.6 |
   6170.4 |
   6478.2 |###
   6786.0 |
   7093.9 |######
   7401.7 |#########
   7709.5 |
   8017.3 |######
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=64.4% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-minimum**: CV=41.5% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-native**: CV=31.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **warm-container-plusone**: CV=62.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
