# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 19% faster than the next best (warm-clamp-head)

warm-clamp-min-lanes (2.74 us) leads warm-clamp-head (3.24 us) by 19%, a clear separation rather than a photo finish. CV 76.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is fastest but the noisiest (CV 76.9%)

warm-clamp-min-lanes wins on median (2.74 us) yet has the highest variance (CV 76.9%), while warm-clamp-acc64 is the steadiest (CV 35.2%, 3.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.59)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.31 us) is smaller than the fastest variant's own run-to-run std-dev (2.10 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-head is inconsistent: worst-20% is 4.4x its best-20%

warm-clamp-head's best 20% of batches run at 1.04 us but its worst 20% at 4.61 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2735.8 ns median (-21.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.48x (fastest 2735.8 ns, slowest 4050.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3476ns | 3635ns | 1436ns | 3610ns | 5115ns | base |
| warm-clamp-accfit | 3859ns | 4219ns | 1695ns | 3936ns | 5790ns | +11.00% |
| warm-clamp-accfit-dyn | 3704ns | 3398ns | 1563ns | 3585ns | 6201ns | +6.54% |
| warm-clamp-head | 3285ns | 3415ns | 1095ns | 3479ns | 4890ns | -5.51% |
| warm-clamp-min-lanes | 4122ns | 2810ns | 1959ns | 3842ns | 7126ns | +18.59% |
| warm-clamp-minimum | 4425ns | 4139ns | 2286ns | 4126ns | 7462ns | +27.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3305ns | 1371ns | 4838ns | base | 2.478 |
| warm-clamp-accfit | 3693ns | 1628ns | 5516ns | +11.74% | 2.218 |
| warm-clamp-accfit-dyn | 3546ns | 1501ns | 5913ns | +7.27% | 2.310 |
| warm-clamp-head | 3106ns | 1039ns | 4611ns | -6.03% | 2.637 |
| warm-clamp-min-lanes | 4001ns | 1908ns | 6876ns | +21.05% | 2.047 |
| warm-clamp-minimum | 4298ns | 2226ns | 7227ns | +30.03% | 1.906 |

## Performance model

- Peak throughput: **7.881 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.361 | 30.0% |
| warm-clamp-accfit | 2.022 | 25.7% |
| warm-clamp-accfit-dyn | 2.510 | 31.9% |
| warm-clamp-head | 2.527 | 32.1% |
| warm-clamp-min-lanes | 2.994 | 38.0% |
| warm-clamp-minimum | 2.032 | 25.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3476ns | 3476ns | base |
| warm-clamp-accfit | 3859ns | 3859ns | +11.00% |
| warm-clamp-accfit-dyn | 3704ns | 3704ns | +6.54% |
| warm-clamp-head | 3285ns | 3285ns | -5.51% |
| warm-clamp-min-lanes | 4122ns | 4122ns | +18.59% |
| warm-clamp-minimum | 4425ns | 4425ns | +27.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3469ns | base | --- | [3180, 3740] | --- | --- | --- | --- |
| warm-clamp-accfit | 4051ns | +319.8ns (+9.2%) | [+267, +820]ns | [3543, 4468] | YES | 0.0009 | 0.0002 | 0 |
| warm-clamp-accfit-dyn | 3264ns | no significant difference | [-1225, +1519]ns | [1662, 5108] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-head | 3242ns | no significant difference | [-570, +319]ns | [3010, 3521] | no | 0.7948 | 0.6358 | 0 |
| warm-clamp-min-lanes | 2736ns | no significant difference | [-783, +1786]ns | [2332, 5634] | no | 0.4470 | 0.2682 | 0 |
| warm-clamp-minimum | 4031ns | +1100.4ns (+31.7%) | [+203, +1490]ns | [2338, 5644] | YES (adj: no) | 0.2017 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1374ns | +23.2% | +173.9% | +138.9% | +298.7% | +192.3% |
| 1 | 2 | 3809ns | +16.4% | -33.1% | +8.7% | -38.6% | -23.2% |
| 1 | 3 | 3180ns | +40.2% | +26.4% | -38.1% | +32.1% | -26.5% |
| 1 | 4 | 3652ns | +9.0% | -2.6% | -12.9% | -19.2% | +40.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.281 | moderate- |
| warm-clamp-accfit | -0.119 | ok |
| warm-clamp-accfit-dyn | -0.585 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.450 | moderate- |
| warm-clamp-min-lanes | -0.591 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.297 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/4, lost 4/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 58.3ns | 3305.5ns | 1.8% |  |
| warm-clamp-accfit | 54.3ns | 3693.5ns | 1.5% |  |
| warm-clamp-accfit-dyn | 52.2ns | 3545.9ns | 1.5% |  |
| warm-clamp-head | 64.0ns | 3106.2ns | 2.1% |  |
| warm-clamp-min-lanes | 40.9ns | 4001.3ns | 1.0% |  |
| warm-clamp-minimum | 43.4ns | 4298.2ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1370.6-4837.9 ns)
   1370.6 |########################################
   1544.0 |
   1717.3 |
   1890.7 |
   2064.1 |
   2237.4 |
   2410.8 |
   2584.2 |
   2757.5 |#################
   2930.9 |###########
   3104.3 |###########
   3277.6 |######################
   3451.0 |##################################
   3624.4 |#################
   3797.7 |#################
   3971.1 |###########
   4144.5 |#################
   4317.8 |#####
   4491.2 |#####
   4664.6 |
  (1 below, 2 above range)

warm-clamp-accfit (n=40, range 1627.6-5515.9 ns)
   1627.6 |########################################
   1822.0 |
   2016.4 |
   2210.9 |
   2405.3 |
   2599.7 |####
   2794.1 |####
   2988.5 |
   3182.9 |
   3377.3 |####
   3571.8 |############
   3766.2 |####
   3960.6 |############
   4155.0 |####################
   4349.4 |
   4543.8 |####################
   4738.3 |
   4932.7 |############
   5127.1 |####
   5321.5 |########
  (1 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1500.6-5913.4 ns)
   1500.6 |########################################
   1721.2 |
   1941.9 |###
   2162.5 |
   2383.1 |
   2603.8 |
   2824.4 |###
   3045.1 |###
   3265.7 |###
   3486.3 |
   3707.0 |
   3927.6 |
   4148.3 |
   4368.9 |###
   4589.5 |###
   4810.2 |#########
   5030.8 |#########
   5251.5 |###
   5472.1 |############
   5692.7 |
  (4 below, 6 above range)

warm-clamp-head (n=40, range 1039.5-4610.8 ns)
   1039.5 |####################
   1218.0 |
   1396.6 |
   1575.2 |
   1753.7 |
   1932.3 |
   2110.9 |
   2289.4 |
   2468.0 |
   2646.6 |#############
   2825.1 |####################
   3003.7 |########################################
   3182.3 |#################################
   3360.8 |####################
   3539.4 |#############
   3717.9 |######
   3896.5 |######
   4075.1 |####################
   4253.6 |######
   4432.2 |######
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1908.1-6875.7 ns)
   1908.1 |########################################
   2156.5 |#########################
   2404.8 |
   2653.2 |
   2901.6 |###
   3150.0 |
   3398.4 |
   3646.7 |
   3895.1 |###
   4143.5 |
   4391.9 |
   4640.3 |
   4888.6 |###
   5137.0 |###
   5385.4 |#######
   5633.8 |#######
   5882.2 |##################
   6130.5 |###
   6378.9 |#######
   6627.3 |###
  (2 below, 3 above range)

warm-clamp-minimum (n=40, range 2226.3-7226.5 ns)
   2226.3 |########################################
   2476.3 |
   2726.3 |##
   2976.4 |
   3226.4 |
   3476.4 |##
   3726.4 |
   3976.4 |
   4226.4 |
   4476.4 |##
   4726.4 |#####
   4976.4 |
   5226.5 |#####
   5476.5 |#####
   5726.5 |##
   5976.5 |#######
   6226.5 |##
   6476.5 |#####
   6726.5 |##
   6976.5 |##
  (2 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=36.9% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=39.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=53.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=38.6% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=52.6% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=47.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.2x (possible bimodal distribution)
