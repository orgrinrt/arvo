# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (280 ns) is smaller than the fastest variant's own run-to-run std-dev (815 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-ragged-overread is inconsistent: worst-20% is 4.3x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 872 ns but its worst 20% at 3.71 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround** at 2832.6 ns median (-4.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.10x (fastest 2832.6 ns, slowest 3113.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112830321 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2828ns | 3154ns | 1084ns | 2902ns | 4353ns | base |
| wide-rung-ragged | 2850ns | 3336ns | 934ns | 3112ns | 3981ns | +0.77% |
| wide-rung-ragged-overread | 2778ns | 3116ns | 930ns | 2984ns | 4006ns | -1.79% |
| wide-rung-wordround | 2906ns | 3074ns | 1406ns | 3081ns | 3877ns | +2.73% |
| wide-rung-wordround-alias | 2845ns | 3286ns | 926ns | 3134ns | 3898ns | +0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2634ns | 1012ns | 4028ns | base | 0.777 |
| wide-rung-ragged | 2650ns | 872ns | 3673ns | +0.59% | 0.773 |
| wide-rung-ragged-overread | 2587ns | 872ns | 3709ns | -1.78% | 0.792 |
| wide-rung-wordround | 2671ns | 1298ns | 3551ns | +1.39% | 0.767 |
| wide-rung-wordround-alias | 2610ns | 853ns | 3533ns | -0.91% | 0.785 |

## Performance model

- Peak throughput: **2.400 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.694 | 28.9% |
| wide-rung-ragged | 0.658 | 27.4% |
| wide-rung-ragged-overread | 0.704 | 29.3% |
| wide-rung-wordround | 0.723 | 30.1% |
| wide-rung-wordround-alias | 0.675 | 28.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2828ns | 2828ns | base |
| wide-rung-ragged | 2850ns | 2850ns | +0.77% |
| wide-rung-ragged-overread | 2778ns | 2778ns | -1.79% |
| wide-rung-wordround | 2906ns | 2906ns | +2.73% |
| wide-rung-wordround-alias | 2845ns | 2845ns | +0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2949ns | base | --- | [2044, 3489] | --- | --- | --- | --- |
| wide-rung-ragged | 3113ns | no significant difference | [-410, +255]ns | [2839, 3409] | no | 0.1539 | 0.1539 | 0 |
| wide-rung-ragged-overread | 2909ns | no significant difference | [-440, +40]ns | [2667, 3162] | no | 0.1539 | 0.1539 | 0 |
| wide-rung-wordround | 2833ns | -282.8ns (-9.6%) | [-396, -170]ns | [2665, 2943] | YES | 0.0257 | 0.0064 | 0 |
| wide-rung-wordround-alias | 3034ns | no significant difference | [-239, +58]ns | [2872, 3209] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 2867ns | +12.5% | +2.7% | -9.2% | +10.8% |
| 1 | 2 | 3426ns | +2.8% | -18.4% | +1.2% | -15.1% |
| 1 | 3 | 3500ns | -16.8% | -23.3% | -19.6% | -10.3% |
| 1 | 4 | 966ns | +174.3% | +251.8% | +117.8% | +34.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.159 | ok |
| wide-rung-ragged | 0.145 | ok |
| wide-rung-ragged-overread | -0.253 | moderate- |
| wide-rung-wordround | -0.103 | ok |
| wide-rung-wordround-alias | -0.159 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 3/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 54.1ns | 2634.2ns | 2.1% |  |
| wide-rung-ragged | 63.9ns | 2649.9ns | 2.4% |  |
| wide-rung-ragged-overread | 64.4ns | 2587.2ns | 2.5% |  |
| wide-rung-wordround | 63.9ns | 2670.9ns | 2.4% |  |
| wide-rung-wordround-alias | 62.6ns | 2610.4ns | 2.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1012.3-4028.2 ns)
   1012.3 |
   1163.1 |########################################
   1313.9 |######
   1464.7 |
   1615.5 |
   1766.3 |######
   1917.1 |
   2067.9 |
   2218.7 |######
   2369.5 |
   2520.3 |
   2671.1 |####################
   2821.9 |#############
   2972.7 |#############
   3123.5 |####################
   3274.3 |
   3425.0 |##########################
   3575.8 |#############
   3726.6 |####################
   3877.4 |#############
  (6 below, 4 above range)

wide-rung-ragged (n=40, range 872.5-3672.8 ns)
    872.5 |##########################
   1012.5 |
   1152.5 |
   1292.5 |
   1432.5 |
   1572.5 |####
   1712.6 |
   1852.6 |
   1992.6 |
   2132.6 |####
   2272.6 |
   2412.7 |
   2552.7 |
   2692.7 |########
   2832.7 |#############
   2972.7 |#############
   3112.8 |#############
   3252.8 |########
   3392.8 |########################################
   3532.8 |#############
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 871.7-3709.2 ns)
    871.7 |#################################
   1013.6 |
   1155.5 |######
   1297.3 |
   1439.2 |
   1581.1 |
   1723.0 |
   1864.8 |######
   2006.7 |######
   2148.6 |
   2290.4 |
   2432.3 |
   2574.2 |##########################
   2716.1 |#############
   2857.9 |#################################
   2999.8 |####################
   3141.7 |######
   3283.6 |#############
   3425.4 |########################################
   3567.3 |######
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 1298.0-3551.4 ns)
   1298.0 |#####
   1410.6 |
   1523.3 |
   1636.0 |
   1748.6 |
   1861.3 |
   1974.0 |
   2086.7 |
   2199.3 |#####
   2312.0 |#####
   2424.7 |##########
   2537.3 |#####
   2650.0 |########################################
   2762.7 |#####
   2875.4 |###################################
   2988.0 |#####
   3100.7 |##########
   3213.4 |###############
   3326.1 |
   3438.7 |####################
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 853.4-3533.4 ns)
    853.4 |################
    987.4 |
   1121.4 |
   1255.4 |
   1389.4 |
   1523.4 |####
   1657.4 |
   1791.4 |####
   1925.4 |
   2059.4 |
   2193.4 |
   2327.4 |
   2461.4 |
   2595.4 |####
   2729.4 |########
   2863.4 |####################
   2997.4 |############
   3131.4 |####################
   3265.4 |########
   3399.4 |########################################
  (5 below, 1 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=44.9% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **wide-rung-ragged**: CV=41.2% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=40.7% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **wide-rung-wordround**: CV=30.5% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: CV=39.4% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 4.1x (possible bimodal distribution)
