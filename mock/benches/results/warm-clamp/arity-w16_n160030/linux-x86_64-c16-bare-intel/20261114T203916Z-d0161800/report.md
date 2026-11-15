# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 2.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head beats baseline by 27% (significant)

warm-clamp-head is -960 ns (27%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is fastest but the noisiest (CV 103.5%)

warm-clamp-minimum wins on median (2.34 us) yet has the highest variance (CV 103.5%), while warm-clamp-head is the steadiest (CV 14.7%, 2.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.51)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.16 us) is smaller than the fastest variant's own run-to-run std-dev (2.42 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-min-lanes is inconsistent: worst-20% is 3.6x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 2.17 us but its worst 20% at 7.70 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2337.4 ns median (-33.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.50x (fastest 2337.4 ns, slowest 3500.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3573ns | 3677ns | 1740ns | 3680ns | 5083ns | base |
| warm-clamp-accfit | 3423ns | 3398ns | 2289ns | 3313ns | 4887ns | -4.19% |
| warm-clamp-accfit-dyn | 2924ns | 2875ns | 2077ns | 2949ns | 3696ns | -18.16% |
| warm-clamp-head | 3021ns | 2872ns | 2589ns | 2941ns | 3695ns | -15.43% |
| warm-clamp-min-lanes | 4459ns | 2629ns | 2225ns | 4040ns | 7949ns | +24.81% |
| warm-clamp-minimum | 4180ns | 2400ns | 2387ns | 3390ns | 8339ns | +16.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3386ns | 1655ns | 4802ns | base | 2.420 |
| warm-clamp-accfit | 3184ns | 2131ns | 4546ns | -5.96% | 2.573 |
| warm-clamp-accfit-dyn | 2696ns | 1928ns | 3352ns | -20.38% | 3.039 |
| warm-clamp-head | 2758ns | 2365ns | 3388ns | -18.54% | 2.971 |
| warm-clamp-min-lanes | 4314ns | 2166ns | 7699ns | +27.43% | 1.899 |
| warm-clamp-minimum | 4059ns | 2325ns | 8065ns | +19.88% | 2.018 |

## Performance model

- Peak throughput: **4.951 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.340 | 47.3% |
| warm-clamp-accfit | 2.591 | 52.3% |
| warm-clamp-accfit-dyn | 3.058 | 61.8% |
| warm-clamp-head | 3.145 | 63.5% |
| warm-clamp-min-lanes | 3.234 | 65.3% |
| warm-clamp-minimum | 3.505 | 70.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3573ns | 3573ns | base |
| warm-clamp-accfit | 3423ns | 3423ns | -4.19% |
| warm-clamp-accfit-dyn | 2924ns | 2924ns | -18.16% |
| warm-clamp-head | 3021ns | 3021ns | -15.43% |
| warm-clamp-min-lanes | 4459ns | 4459ns | +24.81% |
| warm-clamp-minimum | 4180ns | 4180ns | +16.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3501ns | base | --- | [3086, 3808] | --- | --- | --- | --- |
| warm-clamp-accfit | 3161ns | -351.6ns (-10.0%) | [-485, -44]ns | [2938, 3184] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 2679ns | -677.3ns (-19.3%) | [-826, -210]ns | [2532, 2900] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2605ns | -960.2ns (-27.4%) | [-1198, -421]ns | [2542, 2824] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2533ns | no significant difference | [-495, +1199]ns | [2427, 5602] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-minimum | 2337ns | no significant difference | [-625, +1071]ns | [2327, 3844] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4517ns | -39.4% | -49.7% | -29.0% | +20.1% | -48.2% |
| 1 | 2 | 3582ns | -11.2% | -20.1% | -31.6% | -32.0% | -34.9% |
| 1 | 3 | 3766ns | -9.3% | -18.3% | -25.0% | -33.1% | -16.3% |
| 1 | 4 | 2984ns | +6.2% | -3.6% | -14.0% | -21.6% | -21.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.127 | ok |
| warm-clamp-accfit | 0.029 | ok |
| warm-clamp-accfit-dyn | 0.040 | ok |
| warm-clamp-head | -0.507 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.093 | ok |
| warm-clamp-minimum | -0.421 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 64.5ns | 3385.6ns | 1.9% |  |
| warm-clamp-accfit | 81.8ns | 3183.8ns | 2.6% |  |
| warm-clamp-accfit-dyn | 78.1ns | 2695.7ns | 2.9% |  |
| warm-clamp-head | 89.0ns | 2757.7ns | 3.2% |  |
| warm-clamp-min-lanes | 48.8ns | 4314.2ns | 1.1% |  |
| warm-clamp-minimum | 41.1ns | 4058.5ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1654.7-4802.4 ns)
   1654.7 |
   1812.1 |
   1969.4 |
   2126.8 |
   2284.2 |
   2441.6 |
   2599.0 |
   2756.4 |################################
   2913.8 |################################
   3071.2 |################
   3228.6 |################
   3385.9 |########################################
   3543.3 |########
   3700.7 |########################
   3858.1 |########
   4015.5 |################
   4172.9 |################
   4330.3 |########
   4487.7 |########################
   4645.0 |########
  (6 below, 3 above range)

warm-clamp-accfit (n=40, range 2130.7-4545.6 ns)
   2130.7 |
   2251.4 |###
   2372.2 |#######
   2492.9 |
   2613.6 |##########
   2734.4 |###
   2855.1 |#########################
   2975.9 |#######
   3096.6 |########################################
   3217.4 |##############
   3338.1 |###
   3458.9 |#######
   3579.6 |
   3700.4 |
   3821.1 |###
   3941.9 |###
   4062.6 |
   4183.3 |
   4304.1 |
   4424.8 |###
  (2 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 1927.7-3351.7 ns)
   1927.7 |
   1998.9 |
   2070.1 |################
   2141.3 |########
   2212.5 |########################
   2283.7 |################
   2354.9 |
   2426.1 |########################
   2497.3 |################
   2568.5 |################
   2639.7 |################################
   2710.9 |
   2782.1 |########
   2853.3 |########################################
   2924.5 |########
   2995.7 |########
   3066.9 |################
   3138.1 |################
   3209.3 |########################
   3280.5 |########
  (2 below, 3 above range)

warm-clamp-head (n=40, range 2364.8-3387.8 ns)
   2364.8 |###
   2416.0 |###
   2467.1 |###
   2518.3 |########################################
   2569.4 |##########
   2620.6 |###
   2671.7 |
   2722.9 |#######
   2774.0 |#######
   2825.2 |#######
   2876.3 |##########
   2927.5 |###
   2978.6 |
   3029.8 |###
   3080.9 |#######
   3132.1 |
   3183.2 |
   3234.3 |
   3285.5 |###
   3336.6 |
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2166.5-7699.4 ns)
   2166.5 |########################################
   2443.1 |#########################
   2719.8 |
   2996.4 |
   3273.1 |
   3549.7 |
   3826.4 |
   4103.0 |###
   4379.7 |
   4656.3 |
   4933.0 |###
   5209.6 |#######
   5486.3 |#######
   5762.9 |###
   6039.5 |##########
   6316.2 |###
   6592.8 |###
   6869.5 |###
   7146.1 |###
   7422.8 |###
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 2324.7-8064.7 ns)
   2324.7 |########################################
   2611.7 |
   2898.7 |
   3185.7 |
   3472.7 |#
   3759.7 |#
   4046.7 |
   4333.7 |
   4620.7 |
   4907.7 |#
   5194.7 |
   5481.7 |
   5768.7 |#
   6055.7 |
   6342.7 |#
   6629.7 |
   6916.7 |###
   7203.7 |#
   7490.7 |#
   7777.7 |#
  (4 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=32.9% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=40.6% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=52.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=59.6% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.5x (possible bimodal distribution)
