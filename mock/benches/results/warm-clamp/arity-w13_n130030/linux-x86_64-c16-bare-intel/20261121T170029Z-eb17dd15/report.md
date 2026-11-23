# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.65 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 2.33 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head beats baseline by 26% (significant)

warm-clamp-head is -940 ns (26%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-clamp-minimum, warm-clamp-accfit) are a dead heat (<1%)

warm-clamp-minimum (2.33 us) and warm-clamp-accfit (2.33 us) differ by 0.01%, inside the noise, even though the wider field spreads 56.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.53)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.31 us) is smaller than the fastest variant's own run-to-run std-dev (2.16 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-acc64} (39% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-acc64} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.0x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 2.02 us but its worst 20% at 8.12 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2334.0 ns median (-36.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.56x (fastest 2334.0 ns, slowest 3647.2 ns)

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
| warm-clamp-acc64 | 3682ns | 3827ns | 1980ns | 3860ns | 4851ns | base |
| warm-clamp-accfit | 4079ns | 2397ns | 1960ns | 3567ns | 7734ns | +10.78% |
| warm-clamp-accfit-dyn | 4156ns | 2403ns | 2075ns | 3442ns | 8381ns | +12.87% |
| warm-clamp-head | 2724ns | 2773ns | 1671ns | 2750ns | 3698ns | -26.03% |
| warm-clamp-min-lanes | 3954ns | 3671ns | 1981ns | 3359ns | 7712ns | +7.38% |
| warm-clamp-minimum | 3835ns | 2397ns | 1961ns | 3289ns | 7348ns | +4.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3498ns | 1886ns | 4580ns | base | 2.342 |
| warm-clamp-accfit | 3960ns | 1909ns | 7473ns | +13.21% | 2.069 |
| warm-clamp-accfit-dyn | 4036ns | 2021ns | 8121ns | +15.38% | 2.030 |
| warm-clamp-head | 2491ns | 1534ns | 3363ns | -28.78% | 3.288 |
| warm-clamp-min-lanes | 3841ns | 1929ns | 7467ns | +9.79% | 2.133 |
| warm-clamp-minimum | 3723ns | 1909ns | 7097ns | +6.42% | 2.201 |

## Performance model

- Peak throughput: **5.340 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.246 | 42.1% |
| warm-clamp-accfit | 3.509 | 65.7% |
| warm-clamp-accfit-dyn | 3.501 | 65.6% |
| warm-clamp-head | 3.217 | 60.3% |
| warm-clamp-min-lanes | 2.312 | 43.3% |
| warm-clamp-minimum | 3.510 | 65.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3682ns | 3682ns | base |
| warm-clamp-accfit | 4079ns | 4079ns | +10.78% |
| warm-clamp-accfit-dyn | 4156ns | 4156ns | +12.87% |
| warm-clamp-head | 2724ns | 2724ns | -26.03% |
| warm-clamp-min-lanes | 3954ns | 3954ns | +7.38% |
| warm-clamp-minimum | 3835ns | 3835ns | +4.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3647ns | base | --- | [3493, 3847] | --- | --- | --- | --- |
| warm-clamp-accfit | 2334ns | no significant difference | [-1157, +1679]ns | [2332, 4966] | no | 1.0000 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 2340ns | no significant difference | [-1069, +1171]ns | [2333, 4293] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-head | 2546ns | -940.0ns (-25.8%) | [-1243, -726]ns | [2425, 2595] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3543ns | no significant difference | [-296, +748]ns | [2358, 4042] | no | 1.0000 | 0.4296 | 0 |
| warm-clamp-minimum | 2334ns | no significant difference | [-968, +853]ns | [2332, 3789] | no | 1.0000 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3309ns | +50.1% | -29.3% | -23.1% | +18.4% | -29.3% |
| 1 | 2 | 4418ns | -47.2% | -47.0% | -49.3% | -56.2% | -56.5% |
| 1 | 3 | 3782ns | -49.4% | -49.1% | -17.5% | -48.8% | -49.5% |
| 1 | 4 | 3775ns | -38.0% | -37.9% | -31.0% | -37.3% | -33.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.526 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.014 | ok |
| warm-clamp-accfit-dyn | -0.424 | moderate- |
| warm-clamp-head | -0.428 | moderate- |
| warm-clamp-min-lanes | -0.137 | ok |
| warm-clamp-minimum | -0.241 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 66.7ns | 3498.1ns | 1.9% |  |
| warm-clamp-accfit | 39.2ns | 3960.2ns | 1.0% |  |
| warm-clamp-accfit-dyn | 39.8ns | 4036.1ns | 1.0% |  |
| warm-clamp-head | 83.9ns | 2491.1ns | 3.4% |  |
| warm-clamp-min-lanes | 38.2ns | 3840.5ns | 1.0% |  |
| warm-clamp-minimum | 37.1ns | 3722.6ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1885.6-4580.3 ns)
   1885.6 |
   2020.3 |######
   2155.1 |
   2289.8 |
   2424.5 |
   2559.3 |
   2694.0 |
   2828.7 |
   2963.5 |#############
   3098.2 |######
   3232.9 |##########################
   3367.7 |##########################
   3502.4 |####################
   3637.1 |######
   3771.9 |########################################
   3906.6 |
   4041.3 |####################
   4176.1 |####################
   4310.8 |
   4445.6 |#################################
  (5 below, 2 above range)

warm-clamp-accfit (n=40, range 1908.7-7473.3 ns)
   1908.7 |##################
   2186.9 |########################################
   2465.2 |
   2743.4 |
   3021.6 |
   3299.9 |###
   3578.1 |
   3856.3 |###
   4134.6 |###
   4412.8 |
   4691.0 |
   4969.2 |
   5247.5 |
   5525.7 |###
   5803.9 |###
   6082.2 |###
   6360.4 |######
   6638.6 |######
   6916.9 |#########
   7195.1 |###
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2020.6-8121.2 ns)
   2020.6 |
   2325.6 |########################################
   2630.7 |
   2935.7 |
   3240.7 |##
   3545.8 |
   3850.8 |
   4155.8 |
   4460.9 |
   4765.9 |
   5070.9 |##
   5376.0 |
   5681.0 |##
   5986.0 |##
   6291.1 |##
   6596.1 |##
   6901.1 |##
   7206.1 |##
   7511.2 |##
   7816.2 |##
  (6 below, 5 above range)

warm-clamp-head (n=40, range 1534.2-3363.3 ns)
   1534.2 |
   1625.7 |###
   1717.1 |
   1808.6 |
   1900.0 |
   1991.5 |
   2082.9 |###############
   2174.4 |
   2265.8 |############
   2357.3 |###
   2448.8 |######
   2540.2 |########################################
   2631.7 |###
   2723.1 |
   2814.6 |######
   2906.0 |###
   2997.5 |######
   3088.9 |
   3180.4 |###
   3271.8 |###
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1929.2-7466.6 ns)
   1929.2 |########################################
   2206.1 |########################################
   2483.0 |
   2759.9 |
   3036.7 |
   3313.6 |###########
   3590.5 |###########
   3867.3 |##################################
   4144.2 |#####
   4421.1 |
   4697.9 |#####
   4974.8 |
   5251.7 |
   5528.6 |
   5805.4 |
   6082.3 |
   6359.2 |###########
   6636.0 |###########
   6912.9 |###########
   7189.8 |
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 1909.1-7096.7 ns)
   1909.1 |##########################
   2168.5 |########################################
   2427.9 |###
   2687.3 |
   2946.6 |
   3206.0 |
   3465.4 |###
   3724.8 |###
   3984.2 |
   4243.6 |
   4502.9 |
   4762.3 |
   5021.7 |
   5281.1 |###
   5540.5 |
   5799.8 |
   6059.2 |###
   6318.6 |##########
   6578.0 |######
   6837.4 |######
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=27.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=57.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=61.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-head**: CV=27.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=55.0% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=58.0% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
