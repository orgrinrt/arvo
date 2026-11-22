# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 16% faster than the next best (warm-container-lanes-deferred)

warm-container-kernel (1.52 us) leads warm-container-lanes-deferred (1.77 us) by 16%, a clear separation rather than a photo finish. CV 9.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 67% (significant)

warm-container-kernel is -2.86 us (67%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2.9x slower than the field

warm-container-minimum (4.40 us) is 2.9x the fastest (1.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} vs {warm-container-headroom, warm-container-plusone, warm-container-minimum} (135% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone, warm-container-minimum} with a 135% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-headroom is inconsistent: worst-20% is 4.2x its best-20%

warm-container-headroom's best 20% of batches run at 1.47 us but its worst 20% at 6.12 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-minimum's edge over baseline is significant but tiny (-13 ns, 0.31%)

warm-container-minimum differs from baseline warm-container-headroom by -13 ns (0.31%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 1523.0 ns median (-64.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.89x (fastest 1523.0 ns, slowest 4400.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112561731 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3785ns | 4442ns | 1519ns | 3674ns | 6384ns | base |
| warm-container-kernel | 1773ns | 1727ns | 1588ns | 1753ns | 2019ns | -53.15% |
| warm-container-lanes-deferred | 2038ns | 2004ns | 1615ns | 2039ns | 2456ns | -46.16% |
| warm-container-minimum | 3591ns | 4574ns | 1508ns | 3573ns | 5727ns | -5.11% |
| warm-container-native | 2127ns | 2073ns | 1762ns | 2098ns | 2578ns | -43.80% |
| warm-container-plusone | 3861ns | 4465ns | 1611ns | 3679ns | 6659ns | +2.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3643ns | 1467ns | 6118ns | base | 6.746 |
| warm-container-kernel | 1554ns | 1400ns | 1763ns | -57.35% | 15.816 |
| warm-container-lanes-deferred | 1784ns | 1421ns | 2106ns | -51.02% | 13.773 |
| warm-container-minimum | 3454ns | 1456ns | 5470ns | -5.18% | 7.115 |
| warm-container-native | 1864ns | 1552ns | 2247ns | -48.82% | 13.182 |
| warm-container-plusone | 3716ns | 1556ns | 6399ns | +2.00% | 6.614 |

## Performance model

- Peak throughput: **17.556 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.728 | 32.6% |
| warm-container-kernel | 16.137 | 91.9% |
| warm-container-lanes-deferred | 13.897 | 79.2% |
| warm-container-minimum | 5.585 | 31.8% |
| warm-container-native | 13.440 | 76.6% |
| warm-container-plusone | 5.699 | 32.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3785ns | 3785ns | base |
| warm-container-kernel | 1773ns | 1773ns | -53.15% |
| warm-container-lanes-deferred | 2038ns | 2038ns | -46.16% |
| warm-container-minimum | 3591ns | 3591ns | -5.11% |
| warm-container-native | 2127ns | 2127ns | -43.80% |
| warm-container-plusone | 3861ns | 3861ns | +2.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4290ns | base | --- | [1793, 4966] | --- | --- | --- | --- |
| warm-container-kernel | 1523ns | -2856.2ns (-66.6%) | [-3308, -369]ns | [1464, 1596] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1768ns | -2339.6ns (-54.5%) | [-3082, -59]ns | [1751, 1886] | YES | 0.0005 | 0.0002 | 0 |
| warm-container-minimum | 4401ns | -13.1ns (-0.3%) | [-291, -3]ns | [1468, 4624] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-container-native | 1829ns | no significant difference | [-3151, +41]ns | [1806, 1894] | no | 0.4296 | 0.4296 | 0 |
| warm-container-plusone | 4312ns | no significant difference | [-231, +3]ns | [1791, 4726] | no | 0.1009 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3069ns | -51.5% | -54.1% | +46.0% | -49.6% | +48.0% |
| 1 | 2 | 4149ns | -59.1% | -56.0% | -54.7% | -56.4% | +3.9% |
| 1 | 3 | 4290ns | -66.0% | -53.6% | +2.9% | -57.3% | -40.6% |
| 1 | 4 | 3531ns | -58.5% | -48.8% | +27.2% | -42.4% | -13.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.191 | ok |
| warm-container-kernel | -0.360 | moderate- |
| warm-container-lanes-deferred | 0.021 | ok |
| warm-container-minimum | -0.408 | moderate- |
| warm-container-native | 0.049 | ok |
| warm-container-plusone | 0.176 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 49.7ns | 3643.0ns | 1.4% |  |
| warm-container-kernel | 78.5ns | 1553.8ns | 5.1% | HIGH |
| warm-container-lanes-deferred | 89.8ns | 1784.3ns | 5.0% | HIGH |
| warm-container-minimum | 46.8ns | 3454.3ns | 1.4% |  |
| warm-container-native | 89.0ns | 1864.4ns | 4.8% |  |
| warm-container-plusone | 50.5ns | 3715.8ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1466.8-6118.4 ns)
   1466.8 |########################################
   1699.4 |#########################
   1932.0 |
   2164.6 |
   2397.1 |
   2629.7 |
   2862.3 |
   3094.9 |
   3327.4 |#####
   3560.0 |#####
   3792.6 |
   4025.2 |
   4257.8 |##########
   4490.3 |###############
   4722.9 |#####
   4955.5 |####################
   5188.1 |###############
   5420.6 |##########
   5653.2 |#####
   5885.8 |#####
  (4 below, 4 above range)

warm-container-kernel (n=40, range 1399.9-1762.6 ns)
   1399.9 |########
   1418.0 |################
   1436.1 |
   1454.3 |########################################
   1472.4 |
   1490.5 |
   1508.7 |########
   1526.8 |####
   1544.9 |########
   1563.1 |####
   1581.2 |############
   1599.4 |########
   1617.5 |
   1635.6 |####
   1653.8 |
   1671.9 |################
   1690.0 |########
   1708.2 |
   1726.3 |
   1744.5 |########
  (2 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1421.3-2106.2 ns)
   1421.3 |
   1455.6 |
   1489.8 |#####
   1524.1 |#################
   1558.3 |
   1592.5 |
   1626.8 |
   1661.0 |#################
   1695.3 |
   1729.5 |########################################
   1763.7 |#####
   1798.0 |#####
   1832.2 |#####
   1866.5 |######################
   1900.7 |#####
   1935.0 |##################################
   1969.2 |
   2003.4 |#####
   2037.7 |
   2071.9 |#####
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1456.4-5469.9 ns)
   1456.4 |########################################
   1657.1 |
   1857.7 |
   2058.4 |####
   2259.1 |####
   2459.8 |####
   2660.4 |
   2861.1 |
   3061.8 |
   3262.5 |
   3463.1 |
   3663.8 |
   3864.5 |
   4065.1 |
   4265.8 |############
   4466.5 |########################
   4667.2 |
   4867.8 |####################
   5068.5 |########
   5269.2 |####
  (5 below, 5 above range)

warm-container-native (n=40, range 1552.1-2246.9 ns)
   1552.1 |###
   1586.9 |
   1621.6 |##############
   1656.3 |
   1691.1 |
   1725.8 |
   1760.5 |
   1795.3 |########################################
   1830.0 |##############
   1864.8 |#######
   1899.5 |#######
   1934.2 |
   1969.0 |###
   2003.7 |##############
   2038.5 |#######
   2073.2 |
   2107.9 |
   2142.7 |
   2177.4 |
   2212.1 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 1555.5-6398.9 ns)
   1555.5 |########################################
   1797.7 |########
   2039.9 |
   2282.0 |
   2524.2 |
   2766.4 |
   3008.5 |
   3250.7 |####
   3492.9 |
   3735.0 |
   3977.2 |####
   4219.4 |#################
   4461.5 |#############
   4703.7 |#################
   4945.9 |
   5188.1 |########
   5430.2 |
   5672.4 |########
   5914.6 |
   6156.7 |
  (6 below, 6 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=52.3% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=49.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=51.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.1x (possible bimodal distribution)
