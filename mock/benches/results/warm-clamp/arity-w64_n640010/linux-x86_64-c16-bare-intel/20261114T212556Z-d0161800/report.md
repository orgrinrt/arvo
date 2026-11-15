# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn dominates: 49% faster than the next best (warm-clamp-head)

warm-clamp-accfit-dyn (2.05 us) leads warm-clamp-head (3.06 us) by 49%, a clear separation rather than a photo finish. CV 103.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum is an outlier: 2.2x slower than the field

warm-clamp-minimum (4.60 us) is 2.2x the fastest (2.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit-dyn is fastest but the noisiest (CV 103.2%)

warm-clamp-accfit-dyn wins on median (2.05 us) yet has the highest variance (CV 103.2%), while warm-clamp-acc64 is the steadiest (CV 35.6%, 4.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.65)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 3.8x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.84 us but its worst 20% at 7.04 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2047.8 ns median (-52.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.25x (fastest 2047.8 ns, slowest 4602.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4007ns | 4564ns | 1634ns | 4183ns | 5850ns | base |
| warm-clamp-accfit | 4087ns | 4436ns | 1946ns | 3867ns | 6887ns | +2.00% |
| warm-clamp-accfit-dyn | 3950ns | 2135ns | 1913ns | 3489ns | 7371ns | -1.41% |
| warm-clamp-head | 4654ns | 3144ns | 3136ns | 3408ns | 9912ns | +16.17% |
| warm-clamp-min-lanes | 3628ns | 4084ns | 1594ns | 3613ns | 5710ns | -9.44% |
| warm-clamp-minimum | 3897ns | 4856ns | 1603ns | 3965ns | 5984ns | -2.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3787ns | 1548ns | 5491ns | base | 2.163 |
| warm-clamp-accfit | 3907ns | 1864ns | 6566ns | +3.16% | 2.097 |
| warm-clamp-accfit-dyn | 3780ns | 1836ns | 7036ns | -0.19% | 2.167 |
| warm-clamp-head | 4520ns | 3053ns | 9596ns | +19.36% | 1.812 |
| warm-clamp-min-lanes | 3434ns | 1513ns | 5380ns | -9.34% | 2.386 |
| warm-clamp-minimum | 3679ns | 1517ns | 5612ns | -2.87% | 2.227 |

## Performance model

- Peak throughput: **5.416 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.895 | 35.0% |
| warm-clamp-accfit | 1.931 | 35.7% |
| warm-clamp-accfit-dyn | 4.000 | 73.9% |
| warm-clamp-head | 2.677 | 49.4% |
| warm-clamp-min-lanes | 2.114 | 39.0% |
| warm-clamp-minimum | 1.780 | 32.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4007ns | 4007ns | base |
| warm-clamp-accfit | 4087ns | 4087ns | +2.00% |
| warm-clamp-accfit-dyn | 3950ns | 3950ns | -1.41% |
| warm-clamp-head | 4654ns | 4654ns | +16.17% |
| warm-clamp-min-lanes | 3628ns | 3628ns | -9.44% |
| warm-clamp-minimum | 3897ns | 3897ns | -2.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4324ns | base | --- | [3665, 4663] | --- | --- | --- | --- |
| warm-clamp-accfit | 4241ns | no significant difference | [-125, +487]ns | [2020, 5104] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 2048ns | no significant difference | [-188, +490]ns | [2040, 4978] | no | 0.3352 | 0.1539 | 0 |
| warm-clamp-head | 3060ns | no significant difference | [-963, +1506]ns | [3054, 3067] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-min-lanes | 3875ns | -340.4ns (-7.9%) | [-501, -9]ns | [1848, 4348] | YES | 0.0321 | 0.0064 | 0 |
| warm-clamp-minimum | 4603ns | no significant difference | [-7, +242]ns | [1663, 5071] | no | 0.3352 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1556ns | +30.1% | +31.4% | +97.2% | +177.6% | +188.9% |
| 1 | 2 | 4366ns | +17.8% | -39.7% | -29.9% | -61.3% | -17.9% |
| 1 | 3 | 4661ns | -6.0% | -3.1% | -34.4% | -3.8% | +1.6% |
| 1 | 4 | 4310ns | -50.5% | -52.6% | -29.1% | -13.0% | -29.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.038 | ok |
| warm-clamp-accfit | -0.265 | moderate- |
| warm-clamp-accfit-dyn | -0.358 | moderate- |
| warm-clamp-head | -0.048 | ok |
| warm-clamp-min-lanes | -0.598 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.648 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 60.7ns | 3787.4ns | 1.6% |  |
| warm-clamp-accfit | 49.8ns | 3907.0ns | 1.3% |  |
| warm-clamp-accfit-dyn | 46.7ns | 3780.3ns | 1.2% |  |
| warm-clamp-head | 36.4ns | 4520.5ns | 0.8% |  |
| warm-clamp-min-lanes | 56.5ns | 3433.8ns | 1.6% |  |
| warm-clamp-minimum | 59.3ns | 3678.6ns | 1.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1548.2-5490.8 ns)
   1548.2 |########################################
   1745.3 |
   1942.4 |
   2139.6 |######
   2336.7 |
   2533.8 |
   2731.0 |
   2928.1 |######
   3125.2 |
   3322.4 |######
   3519.5 |
   3716.6 |
   3913.7 |####################
   4110.9 |#############
   4308.0 |####################
   4505.1 |#################################
   4702.3 |
   4899.4 |#################################
   5096.5 |#############
   5293.7 |
  (5 below, 6 above range)

warm-clamp-accfit (n=40, range 1863.8-6566.0 ns)
   1863.8 |########################################
   2098.9 |##
   2334.1 |
   2569.2 |
   2804.3 |
   3039.4 |
   3274.5 |
   3509.6 |
   3744.7 |
   3979.8 |##
   4214.9 |#####
   4450.0 |#####
   4685.2 |##
   4920.3 |#####
   5155.4 |
   5390.5 |
   5625.6 |########
   5860.7 |#####
   6095.8 |#####
   6330.9 |#####
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1835.5-7035.5 ns)
   1835.5 |########################################
   2095.5 |
   2355.5 |
   2615.5 |
   2875.5 |
   3135.5 |##
   3395.5 |
   3655.5 |
   3915.5 |#######
   4175.5 |
   4435.5 |
   4695.5 |##
   4955.5 |##
   5215.5 |####
   5475.5 |
   5735.5 |####
   5995.5 |##
   6255.5 |
   6515.5 |#######
   6775.5 |##
  (4 below, 4 above range)

warm-clamp-head (n=40, range 3053.1-9596.0 ns)
   3053.1 |########################################
   3380.2 |#
   3707.4 |
   4034.5 |
   4361.7 |
   4688.8 |
   5016.0 |
   5343.1 |
   5670.3 |#
   5997.4 |#
   6324.5 |
   6651.7 |
   6978.8 |
   7306.0 |
   7633.1 |
   7960.3 |
   8287.4 |
   8614.6 |####
   8941.7 |
   9268.8 |#
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1512.6-5379.5 ns)
   1512.6 |########################################
   1706.0 |######
   1899.3 |
   2092.7 |
   2286.0 |
   2479.3 |
   2672.7 |
   2866.0 |###
   3059.4 |
   3252.7 |
   3446.1 |######
   3639.4 |######
   3832.8 |######
   4026.1 |
   4219.5 |################
   4412.8 |
   4606.2 |#############
   4799.5 |###
   4992.8 |##########
   5186.2 |###
  (1 below, 4 above range)

warm-clamp-minimum (n=40, range 1517.2-5611.6 ns)
   1517.2 |########################################
   1721.9 |####
   1926.6 |
   2131.3 |####
   2336.1 |
   2540.8 |
   2745.5 |
   2950.2 |
   3155.0 |
   3359.7 |####
   3564.4 |
   3769.1 |
   3973.9 |
   4178.6 |########
   4383.3 |####
   4588.0 |############
   4792.8 |####
   4997.5 |################
   5202.2 |####################
   5406.9 |################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=40.6% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=50.1% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=55.9% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-head**: CV=58.3% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=45.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=47.9% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
