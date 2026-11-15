# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 12% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-min-lanes (1.79 us) leads warm-clamp-accfit-dyn (2.01 us) by 12%, a clear separation rather than a photo finish. CV 115.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum is an outlier: 2.4x slower than the field

warm-clamp-minimum (4.21 us) is 2.4x the fastest (1.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes is fastest but the noisiest (CV 115.7%)

warm-clamp-min-lanes wins on median (1.79 us) yet has the highest variance (CV 115.7%), while warm-clamp-acc64 is the steadiest (CV 30.8%, 3.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.5x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.42 us but its worst 20% at 6.38 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 1791.3 ns median (-54.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.35x (fastest 1791.3 ns, slowest 4214.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3839ns | 4142ns | 1633ns | 4093ns | 5281ns | base |
| warm-clamp-accfit | 3683ns | 2295ns | 1519ns | 3476ns | 6471ns | -4.05% |
| warm-clamp-accfit-dyn | 3489ns | 2114ns | 1600ns | 3188ns | 6281ns | -9.11% |
| warm-clamp-head | 4451ns | 3143ns | 2768ns | 3548ns | 8842ns | +15.94% |
| warm-clamp-min-lanes | 3455ns | 1883ns | 1503ns | 2997ns | 6782ns | -9.99% |
| warm-clamp-minimum | 4316ns | 4440ns | 1620ns | 4503ns | 6451ns | +12.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3618ns | 1546ns | 4949ns | base | 2.264 |
| warm-clamp-accfit | 3496ns | 1446ns | 6108ns | -3.37% | 2.343 |
| warm-clamp-accfit-dyn | 3312ns | 1524ns | 5940ns | -8.48% | 2.474 |
| warm-clamp-head | 4320ns | 2693ns | 8568ns | +19.39% | 1.896 |
| warm-clamp-min-lanes | 3269ns | 1424ns | 6384ns | -9.65% | 2.506 |
| warm-clamp-minimum | 4080ns | 1535ns | 6072ns | +12.75% | 2.008 |

## Performance model

- Peak throughput: **5.752 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.094 | 36.4% |
| warm-clamp-accfit | 3.746 | 65.1% |
| warm-clamp-accfit-dyn | 4.066 | 70.7% |
| warm-clamp-head | 2.680 | 46.6% |
| warm-clamp-min-lanes | 4.573 | 79.5% |
| warm-clamp-minimum | 1.944 | 33.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3839ns | 3839ns | base |
| warm-clamp-accfit | 3683ns | 3683ns | -4.05% |
| warm-clamp-accfit-dyn | 3489ns | 3489ns | -9.11% |
| warm-clamp-head | 4451ns | 4451ns | +15.94% |
| warm-clamp-min-lanes | 3455ns | 3455ns | -9.99% |
| warm-clamp-minimum | 4316ns | 4316ns | +12.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3913ns | base | --- | [3583, 4336] | --- | --- | --- | --- |
| warm-clamp-accfit | 2187ns | no significant difference | [-344, +844]ns | [1674, 5436] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-accfit-dyn | 2015ns | no significant difference | [-222, +160]ns | [1676, 4592] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-head | 3057ns | no significant difference | [-719, +1464]ns | [3044, 3068] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-min-lanes | 1791ns | no significant difference | [-1488, +608]ns | [1683, 4455] | no | 0.5370 | 0.4296 | 0 |
| warm-clamp-minimum | 4214ns | +81.5ns (+2.1%) | [+17, +763]ns | [3750, 4970] | YES (adj: no) | 0.1924 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3913ns | +34.1% | -20.5% | -0.9% | -64.1% | -1.0% |
| 1 | 2 | 4403ns | -44.5% | -29.4% | -30.4% | -58.6% | +2.1% |
| 1 | 3 | 3651ns | -54.0% | -45.3% | -16.8% | +16.7% | +43.7% |
| 1 | 4 | 3774ns | -63.6% | -52.6% | -18.9% | -55.4% | +54.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.300 | moderate- |
| warm-clamp-accfit | 0.101 | ok |
| warm-clamp-accfit-dyn | 0.282 | moderate+ |
| warm-clamp-head | -0.064 | ok |
| warm-clamp-min-lanes | -0.325 | moderate- |
| warm-clamp-minimum | 0.270 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 57.0ns | 3618.3ns | 1.6% |  |
| warm-clamp-accfit | 54.3ns | 3496.4ns | 1.6% |  |
| warm-clamp-accfit-dyn | 49.0ns | 3311.5ns | 1.5% |  |
| warm-clamp-head | 34.5ns | 4319.9ns | 0.8% |  |
| warm-clamp-min-lanes | 48.1ns | 3269.2ns | 1.5% |  |
| warm-clamp-minimum | 64.7ns | 4079.8ns | 1.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1545.5-4949.1 ns)
   1545.5 |########################################
   1715.7 |
   1885.9 |
   2056.1 |#####
   2226.2 |
   2396.4 |
   2566.6 |
   2736.8 |#####
   2906.9 |
   3077.1 |
   3247.3 |#####
   3417.5 |######################
   3587.6 |###########
   3757.8 |######################
   3928.0 |###########
   4098.2 |###########
   4268.3 |#################
   4438.5 |######################
   4608.7 |###########
   4778.9 |#################
  (1 below, 3 above range)

warm-clamp-accfit (n=40, range 1446.2-6107.9 ns)
   1446.2 |########################################
   1679.3 |######
   1912.4 |
   2145.5 |
   2378.5 |
   2611.6 |###
   2844.7 |
   3077.8 |###
   3310.9 |
   3544.0 |###
   3777.1 |
   4010.2 |
   4243.2 |
   4476.3 |
   4709.4 |
   4942.5 |######
   5175.6 |###
   5408.7 |##########
   5641.8 |###
   5874.9 |##############################
  (6 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 1523.7-5939.8 ns)
   1523.7 |########################################
   1744.5 |##
   1965.3 |##
   2186.1 |##
   2406.9 |
   2627.7 |
   2848.5 |
   3069.3 |
   3290.1 |
   3510.9 |
   3731.7 |
   3952.5 |##
   4173.3 |#####
   4394.1 |##
   4614.9 |#####
   4835.7 |##
   5056.6 |##
   5277.4 |##
   5498.2 |#####
   5719.0 |#####
  (4 below, 5 above range)

warm-clamp-head (n=40, range 2693.5-8567.6 ns)
   2693.5 |
   2987.2 |########################################
   3280.9 |
   3574.6 |
   3868.3 |
   4162.0 |
   4455.7 |#
   4749.4 |
   5043.1 |#
   5336.8 |
   5630.5 |
   5924.2 |###
   6217.9 |
   6511.6 |
   6805.3 |#
   7099.0 |#
   7392.7 |
   7686.5 |#
   7980.2 |
   8273.9 |#
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1424.2-6383.8 ns)
   1424.2 |################################
   1672.2 |########################################
   1920.2 |
   2168.2 |
   2416.1 |
   2664.1 |####
   2912.1 |
   3160.1 |
   3408.0 |########
   3656.0 |
   3904.0 |
   4152.0 |
   4399.9 |
   4647.9 |
   4895.9 |####
   5143.9 |####
   5391.9 |########
   5639.8 |########
   5887.8 |########
   6135.8 |############
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 1534.9-6072.4 ns)
   1534.9 |########################################
   1761.8 |######
   1988.7 |
   2215.5 |
   2442.4 |
   2669.3 |
   2896.2 |
   3123.0 |
   3349.9 |
   3576.8 |#################################
   3803.7 |##########################
   4030.5 |#############
   4257.4 |######
   4484.3 |######
   4711.2 |####################
   4938.0 |######
   5164.9 |#############
   5391.8 |######
   5618.7 |####################
   5845.5 |##########################
  (3 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=33.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=58.4% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=56.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=53.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=63.4% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=40.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
