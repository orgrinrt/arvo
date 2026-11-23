# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 2.77 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head dominates: 13% faster than the next best (warm-clamp-minimum)

warm-clamp-head (2.77 us) leads warm-clamp-minimum (3.14 us) by 13%, a clear separation rather than a photo finish. CV 22.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 24% (significant)

warm-clamp-head is -975 ns (24%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.61)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.1x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.33 us but its worst 20% at 5.40 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2768.4 ns median (-30.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.44x (fastest 2768.4 ns, slowest 3984.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761915 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3636ns | 4159ns | 1483ns | 3774ns | 5378ns | base |
| warm-clamp-accfit | 3449ns | 3336ns | 2368ns | 3445ns | 4540ns | -5.16% |
| warm-clamp-accfit-dyn | 3681ns | 3919ns | 1951ns | 3950ns | 4606ns | +1.24% |
| warm-clamp-head | 2956ns | 2962ns | 1997ns | 3035ns | 3676ns | -18.72% |
| warm-clamp-min-lanes | 3600ns | 4113ns | 1382ns | 3652ns | 5661ns | -1.01% |
| warm-clamp-minimum | 3303ns | 3316ns | 2682ns | 3309ns | 3908ns | -9.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3478ns | 1423ns | 5118ns | base | 2.355 |
| warm-clamp-accfit | 3256ns | 2243ns | 4270ns | -6.38% | 2.516 |
| warm-clamp-accfit-dyn | 3475ns | 1847ns | 4314ns | -0.09% | 2.357 |
| warm-clamp-head | 2751ns | 1862ns | 3396ns | -20.89% | 2.977 |
| warm-clamp-min-lanes | 3442ns | 1326ns | 5404ns | -1.04% | 2.380 |
| warm-clamp-minimum | 3121ns | 2542ns | 3667ns | -10.27% | 2.625 |

## Performance model

- Peak throughput: **6.178 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.056 | 33.3% |
| warm-clamp-accfit | 2.592 | 42.0% |
| warm-clamp-accfit-dyn | 2.207 | 35.7% |
| warm-clamp-head | 2.959 | 47.9% |
| warm-clamp-min-lanes | 2.084 | 33.7% |
| warm-clamp-minimum | 2.607 | 42.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3636ns | 3636ns | base |
| warm-clamp-accfit | 3449ns | 3449ns | -5.16% |
| warm-clamp-accfit-dyn | 3681ns | 3681ns | +1.24% |
| warm-clamp-head | 2956ns | 2956ns | -18.72% |
| warm-clamp-min-lanes | 3600ns | 3600ns | -1.01% |
| warm-clamp-minimum | 3303ns | 3303ns | -9.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3985ns | base | --- | [3223, 4321] | --- | --- | --- | --- |
| warm-clamp-accfit | 3161ns | -627.0ns (-15.7%) | [-983, -259]ns | [2916, 3444] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-accfit-dyn | 3713ns | -273.4ns (-6.9%) | [-371, -40]ns | [3438, 4080] | YES | 0.0481 | 0.0385 | 0 |
| warm-clamp-head | 2768ns | -975.3ns (-24.5%) | [-1401, -686]ns | [2689, 2951] | YES | 0.0111 | 0.0022 | 0 |
| warm-clamp-min-lanes | 3931ns | no significant difference | [-216, +195]ns | [2016, 4454] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-minimum | 3142ns | -873.0ns (-21.9%) | [-1043, -492]ns | [2935, 3334] | YES | 0.0276 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3491ns | -16.5% | -2.7% | -27.0% | -28.8% | -16.4% |
| 1 | 2 | 3851ns | -3.8% | +4.0% | -23.8% | +26.5% | -9.6% |
| 1 | 3 | 4449ns | -9.7% | -20.1% | -39.5% | -12.5% | -31.2% |
| 1 | 4 | 3985ns | -26.9% | +3.2% | -19.0% | +1.7% | -21.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.034 | ok |
| warm-clamp-accfit | -0.264 | moderate- |
| warm-clamp-accfit-dyn | -0.592 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.372 | moderate- |
| warm-clamp-min-lanes | -0.447 | moderate- |
| warm-clamp-minimum | -0.606 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 56.0ns | 3478.0ns | 1.6% |  |
| warm-clamp-accfit | 65.7ns | 3256.3ns | 2.0% |  |
| warm-clamp-accfit-dyn | 70.4ns | 3475.0ns | 2.0% |  |
| warm-clamp-head | 71.9ns | 2751.4ns | 2.6% |  |
| warm-clamp-min-lanes | 55.5ns | 3441.8ns | 1.6% |  |
| warm-clamp-minimum | 63.3ns | 3120.9ns | 2.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1422.7-5118.0 ns)
   1422.7 |########################################
   1607.5 |
   1792.2 |
   1977.0 |####
   2161.8 |
   2346.5 |
   2531.3 |
   2716.1 |
   2900.8 |########
   3085.6 |
   3270.3 |####
   3455.1 |#############
   3639.9 |
   3824.6 |#############
   4009.4 |########
   4194.2 |######################
   4378.9 |
   4563.7 |#################
   4748.5 |########
   4933.2 |########
  (2 below, 4 above range)

warm-clamp-accfit (n=40, range 2242.9-4269.8 ns)
   2242.9 |
   2344.2 |
   2445.6 |
   2546.9 |
   2648.3 |##########################
   2749.6 |##########################
   2851.0 |##########################
   2952.3 |
   3053.7 |#################################
   3155.0 |####################
   3256.4 |
   3357.7 |##########################
   3459.1 |
   3560.4 |
   3661.7 |####################
   3763.1 |
   3864.4 |######
   3965.8 |######
   4067.1 |########################################
   4168.5 |
  (3 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1847.5-4313.8 ns)
   1847.5 |
   1970.8 |
   2094.1 |
   2217.4 |
   2340.7 |###
   2464.1 |
   2587.4 |
   2710.7 |
   2834.0 |
   2957.3 |
   3080.6 |#######
   3203.9 |###
   3327.2 |#####################
   3450.6 |
   3573.9 |##########
   3697.2 |##################
   3820.5 |#######
   3943.8 |
   4067.1 |########################################
   4190.4 |###
  (5 below, 3 above range)

warm-clamp-head (n=40, range 1862.4-3395.9 ns)
   1862.4 |
   1939.1 |
   2015.7 |
   2092.4 |
   2169.1 |
   2245.8 |#####
   2322.4 |#####
   2399.1 |
   2475.8 |##################################
   2552.5 |#####
   2629.1 |########################################
   2705.8 |#####
   2782.5 |###########
   2859.1 |#################
   2935.8 |###########
   3012.5 |#####
   3089.2 |
   3165.8 |##################################
   3242.5 |#################
   3319.2 |
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1326.0-5404.2 ns)
   1326.0 |########################################
   1530.0 |#####
   1733.9 |
   1937.8 |
   2141.7 |
   2345.6 |#####
   2549.5 |
   2753.4 |
   2957.3 |
   3161.2 |
   3365.1 |#####
   3569.0 |#####
   3772.9 |####################
   3976.9 |###############
   4180.8 |#####
   4384.7 |###############
   4588.6 |#####
   4792.5 |###############
   4996.4 |
   5200.3 |##############################
  (5 below, 2 above range)

warm-clamp-minimum (n=40, range 2542.2-3666.6 ns)
   2542.2 |
   2598.5 |
   2654.7 |
   2710.9 |########################################
   2767.1 |######
   2823.3 |######
   2879.5 |#################################
   2935.8 |####################
   2992.0 |
   3048.2 |######
   3104.4 |##########################
   3160.6 |####################
   3216.8 |
   3273.1 |######
   3329.3 |#############
   3385.5 |#################################
   3441.7 |#############
   3497.9 |######
   3554.1 |######
   3610.4 |
  (1 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=40.7% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=23.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=27.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=22.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=46.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.1x (possible bimodal distribution)
