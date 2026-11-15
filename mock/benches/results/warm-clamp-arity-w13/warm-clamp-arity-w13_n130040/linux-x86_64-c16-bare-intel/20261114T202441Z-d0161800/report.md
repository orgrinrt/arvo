# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 42% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-head (1.59 us) leads warm-clamp-accfit-dyn (2.25 us) by 42%, a clear separation rather than a photo finish. CV 8.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 55% (significant)

warm-clamp-head is -1.90 us (55%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 2.5x slower than the field

warm-clamp-min-lanes (3.91 us) is 2.5x the fastest (1.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.65)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-minimum is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-minimum's best 20% of batches run at 1.48 us but its worst 20% at 5.78 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 1587.9 ns median (-54.4% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.46x (fastest 1587.9 ns, slowest 3912.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3576ns | 3682ns | 1758ns | 3709ns | 4996ns | base |
| warm-clamp-accfit | 2535ns | 2493ns | 2217ns | 2514ns | 2913ns | -29.13% |
| warm-clamp-accfit-dyn | 2577ns | 2488ns | 2443ns | 2522ns | 2878ns | -27.94% |
| warm-clamp-head | 1779ns | 1823ns | 1559ns | 1780ns | 1995ns | -50.27% |
| warm-clamp-min-lanes | 3704ns | 4065ns | 1680ns | 3559ns | 6163ns | +3.57% |
| warm-clamp-minimum | 3686ns | 4064ns | 1540ns | 3611ns | 6059ns | +3.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3357ns | 1653ns | 4661ns | base | 2.440 |
| warm-clamp-accfit | 2265ns | 1979ns | 2610ns | -32.52% | 3.616 |
| warm-clamp-accfit-dyn | 2326ns | 2216ns | 2556ns | -30.72% | 3.523 |
| warm-clamp-head | 1538ns | 1357ns | 1705ns | -54.19% | 5.327 |
| warm-clamp-min-lanes | 3553ns | 1617ns | 5887ns | +5.83% | 2.306 |
| warm-clamp-minimum | 3535ns | 1480ns | 5785ns | +5.30% | 2.317 |

## Performance model

- Peak throughput: **6.037 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.351 | 38.9% |
| warm-clamp-accfit | 3.631 | 60.1% |
| warm-clamp-accfit-dyn | 3.638 | 60.3% |
| warm-clamp-head | 5.159 | 85.5% |
| warm-clamp-min-lanes | 2.094 | 34.7% |
| warm-clamp-minimum | 2.095 | 34.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3576ns | 3576ns | base |
| warm-clamp-accfit | 2535ns | 2535ns | -29.13% |
| warm-clamp-accfit-dyn | 2577ns | 2577ns | -27.94% |
| warm-clamp-head | 1779ns | 1779ns | -50.27% |
| warm-clamp-min-lanes | 3704ns | 3704ns | +3.57% |
| warm-clamp-minimum | 3686ns | 3686ns | +3.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3485ns | base | --- | [3212, 3744] | --- | --- | --- | --- |
| warm-clamp-accfit | 2256ns | -1195.7ns (-34.3%) | [-1282, -1115]ns | [2208, 2300] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2252ns | -1159.2ns (-33.3%) | [-1338, -961]ns | [2251, 2297] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 1588ns | -1899.2ns (-54.5%) | [-2122, -1720]ns | [1487, 1589] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3912ns | no significant difference | [-365, +959]ns | [1617, 4962] | no | 0.1539 | 0.1539 | 0 |
| warm-clamp-minimum | 3910ns | +630.6ns (+18.1%) | [+440, +755]ns | [2168, 4400] | YES | 0.0080 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3234ns | -35.4% | -30.4% | -54.6% | -11.7% | +19.2% |
| 1 | 2 | 4214ns | -50.2% | -46.1% | -66.7% | -61.2% | -62.3% |
| 1 | 3 | 3618ns | -33.4% | -37.8% | -56.1% | +26.3% | +32.4% |
| 1 | 4 | 3518ns | -34.6% | -36.0% | -52.9% | -36.5% | +19.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.483 | moderate- |
| warm-clamp-accfit | 0.107 | ok |
| warm-clamp-accfit-dyn | -0.405 | moderate- |
| warm-clamp-head | 0.199 | ok |
| warm-clamp-min-lanes | -0.654 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.372 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 74.2ns | 3356.9ns | 2.2% |  |
| warm-clamp-accfit | 92.9ns | 2265.2ns | 4.1% |  |
| warm-clamp-accfit-dyn | 88.7ns | 2325.5ns | 3.8% |  |
| warm-clamp-head | 84.0ns | 1538.0ns | 5.5% | HIGH |
| warm-clamp-min-lanes | 52.0ns | 3552.5ns | 1.5% |  |
| warm-clamp-minimum | 52.9ns | 3534.8ns | 1.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1653.5-4661.1 ns)
   1653.5 |######
   1803.9 |
   1954.2 |
   2104.6 |
   2255.0 |######
   2405.4 |
   2555.8 |
   2706.2 |
   2856.5 |####################
   3006.9 |
   3157.3 |########################################
   3307.7 |####################
   3458.1 |#################################
   3608.4 |####################
   3758.8 |#############
   3909.2 |#############
   4059.6 |#############
   4210.0 |####################
   4360.3 |
   4510.7 |######
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 1979.4-2609.8 ns)
   1979.4 |
   2010.9 |####
   2042.4 |################
   2073.9 |
   2105.5 |####
   2137.0 |############
   2168.5 |
   2200.0 |
   2231.5 |########################################
   2263.1 |####
   2294.6 |################
   2326.1 |########
   2357.6 |
   2389.1 |
   2420.7 |
   2452.2 |####
   2483.7 |####################
   2515.2 |
   2546.7 |
   2578.3 |
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2215.7-2555.7 ns)
   2215.7 |
   2232.7 |########
   2249.7 |########################################
   2266.7 |
   2283.7 |####
   2300.7 |##
   2317.7 |##
   2334.7 |
   2351.7 |##
   2368.7 |##
   2385.7 |
   2402.7 |
   2419.7 |##
   2436.7 |##
   2453.7 |
   2470.7 |
   2487.7 |########
   2504.7 |##
   2521.7 |
   2538.7 |
  (2 below, 3 above range)

warm-clamp-head (n=40, range 1356.9-1705.0 ns)
   1356.9 |#####
   1374.3 |
   1391.7 |##
   1409.1 |
   1426.5 |
   1443.9 |################
   1461.3 |
   1478.7 |#####
   1496.1 |
   1513.5 |########
   1530.9 |
   1548.3 |
   1565.7 |
   1583.1 |########################################
   1600.5 |
   1617.9 |##
   1635.3 |##
   1652.7 |
   1670.2 |
   1687.6 |#####
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1616.9-5887.4 ns)
   1616.9 |########################################
   1830.4 |
   2043.9 |
   2257.5 |
   2471.0 |
   2684.5 |##
   2898.0 |
   3111.6 |
   3325.1 |
   3538.6 |##
   3752.2 |
   3965.7 |##
   4179.2 |##
   4392.7 |#####
   4606.3 |##
   4819.8 |##
   5033.3 |##
   5246.9 |########
   5460.4 |##
   5673.9 |################
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 1480.4-5784.7 ns)
   1480.4 |########################################
   1695.6 |
   1910.9 |####
   2126.1 |####
   2341.3 |
   2556.5 |
   2771.7 |
   2986.9 |
   3202.1 |
   3417.3 |########
   3632.6 |########
   3847.8 |########
   4063.0 |############
   4278.2 |########################
   4493.4 |####
   4708.6 |############
   4923.8 |####
   5139.1 |####
   5354.3 |####
   5569.5 |########
  (3 below, 1 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=31.5% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=52.2% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=49.3% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
