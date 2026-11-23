# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is fastest but the noisiest (CV 106.5%)

warm-clamp-minimum wins on median (2.40 us) yet has the highest variance (CV 106.5%), while warm-clamp-accfit is the steadiest (CV 33.3%, 3.82 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-minimum, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-minimum (2.40 us) and warm-clamp-min-lanes (2.40 us) differ by 0.02%, inside the noise, even though the wider field spreads 58.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.77)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.42 us) is smaller than the fastest variant's own run-to-run std-dev (2.56 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-accfit} (41% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-accfit} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.28 us but its worst 20% at 5.54 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2401.4 ns median (-35.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.59x (fastest 2401.4 ns, slowest 3816.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112755372 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3566ns | 3864ns | 1329ns | 3566ns | 5805ns | base |
| warm-clamp-accfit | 3677ns | 4010ns | 1489ns | 3876ns | 5270ns | +3.11% |
| warm-clamp-accfit-dyn | 3537ns | 3794ns | 1325ns | 3585ns | 5606ns | -0.82% |
| warm-clamp-head | 3322ns | 3595ns | 1201ns | 3549ns | 4760ns | -6.86% |
| warm-clamp-min-lanes | 3878ns | 2465ns | 2014ns | 2980ns | 8437ns | +8.75% |
| warm-clamp-minimum | 4162ns | 2466ns | 2015ns | 3435ns | 8488ns | +16.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3411ns | 1275ns | 5541ns | base | 2.401 |
| warm-clamp-accfit | 3518ns | 1429ns | 5018ns | +3.12% | 2.329 |
| warm-clamp-accfit-dyn | 3385ns | 1270ns | 5333ns | -0.77% | 2.420 |
| warm-clamp-head | 3138ns | 1138ns | 4470ns | -8.02% | 2.611 |
| warm-clamp-min-lanes | 3768ns | 1963ns | 8163ns | +10.44% | 2.174 |
| warm-clamp-minimum | 4042ns | 1964ns | 8208ns | +18.49% | 2.027 |

## Performance model

- Peak throughput: **7.196 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.207 | 30.7% |
| warm-clamp-accfit | 2.146 | 29.8% |
| warm-clamp-accfit-dyn | 2.247 | 31.2% |
| warm-clamp-head | 2.416 | 33.6% |
| warm-clamp-min-lanes | 3.410 | 47.4% |
| warm-clamp-minimum | 3.411 | 47.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3566ns | 3566ns | base |
| warm-clamp-accfit | 3677ns | 3677ns | +3.11% |
| warm-clamp-accfit-dyn | 3537ns | 3537ns | -0.82% |
| warm-clamp-head | 3322ns | 3322ns | -6.86% |
| warm-clamp-min-lanes | 3878ns | 3878ns | +8.75% |
| warm-clamp-minimum | 4162ns | 4162ns | +16.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3713ns | base | --- | [2480, 4273] | --- | --- | --- | --- |
| warm-clamp-accfit | 3817ns | no significant difference | [-770, +688]ns | [3284, 4287] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-accfit-dyn | 3645ns | -349.2ns (-9.4%) | [-533, -125]ns | [2901, 4245] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-head | 3391ns | -393.7ns (-10.6%) | [-658, -8]ns | [3093, 3853] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-clamp-min-lanes | 2402ns | no significant difference | [-293, +867]ns | [2187, 2415] | no | 0.1009 | 0.0807 | 0 |
| warm-clamp-minimum | 2401ns | +429.8ns (+11.6%) | [+311, +1221]ns | [2399, 4736] | YES (adj: no) | 0.0641 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3958ns | -16.0% | -7.9% | -21.9% | -50.4% | -50.2% |
| 1 | 2 | 4780ns | -18.6% | -8.4% | -16.0% | -58.9% | -48.9% |
| 1 | 3 | 1540ns | +188.7% | +31.7% | -25.5% | +56.8% | +27.6% |
| 1 | 4 | 3640ns | +19.0% | -44.3% | +2.8% | -33.8% | -33.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.387 | moderate- |
| warm-clamp-accfit | 0.224 | moderate+ |
| warm-clamp-accfit-dyn | 0.116 | ok |
| warm-clamp-head | -0.629 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | 0.247 | moderate+ |
| warm-clamp-minimum | -0.769 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 54.1ns | 3411.5ns | 1.6% |  |
| warm-clamp-accfit | 56.7ns | 3518.0ns | 1.6% |  |
| warm-clamp-accfit-dyn | 52.1ns | 3385.1ns | 1.5% |  |
| warm-clamp-head | 65.0ns | 3138.0ns | 2.1% |  |
| warm-clamp-min-lanes | 36.8ns | 3767.6ns | 1.0% |  |
| warm-clamp-minimum | 39.0ns | 4042.2ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1275.2-5540.6 ns)
   1275.2 |######
   1488.4 |########################################
   1701.7 |
   1915.0 |######
   2128.2 |
   2341.5 |
   2554.8 |
   2768.1 |######
   2981.3 |
   3194.6 |######
   3407.9 |####################
   3621.2 |####################
   3834.4 |#############
   4047.7 |#############
   4261.0 |#############
   4474.2 |#############
   4687.5 |
   4900.8 |#############
   5114.1 |####################
   5327.3 |#############
  (6 below, 3 above range)

warm-clamp-accfit (n=40, range 1429.1-5017.5 ns)
   1429.1 |####################
   1608.5 |
   1787.9 |
   1967.3 |######
   2146.7 |######
   2326.2 |######
   2505.6 |
   2685.0 |
   2864.4 |
   3043.9 |######
   3223.3 |########################################
   3402.7 |######
   3582.1 |######
   3761.6 |##########################
   3941.0 |######
   4120.4 |#############
   4299.8 |#################################
   4479.3 |####################
   4658.7 |
   4838.1 |#############
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1269.7-5333.2 ns)
   1269.7 |
   1472.9 |########################################
   1676.1 |
   1879.2 |
   2082.4 |
   2285.6 |
   2488.8 |########
   2691.9 |########
   2895.1 |########
   3098.3 |########
   3301.5 |################
   3504.6 |########################
   3707.8 |################
   3911.0 |################
   4114.2 |################
   4317.3 |################
   4520.5 |################
   4723.7 |################
   4926.9 |########
   5130.0 |########
  (7 below, 5 above range)

warm-clamp-head (n=40, range 1138.5-4470.2 ns)
   1138.5 |########################
   1305.1 |
   1471.7 |
   1638.2 |
   1804.8 |
   1971.4 |
   2138.0 |
   2304.6 |
   2471.2 |
   2637.8 |########
   2804.3 |################
   2970.9 |################
   3137.5 |################################
   3304.1 |########################################
   3470.7 |########
   3637.3 |################
   3803.9 |################
   3970.5 |################################
   4137.0 |################################
   4303.6 |
  (6 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1962.7-8163.0 ns)
   1962.7 |##############################
   2272.7 |########################################
   2582.8 |###
   2892.8 |
   3202.8 |
   3512.8 |
   3822.8 |
   4132.8 |
   4442.8 |
   4752.9 |
   5062.9 |
   5372.9 |######
   5682.9 |
   5992.9 |###
   6302.9 |
   6612.9 |
   6923.0 |######
   7233.0 |###
   7543.0 |###
   7853.0 |###
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 1963.8-8208.4 ns)
   1963.8 |##############################
   2276.1 |########################################
   2588.3 |
   2900.5 |
   3212.7 |
   3525.0 |
   3837.2 |
   4149.4 |###
   4461.7 |
   4773.9 |
   5086.1 |######
   5398.3 |###
   5710.6 |
   6022.8 |
   6335.0 |
   6647.3 |###
   6959.5 |######
   7271.7 |
   7583.9 |##########
   7896.2 |###
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=47.6% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=36.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=44.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-head**: CV=38.3% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=66.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=63.3% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.2x (possible bimodal distribution)
