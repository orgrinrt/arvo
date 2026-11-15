# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (2.92 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-accfit at 1.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit dominates: 41% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-accfit (1.15 us) leads warm-clamp-accfit-dyn (1.62 us) by 41%, a clear separation rather than a photo finish. CV 12.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 62% (significant)

warm-clamp-accfit is -1.80 us (62%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.5x slower than the field

warm-clamp-acc64 (2.92 us) is 2.5x the fastest (1.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.67)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-min-lanes is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.96 us but its worst 20% at 8.44 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 1149.0 ns median (-60.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.54x (fastest 1149.0 ns, slowest 2919.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3080ns | 3175ns | 2113ns | 3123ns | 3921ns | base |
| warm-clamp-accfit | 1388ns | 1389ns | 1150ns | 1361ns | 1708ns | -54.93% |
| warm-clamp-accfit-dyn | 1837ns | 1863ns | 1626ns | 1843ns | 2031ns | -40.36% |
| warm-clamp-head | 4404ns | 2405ns | 2393ns | 3712ns | 8492ns | +42.96% |
| warm-clamp-min-lanes | 3943ns | 2462ns | 2014ns | 2993ns | 8720ns | +27.99% |
| warm-clamp-minimum | 3986ns | 2460ns | 2013ns | 3347ns | 7875ns | +29.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2837ns | 1962ns | 3543ns | base | 2.888 |
| warm-clamp-accfit | 1137ns | 956ns | 1371ns | -59.93% | 7.206 |
| warm-clamp-accfit-dyn | 1594ns | 1417ns | 1732ns | -43.82% | 5.139 |
| warm-clamp-head | 4277ns | 2330ns | 8215ns | +50.77% | 1.915 |
| warm-clamp-min-lanes | 3830ns | 1963ns | 8436ns | +34.99% | 2.139 |
| warm-clamp-minimum | 3872ns | 1961ns | 7629ns | +36.47% | 2.116 |

## Performance model

- Peak throughput: **8.572 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.806 | 32.7% |
| warm-clamp-accfit | 7.130 | 83.2% |
| warm-clamp-accfit-dyn | 5.052 | 58.9% |
| warm-clamp-head | 3.498 | 40.8% |
| warm-clamp-min-lanes | 3.415 | 39.8% |
| warm-clamp-minimum | 3.417 | 39.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3080ns | 3080ns | base |
| warm-clamp-accfit | 1388ns | 1388ns | -54.93% |
| warm-clamp-accfit-dyn | 1837ns | 1837ns | -40.36% |
| warm-clamp-head | 4404ns | 4404ns | +42.96% |
| warm-clamp-min-lanes | 3943ns | 3943ns | +27.99% |
| warm-clamp-minimum | 3986ns | 3986ns | +29.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2920ns | base | --- | [2660, 3128] | --- | --- | --- | --- |
| warm-clamp-accfit | 1149ns | -1804.4ns (-61.8%) | [-1946, -1625]ns | [1049, 1153] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1622ns | -1323.9ns (-45.3%) | [-1524, -1013]ns | [1608, 1637] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2342ns | no significant difference | [-330, +2749]ns | [2330, 5792] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-min-lanes | 2399ns | no significant difference | [-467, +639]ns | [2189, 2539] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-minimum | 2397ns | no significant difference | [-524, +1528]ns | [2396, 3872] | no | 0.7948 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3128ns | -63.1% | -53.7% | -25.3% | -23.3% | -22.6% |
| 1 | 2 | 2872ns | -63.6% | -42.5% | -18.9% | +44.7% | -31.4% |
| 1 | 3 | 3232ns | -57.3% | -50.1% | -27.5% | -39.0% | -25.8% |
| 1 | 4 | 2536ns | -61.1% | -35.8% | +62.0% | -22.4% | +12.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.524 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.675 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.236 | moderate- |
| warm-clamp-head | -0.079 | ok |
| warm-clamp-min-lanes | -0.281 | moderate- |
| warm-clamp-minimum | -0.015 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 78.3ns | 2837.0ns | 2.8% |  |
| warm-clamp-accfit | 85.4ns | 1136.8ns | 7.5% | HIGH |
| warm-clamp-accfit-dyn | 84.3ns | 1594.0ns | 5.3% | HIGH |
| warm-clamp-head | 43.2ns | 4277.4ns | 1.0% |  |
| warm-clamp-min-lanes | 38.4ns | 3829.6ns | 1.0% |  |
| warm-clamp-minimum | 38.3ns | 3871.7ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1962.2-3543.4 ns)
   1962.2 |
   2041.2 |#####
   2120.3 |
   2199.3 |#####
   2278.4 |
   2357.5 |###############
   2436.5 |##########
   2515.6 |
   2594.7 |###################################
   2673.7 |
   2752.8 |
   2831.9 |####################
   2910.9 |###############
   2990.0 |
   3069.0 |########################################
   3148.1 |##########
   3227.2 |#####
   3306.2 |
   3385.3 |#####
   3464.4 |###############
  (2 below, 2 above range)

warm-clamp-accfit (n=40, range 955.7-1371.0 ns)
    955.7 |
    976.4 |##################
    997.2 |###
   1018.0 |
   1038.7 |#####################
   1059.5 |###
   1080.3 |###
   1101.0 |
   1121.8 |
   1142.6 |########################################
   1163.3 |
   1184.1 |
   1204.9 |
   1225.6 |
   1246.4 |##############
   1267.2 |###
   1287.9 |
   1308.7 |
   1329.5 |
   1350.2 |###
  (4 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 1416.8-1732.4 ns)
   1416.8 |
   1432.6 |######
   1448.4 |####################
   1464.2 |#############
   1479.9 |######
   1495.7 |######
   1511.5 |
   1527.3 |
   1543.1 |######
   1558.8 |
   1574.6 |######
   1590.4 |######
   1606.2 |########################################
   1621.9 |########################################
   1637.7 |#################################
   1653.5 |##########################
   1669.3 |######
   1685.0 |
   1700.8 |
   1716.6 |
  (3 below, 4 above range)

warm-clamp-head (n=40, range 2330.0-8214.6 ns)
   2330.0 |########################################
   2624.2 |
   2918.5 |
   3212.7 |
   3506.9 |
   3801.1 |
   4095.4 |
   4389.6 |
   4683.8 |
   4978.1 |
   5272.3 |
   5566.5 |#
   5860.8 |#
   6155.0 |
   6449.2 |#
   6743.5 |
   7037.7 |#####
   7331.9 |
   7626.2 |#######
   7920.4 |
  (4 below, 5 above range)

warm-clamp-min-lanes (n=40, range 1962.7-8436.5 ns)
   1962.7 |#################################
   2286.4 |########################################
   2610.0 |
   2933.7 |
   3257.4 |
   3581.1 |######
   3904.8 |
   4228.5 |
   4552.2 |
   4875.9 |
   5199.6 |
   5523.3 |
   5846.9 |###
   6170.6 |
   6494.3 |###
   6818.0 |###
   7141.7 |###
   7465.4 |######
   7789.1 |
   8112.8 |######
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 1960.7-7629.4 ns)
   1960.7 |####################
   2244.2 |########################################
   2527.6 |
   2811.0 |
   3094.5 |
   3377.9 |
   3661.3 |##
   3944.8 |##
   4228.2 |
   4511.7 |
   4795.1 |
   5078.5 |
   5362.0 |
   5645.4 |#####
   5928.8 |
   6212.3 |##
   6495.7 |########
   6779.1 |##
   7062.6 |########
   7346.0 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=21.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=60.1% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=68.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=60.1% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
