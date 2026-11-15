# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes is an outlier: 2.1x slower than the field

warm-clamp-min-lanes (4.56 us) is 2.1x the fastest (2.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.64)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-accfit is inconsistent: worst-20% is 4.2x its best-20%

warm-clamp-accfit's best 20% of batches run at 1.89 us but its worst 20% at 7.85 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 2188.7 ns median (-16.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.08x (fastest 2188.7 ns, slowest 4563.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3787ns | 2690ns | 2159ns | 2795ns | 8389ns | base |
| warm-clamp-accfit | 4250ns | 2347ns | 1965ns | 3699ns | 8189ns | +12.23% |
| warm-clamp-accfit-dyn | 3826ns | 2275ns | 1976ns | 3151ns | 7698ns | +1.02% |
| warm-clamp-head | 4340ns | 3168ns | 2591ns | 3288ns | 9245ns | +14.61% |
| warm-clamp-min-lanes | 3978ns | 4808ns | 1657ns | 4001ns | 6233ns | +5.06% |
| warm-clamp-minimum | 4667ns | 4058ns | 2165ns | 4367ns | 8069ns | +23.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3654ns | 2088ns | 8066ns | base | 2.242 |
| warm-clamp-accfit | 4082ns | 1891ns | 7848ns | +11.71% | 2.007 |
| warm-clamp-accfit-dyn | 3669ns | 1900ns | 7364ns | +0.40% | 2.233 |
| warm-clamp-head | 4211ns | 2521ns | 8938ns | +15.23% | 1.945 |
| warm-clamp-min-lanes | 3776ns | 1577ns | 5889ns | +3.32% | 2.170 |
| warm-clamp-minimum | 4489ns | 2088ns | 7716ns | +22.84% | 1.825 |

## Performance model

- Peak throughput: **5.195 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.144 | 60.5% |
| warm-clamp-accfit | 3.624 | 69.8% |
| warm-clamp-accfit-dyn | 3.743 | 72.0% |
| warm-clamp-head | 2.658 | 51.2% |
| warm-clamp-min-lanes | 1.795 | 34.6% |
| warm-clamp-minimum | 2.099 | 40.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3787ns | 3787ns | base |
| warm-clamp-accfit | 4250ns | 4250ns | +12.23% |
| warm-clamp-accfit-dyn | 3826ns | 3826ns | +1.02% |
| warm-clamp-head | 4340ns | 4340ns | +14.61% |
| warm-clamp-min-lanes | 3978ns | 3978ns | +5.06% |
| warm-clamp-minimum | 4667ns | 4667ns | +23.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2605ns | base | --- | [2592, 2625] | --- | --- | --- | --- |
| warm-clamp-accfit | 2261ns | no significant difference | [-417, +182]ns | [2240, 5296] | no | 0.4470 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 2189ns | no significant difference | [-473, +163]ns | [2181, 3614] | no | 0.5370 | 0.4296 | 0 |
| warm-clamp-head | 3082ns | +491.2ns (+18.9%) | [+468, +508]ns | [2534, 3094] | YES | 0.0321 | 0.0064 | 0 |
| warm-clamp-min-lanes | 4563ns | no significant difference | [-874, +954]ns | [2196, 4956] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-minimum | 3902ns | no significant difference | [-158, +889]ns | [2442, 5906] | no | 0.4470 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2028ns | +138.8% | +8.3% | +24.6% | +97.7% | +76.9% |
| 1 | 2 | 2629ns | -14.8% | -18.5% | -3.9% | +41.7% | +49.8% |
| 1 | 3 | 2577ns | -30.2% | +1.4% | +19.9% | +69.4% | +1.8% |
| 1 | 4 | 2602ns | -13.0% | -15.6% | +18.8% | +84.1% | +68.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.146 | ok |
| warm-clamp-accfit | -0.012 | ok |
| warm-clamp-accfit-dyn | -0.445 | moderate- |
| warm-clamp-head | 0.250 | moderate+ |
| warm-clamp-min-lanes | 0.183 | ok |
| warm-clamp-minimum | -0.643 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 33.9ns | 3654.4ns | 0.9% |  |
| warm-clamp-accfit | 43.5ns | 4082.4ns | 1.1% |  |
| warm-clamp-accfit-dyn | 39.8ns | 3669.0ns | 1.1% |  |
| warm-clamp-head | 33.5ns | 4211.1ns | 0.8% |  |
| warm-clamp-min-lanes | 53.4ns | 3775.9ns | 1.4% |  |
| warm-clamp-minimum | 46.3ns | 4489.3ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2087.8-8066.2 ns)
   2087.8 |
   2386.7 |########################################
   2685.6 |
   2984.5 |#
   3283.4 |
   3582.4 |
   3881.3 |
   4180.2 |
   4479.1 |#
   4778.1 |
   5077.0 |
   5375.9 |
   5674.8 |
   5973.8 |#
   6272.7 |#
   6571.6 |
   6870.5 |
   7169.5 |#
   7468.4 |
   7767.3 |
  (7 below, 5 above range)

warm-clamp-accfit (n=40, range 1890.7-7848.3 ns)
   1890.7 |##
   2188.5 |########################################
   2486.4 |
   2784.3 |##
   3082.2 |
   3380.1 |
   3677.9 |
   3975.8 |
   4273.7 |
   4571.6 |#####
   4869.5 |
   5167.3 |
   5465.2 |
   5763.1 |##
   6061.0 |##
   6358.9 |#####
   6656.8 |##
   6954.6 |
   7252.5 |#######
   7550.4 |##
  (6 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 1900.0-7364.4 ns)
   1900.0 |#######################
   2173.2 |########################################
   2446.4 |
   2719.7 |
   2992.9 |###
   3266.1 |###
   3539.3 |###
   3812.5 |
   4085.8 |
   4359.0 |
   4632.2 |###
   4905.4 |
   5178.6 |###
   5451.8 |###
   5725.1 |######
   5998.3 |
   6271.5 |
   6544.7 |###
   6817.9 |###
   7091.2 |######
  (5 below, 4 above range)

warm-clamp-head (n=40, range 2520.9-8938.4 ns)
   2520.9 |###############################
   2841.8 |########################################
   3162.6 |
   3483.5 |
   3804.4 |
   4125.3 |
   4446.1 |#####
   4767.0 |
   5087.9 |
   5408.8 |
   5729.6 |
   6050.5 |
   6371.4 |##
   6692.3 |
   7013.1 |
   7334.0 |##
   7654.9 |##
   7975.8 |
   8296.6 |##
   8617.5 |##
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1576.8-5888.6 ns)
   1576.8 |########################################
   1792.4 |
   2008.0 |
   2223.6 |
   2439.2 |###
   2654.8 |###
   2870.4 |
   3085.9 |###
   3301.5 |
   3517.1 |
   3732.7 |###
   3948.3 |
   4163.9 |###
   4379.5 |###
   4595.1 |##############
   4810.6 |##########
   5026.2 |#######
   5241.8 |##############
   5457.4 |
   5673.0 |##########
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 2088.3-7716.4 ns)
   2088.3 |#################################
   2369.7 |########################################
   2651.1 |
   2932.5 |
   3213.9 |##########################
   3495.3 |
   3776.7 |
   4058.2 |######
   4339.6 |#############
   4621.0 |######
   4902.4 |
   5183.8 |######
   5465.2 |
   5746.6 |#############
   6028.0 |######
   6309.4 |#############
   6590.8 |######
   6872.2 |####################
   7153.6 |#############
   7435.0 |
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=63.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=60.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=59.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=59.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=46.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=49.6% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
