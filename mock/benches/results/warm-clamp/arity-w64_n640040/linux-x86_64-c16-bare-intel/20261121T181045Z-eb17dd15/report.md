# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.87 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-accfit-dyn at 2.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit-dyn dominates: 31% faster than the next best (warm-clamp-accfit)

warm-clamp-accfit-dyn (2.16 us) leads warm-clamp-accfit (2.82 us) by 31%, a clear separation rather than a photo finish. CV 90.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit-dyn is fastest but the noisiest (CV 90.5%)

warm-clamp-accfit-dyn wins on median (2.16 us) yet has the highest variance (CV 90.5%), while warm-clamp-acc64 is the steadiest (CV 39.2%, 3.87 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.50)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.71 us) is smaller than the fastest variant's own run-to-run std-dev (1.96 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.7x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.26 us but its worst 20% at 5.96 us (4.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2164.2 ns median (-44.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.79x (fastest 2164.2 ns, slowest 3870.6 ns)

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
| warm-clamp-acc64 | 3686ns | 4078ns | 1493ns | 3704ns | 5823ns | base |
| warm-clamp-accfit | 3639ns | 2963ns | 1692ns | 3311ns | 6571ns | -1.26% |
| warm-clamp-accfit-dyn | 3561ns | 2286ns | 1330ns | 3395ns | 6290ns | -3.39% |
| warm-clamp-head | 4134ns | 3138ns | 2572ns | 3361ns | 8014ns | +12.16% |
| warm-clamp-min-lanes | 3402ns | 3331ns | 1564ns | 3182ns | 5899ns | -7.71% |
| warm-clamp-minimum | 3575ns | 3905ns | 1340ns | 3609ns | 5709ns | -3.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3484ns | 1416ns | 5494ns | base | 2.352 |
| warm-clamp-accfit | 3457ns | 1614ns | 6208ns | -0.75% | 2.369 |
| warm-clamp-accfit-dyn | 3377ns | 1263ns | 5956ns | -3.06% | 2.426 |
| warm-clamp-head | 4017ns | 2505ns | 7768ns | +15.31% | 2.039 |
| warm-clamp-min-lanes | 3192ns | 1482ns | 5572ns | -8.37% | 2.566 |
| warm-clamp-minimum | 3380ns | 1270ns | 5370ns | -2.98% | 2.424 |

## Performance model

- Peak throughput: **6.484 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.116 | 32.6% |
| warm-clamp-accfit | 2.900 | 44.7% |
| warm-clamp-accfit-dyn | 3.785 | 58.4% |
| warm-clamp-head | 2.681 | 41.3% |
| warm-clamp-min-lanes | 2.634 | 40.6% |
| warm-clamp-minimum | 2.213 | 34.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3686ns | 3686ns | base |
| warm-clamp-accfit | 3639ns | 3639ns | -1.26% |
| warm-clamp-accfit-dyn | 3561ns | 3561ns | -3.39% |
| warm-clamp-head | 4134ns | 4134ns | +12.16% |
| warm-clamp-min-lanes | 3402ns | 3402ns | -7.71% |
| warm-clamp-minimum | 3575ns | 3575ns | -3.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3871ns | base | --- | [2969, 4205] | --- | --- | --- | --- |
| warm-clamp-accfit | 2825ns | +126.0ns (+3.3%) | [+100, +447]ns | [1677, 4561] | YES (adj: no) | 0.1924 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 2164ns | no significant difference | [-127, +381]ns | [1674, 5038] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-head | 3055ns | no significant difference | [-86, +1095]ns | [2953, 3674] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-min-lanes | 3110ns | no significant difference | [-758, +51]ns | [2777, 3474] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-minimum | 3701ns | no significant difference | [-892, +304]ns | [3327, 3968] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3874ns | +13.4% | -15.8% | -5.2% | -59.7% | +6.7% |
| 1 | 2 | 3855ns | -32.0% | -51.0% | -20.7% | -13.3% | -6.5% |
| 1 | 3 | 4502ns | -62.6% | -30.4% | -44.2% | +0.1% | -16.9% |
| 1 | 4 | 3764ns | -55.3% | -11.0% | -33.1% | -19.2% | -64.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.500 | HIGH- (thermal bounce) |
| warm-clamp-accfit | 0.175 | ok |
| warm-clamp-accfit-dyn | -0.347 | moderate- |
| warm-clamp-head | 0.237 | moderate+ |
| warm-clamp-min-lanes | -0.031 | ok |
| warm-clamp-minimum | -0.087 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 0/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 54.6ns | 3483.6ns | 1.6% |  |
| warm-clamp-accfit | 51.2ns | 3457.3ns | 1.5% |  |
| warm-clamp-accfit-dyn | 52.7ns | 3377.0ns | 1.6% |  |
| warm-clamp-head | 31.7ns | 4017.1ns | 0.8% |  |
| warm-clamp-min-lanes | 57.2ns | 3192.2ns | 1.8% |  |
| warm-clamp-minimum | 52.1ns | 3379.9ns | 1.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1415.9-5493.7 ns)
   1415.9 |########################################
   1619.8 |#####
   1823.7 |
   2027.6 |#####
   2231.5 |
   2435.4 |
   2639.3 |#####
   2843.1 |
   3047.0 |#####
   3250.9 |
   3454.8 |#####
   3658.7 |######################
   3862.6 |############################
   4066.5 |#####
   4270.4 |######################
   4474.3 |#####
   4678.2 |#####
   4882.1 |#####
   5086.0 |#####
   5289.9 |
  (4 below, 6 above range)

warm-clamp-accfit (n=40, range 1614.1-6207.7 ns)
   1614.1 |########################################
   1843.8 |
   2073.5 |
   2303.2 |
   2532.9 |
   2762.5 |####
   2992.2 |
   3221.9 |
   3451.6 |##
   3681.3 |
   3910.9 |##
   4140.6 |####
   4370.3 |##
   4600.0 |##
   4829.6 |##
   5059.3 |##
   5289.0 |##
   5518.7 |####
   5748.4 |
   5978.0 |##############
  (2 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1263.3-5955.6 ns)
   1263.3 |##############
   1498.0 |########################################
   1732.6 |
   1967.2 |###
   2201.8 |###
   2436.4 |
   2671.0 |
   2905.6 |
   3140.2 |
   3374.9 |
   3609.5 |
   3844.1 |
   4078.7 |###
   4313.3 |
   4547.9 |##########
   4782.5 |###
   5017.1 |##########
   5251.8 |##########
   5486.4 |###
   5721.0 |#######
  (4 below, 5 above range)

warm-clamp-head (n=40, range 2505.2-7768.5 ns)
   2505.2 |########################################
   2768.4 |########
   3031.5 |########################################
   3294.7 |####
   3557.9 |#############
   3821.0 |####
   4084.2 |####
   4347.3 |
   4610.5 |####
   4873.7 |####
   5136.8 |####
   5400.0 |
   5663.2 |
   5926.3 |
   6189.5 |####
   6452.7 |
   6715.8 |####
   6979.0 |
   7242.1 |####
   7505.3 |
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1482.3-5571.9 ns)
   1482.3 |########################################
   1686.8 |
   1891.2 |####
   2095.7 |####
   2300.2 |
   2504.7 |
   2709.2 |############
   2913.6 |################
   3118.1 |####################
   3322.6 |############
   3527.1 |####
   3731.6 |########
   3936.1 |
   4140.5 |####
   4345.0 |
   4549.5 |########
   4754.0 |
   4958.5 |####
   5162.9 |####
   5367.4 |
  (1 below, 4 above range)

warm-clamp-minimum (n=40, range 1270.2-5370.4 ns)
   1270.2 |#################################
   1475.3 |
   1680.3 |
   1885.3 |######
   2090.3 |
   2295.3 |######
   2500.3 |
   2705.3 |
   2910.3 |
   3115.3 |#############
   3320.3 |#############
   3525.4 |########################################
   3730.4 |####################
   3935.4 |########################################
   4140.4 |####################
   4345.4 |######
   4550.4 |######
   4755.4 |
   4960.4 |######
   5165.4 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=43.5% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=55.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=58.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.7x (possible bimodal distribution)
- **warm-clamp-head**: CV=52.3% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=46.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=44.9% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.2x (possible bimodal distribution)
