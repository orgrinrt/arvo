# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-head at 2.33 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.53)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (822 ns) is smaller than the fastest variant's own run-to-run std-dev (2.28 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.0x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.96 us but its worst 20% at 7.86 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 2335.0 ns median (-26.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.35x (fastest 2335.0 ns, slowest 3156.6 ns)

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
| warm-clamp-acc64 | 3246ns | 3391ns | 2102ns | 3377ns | 3999ns | base |
| warm-clamp-accfit | 2628ns | 2701ns | 2201ns | 2606ns | 3122ns | -19.04% |
| warm-clamp-accfit-dyn | 3136ns | 3112ns | 2642ns | 3140ns | 3621ns | -3.38% |
| warm-clamp-head | 3916ns | 2398ns | 1960ns | 3284ns | 7769ns | +20.64% |
| warm-clamp-min-lanes | 3932ns | 2455ns | 2010ns | 3173ns | 8130ns | +21.13% |
| warm-clamp-minimum | 3811ns | 2460ns | 2011ns | 3064ns | 7850ns | +17.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3037ns | 1974ns | 3720ns | base | 2.697 |
| warm-clamp-accfit | 2394ns | 2014ns | 2826ns | -21.19% | 3.422 |
| warm-clamp-accfit-dyn | 2918ns | 2468ns | 3343ns | -3.93% | 2.807 |
| warm-clamp-head | 3803ns | 1909ns | 7512ns | +25.20% | 2.154 |
| warm-clamp-min-lanes | 3819ns | 1959ns | 7864ns | +25.75% | 2.145 |
| warm-clamp-minimum | 3703ns | 1959ns | 7598ns | +21.90% | 2.212 |

## Performance model

- Peak throughput: **4.292 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.595 | 60.5% |
| warm-clamp-accfit | 3.363 | 78.4% |
| warm-clamp-accfit-dyn | 2.822 | 65.8% |
| warm-clamp-head | 3.508 | 81.7% |
| warm-clamp-min-lanes | 3.425 | 79.8% |
| warm-clamp-minimum | 3.418 | 79.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3246ns | 3246ns | base |
| warm-clamp-accfit | 2628ns | 2628ns | -19.04% |
| warm-clamp-accfit-dyn | 3136ns | 3136ns | -3.38% |
| warm-clamp-head | 3916ns | 3916ns | +20.64% |
| warm-clamp-min-lanes | 3932ns | 3932ns | +21.13% |
| warm-clamp-minimum | 3811ns | 3811ns | +17.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3157ns | base | --- | [2911, 3472] | --- | --- | --- | --- |
| warm-clamp-accfit | 2436ns | -497.6ns (-15.8%) | [-1034, -411]ns | [2281, 2504] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2903ns | -159.2ns (-5.0%) | [-241, -134]ns | [2794, 3032] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2335ns | no significant difference | [-828, +1377]ns | [2333, 4363] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-min-lanes | 2392ns | no significant difference | [-674, +1072]ns | [2391, 3554] | no | 0.7948 | 0.6358 | 0 |
| warm-clamp-minimum | 2397ns | no significant difference | [-717, +940]ns | [2394, 2805] | no | 0.7948 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2856ns | -12.1% | -3.8% | -33.0% | -16.2% | -12.8% |
| 1 | 2 | 3165ns | -18.4% | -7.3% | -8.0% | -22.6% | -24.1% |
| 1 | 3 | 3495ns | -40.2% | -13.8% | -33.1% | -31.4% | -43.7% |
| 1 | 4 | 3341ns | -34.9% | -8.7% | -29.9% | -41.1% | -28.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.174 | ok |
| warm-clamp-accfit | 0.125 | ok |
| warm-clamp-accfit-dyn | 0.164 | ok |
| warm-clamp-head | -0.530 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.031 | ok |
| warm-clamp-minimum | -0.267 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 76.4ns | 3037.4ns | 2.5% |  |
| warm-clamp-accfit | 84.4ns | 2393.6ns | 3.5% |  |
| warm-clamp-accfit-dyn | 75.8ns | 2918.1ns | 2.6% |  |
| warm-clamp-head | 37.6ns | 3802.7ns | 1.0% |  |
| warm-clamp-min-lanes | 37.3ns | 3819.5ns | 1.0% |  |
| warm-clamp-minimum | 35.7ns | 3702.7ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1974.4-3720.2 ns)
   1974.4 |
   2061.7 |
   2149.0 |
   2236.3 |
   2323.6 |
   2410.9 |
   2498.2 |###
   2585.5 |######
   2672.7 |###
   2760.0 |######
   2847.3 |################
   2934.6 |###
   3021.9 |
   3109.2 |####################
   3196.5 |##########
   3283.8 |
   3371.1 |
   3458.3 |########################################
   3545.6 |
   3632.9 |
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 2013.7-2826.0 ns)
   2013.7 |######
   2054.3 |####################
   2094.9 |#############
   2135.5 |######
   2176.1 |#############
   2216.8 |
   2257.4 |####################
   2298.0 |#############
   2338.6 |######
   2379.2 |######
   2419.8 |######
   2460.4 |##########################
   2501.1 |########################################
   2541.7 |######
   2582.3 |
   2622.9 |######
   2663.5 |
   2704.1 |######
   2744.7 |######
   2785.3 |####################
  (4 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 2468.4-3343.2 ns)
   2468.4 |########
   2512.1 |################
   2555.9 |################
   2599.6 |########
   2643.4 |
   2687.1 |
   2730.8 |########################################
   2774.6 |########################################
   2818.3 |################
   2862.1 |########
   2905.8 |########
   2949.5 |
   2993.3 |########################################
   3037.0 |########################
   3080.8 |########
   3124.5 |########
   3168.2 |
   3212.0 |
   3255.7 |########################
   3299.5 |########
  (1 below, 5 above range)

warm-clamp-head (n=40, range 1908.8-7511.8 ns)
   1908.8 |######################
   2189.0 |########################################
   2469.1 |
   2749.3 |
   3029.4 |
   3309.6 |
   3589.7 |
   3869.9 |##
   4150.0 |
   4430.2 |
   4710.3 |##
   4990.5 |
   5270.6 |
   5550.8 |#####
   5830.9 |#####
   6111.1 |#####
   6391.2 |##
   6671.3 |
   6951.5 |#####
   7231.6 |
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1958.5-7864.3 ns)
   1958.5 |###########
   2253.8 |########################################
   2549.1 |
   2844.4 |##
   3139.7 |
   3435.0 |##
   3730.2 |
   4025.5 |##
   4320.8 |
   4616.1 |
   4911.4 |
   5206.7 |##
   5502.0 |
   5797.3 |##
   6092.6 |##
   6387.9 |#####
   6683.1 |##
   6978.4 |#####
   7273.7 |
   7569.0 |
  (7 below, 4 above range)

warm-clamp-minimum (n=40, range 1958.8-7598.4 ns)
   1958.8 |##################
   2240.8 |########################################
   2522.8 |######
   2804.8 |###
   3086.7 |
   3368.7 |
   3650.7 |
   3932.7 |
   4214.7 |###
   4496.6 |###
   4778.6 |
   5060.6 |
   5342.6 |###
   5624.5 |###
   5906.5 |###
   6188.5 |###
   6470.5 |###
   6752.5 |###
   7034.4 |###
   7316.4 |###
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=22.9% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=60.1% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=62.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=60.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
