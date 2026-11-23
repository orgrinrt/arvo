# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 35% (significant)

warm-clamp-min-lanes is -856 ns (35%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-clamp-accfit, warm-clamp-accfit-dyn) are a dead heat (<1%)

warm-clamp-accfit (2.22 us) and warm-clamp-accfit-dyn (2.23 us) differ by 0.61%, inside the noise, even though the wider field spreads 53.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.77)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.18 us) is smaller than the fastest variant's own run-to-run std-dev (2.08 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-minimum} vs {warm-clamp-head, warm-clamp-min-lanes} (25% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-minimum} and a slow tier {warm-clamp-head, warm-clamp-min-lanes} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.30 us but its worst 20% at 5.60 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 2217.2 ns median (-9.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.53x (fastest 2217.2 ns, slowest 3394.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112579971 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4348ns | 2525ns | 2224ns | 3737ns | 8304ns | base |
| warm-clamp-accfit | 3789ns | 2300ns | 1867ns | 3385ns | 6924ns | -12.85% |
| warm-clamp-accfit-dyn | 3898ns | 2313ns | 1881ns | 3619ns | 6752ns | -10.35% |
| warm-clamp-head | 4347ns | 3167ns | 2589ns | 3394ns | 8965ns | -0.02% |
| warm-clamp-min-lanes | 3455ns | 3570ns | 1365ns | 3325ns | 5936ns | -20.54% |
| warm-clamp-minimum | 3996ns | 2541ns | 2102ns | 3433ns | 7579ns | -8.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 4189ns | 2147ns | 7963ns | base | 1.956 |
| warm-clamp-accfit | 3641ns | 1798ns | 6621ns | -13.09% | 2.250 |
| warm-clamp-accfit-dyn | 3744ns | 1813ns | 6447ns | -10.62% | 2.188 |
| warm-clamp-head | 4222ns | 2520ns | 8680ns | +0.81% | 1.940 |
| warm-clamp-min-lanes | 3271ns | 1297ns | 5597ns | -21.91% | 2.504 |
| warm-clamp-minimum | 3853ns | 2033ns | 7276ns | -8.02% | 2.126 |

## Performance model

- Peak throughput: **6.317 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.359 | 53.2% |
| warm-clamp-accfit | 3.695 | 58.5% |
| warm-clamp-accfit-dyn | 3.672 | 58.1% |
| warm-clamp-head | 2.657 | 42.1% |
| warm-clamp-min-lanes | 2.413 | 38.2% |
| warm-clamp-minimum | 3.333 | 52.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4348ns | 4348ns | base |
| warm-clamp-accfit | 3789ns | 3789ns | -12.85% |
| warm-clamp-accfit-dyn | 3898ns | 3898ns | -10.35% |
| warm-clamp-head | 4347ns | 4347ns | -0.02% |
| warm-clamp-min-lanes | 3455ns | 3455ns | -20.54% |
| warm-clamp-minimum | 3996ns | 3996ns | -8.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2439ns | base | --- | [2406, 5136] | --- | --- | --- | --- |
| warm-clamp-accfit | 2217ns | -599.6ns (-24.6%) | [-628, -188]ns | [1814, 5289] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2231ns | no significant difference | [-653, +93]ns | [2203, 5188] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-head | 3083ns | +107.7ns (+4.4%) | [+90, +771]ns | [3075, 3093] | YES | 0.0107 | 0.0064 | 0 |
| warm-clamp-min-lanes | 3394ns | -855.6ns (-35.1%) | [-1115, -718]ns | [1584, 4515] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-minimum | 2458ns | no significant difference | [-382, +44]ns | [2346, 4351] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2317ns | -3.6% | +148.8% | +33.3% | +60.8% | +1.8% |
| 1 | 2 | 2456ns | -26.1% | +97.0% | +3.0% | -18.2% | -15.9% |
| 1 | 3 | 2413ns | +39.9% | +55.6% | +4.9% | -6.8% | +2.4% |
| 1 | 4 | 4960ns | -63.3% | -55.4% | -37.8% | -21.7% | -59.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.085 | ok |
| warm-clamp-accfit | -0.622 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | 0.230 | moderate+ |
| warm-clamp-head | -0.250 | moderate- |
| warm-clamp-min-lanes | -0.245 | moderate- |
| warm-clamp-minimum | -0.771 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 40.9ns | 4188.7ns | 1.0% |  |
| warm-clamp-accfit | 38.5ns | 3640.6ns | 1.1% |  |
| warm-clamp-accfit-dyn | 38.8ns | 3744.1ns | 1.0% |  |
| warm-clamp-head | 32.3ns | 4222.4ns | 0.8% |  |
| warm-clamp-min-lanes | 48.9ns | 3270.9ns | 1.5% |  |
| warm-clamp-minimum | 36.9ns | 3852.9ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2147.3-7963.2 ns)
   2147.3 |########################################
   2438.1 |#######
   2728.9 |
   3019.7 |
   3310.5 |##
   3601.3 |
   3892.1 |##
   4182.9 |
   4473.7 |
   4764.5 |##
   5055.3 |##
   5346.1 |
   5636.9 |##
   5927.7 |
   6218.5 |##
   6509.3 |#####
   6800.1 |
   7090.9 |#######
   7381.7 |
   7672.5 |#####
  (4 below, 4 above range)

warm-clamp-accfit (n=40, range 1798.1-6620.8 ns)
   1798.1 |########################################
   2039.2 |####################
   2280.3 |
   2521.5 |
   2762.6 |
   3003.7 |
   3244.9 |
   3486.0 |
   3727.1 |
   3968.3 |
   4209.4 |###
   4450.5 |
   4691.7 |###
   4932.8 |###
   5174.0 |
   5415.1 |######
   5656.2 |######
   5897.4 |#############
   6138.5 |###
   6379.6 |######
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1812.7-6446.8 ns)
   1812.7 |############
   2044.4 |########################################
   2276.1 |###
   2507.8 |
   2739.5 |
   2971.2 |
   3202.9 |
   3434.6 |
   3666.3 |
   3898.0 |
   4129.7 |
   4361.4 |
   4593.1 |###
   4824.8 |######
   5056.5 |######
   5288.2 |#########
   5519.9 |######
   5751.6 |###
   5983.4 |#########
   6215.1 |
  (4 below, 4 above range)

warm-clamp-head (n=40, range 2520.5-8679.6 ns)
   2520.5 |#####################
   2828.4 |########################################
   3136.4 |
   3444.3 |#####
   3752.3 |
   4060.3 |
   4368.2 |
   4676.2 |
   4984.1 |
   5292.1 |
   5600.0 |
   5908.0 |##
   6216.0 |
   6523.9 |
   6831.9 |
   7139.8 |##
   7447.8 |
   7755.8 |########
   8063.7 |##
   8371.7 |##
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1296.9-5596.7 ns)
   1296.9 |####
   1511.9 |########################################
   1726.9 |
   1941.9 |
   2156.9 |
   2371.9 |####
   2586.8 |
   2801.8 |####
   3016.8 |####
   3231.8 |
   3446.8 |####
   3661.8 |####
   3876.8 |########
   4091.8 |####
   4306.8 |####
   4521.8 |########
   4736.8 |########
   4951.8 |########
   5166.8 |########
   5381.7 |####
  (7 below, 5 above range)

warm-clamp-minimum (n=40, range 2033.3-7275.8 ns)
   2033.3 |#######################
   2295.5 |########################################
   2557.6 |
   2819.7 |
   3081.8 |
   3344.0 |
   3606.1 |
   3868.2 |###
   4130.3 |
   4392.5 |
   4654.6 |###
   4916.7 |###
   5178.8 |
   5441.0 |
   5703.1 |
   5965.2 |
   6227.3 |###
   6489.5 |###
   6751.6 |##########
   7013.7 |##########
  (6 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=56.9% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=57.1% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=51.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-head**: CV=57.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=52.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=57.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
