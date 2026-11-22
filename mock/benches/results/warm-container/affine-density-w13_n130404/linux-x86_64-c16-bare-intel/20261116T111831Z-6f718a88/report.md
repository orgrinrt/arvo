# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.39 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 14% faster than the next best (warm-container-native)

warm-container-kernel (1.59 us) leads warm-container-native (1.81 us) by 14%, a clear separation rather than a photo finish. CV 18.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 64% (significant)

warm-container-kernel is -2.80 us (64%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.8x slower than the field

warm-container-headroom (4.39 us) is 2.8x the fastest (1.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.68)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-lanes-deferred} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (98% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-lanes-deferred} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 98% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone is inconsistent: worst-20% is 4.2x its best-20%

warm-container-plusone's best 20% of batches run at 1.46 us but its worst 20% at 6.18 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1591.3 ns median (-63.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.76x (fastest 1591.3 ns, slowest 4386.4 ns)

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
| warm-container-headroom | 3808ns | 4542ns | 1606ns | 3644ns | 6502ns | base |
| warm-container-kernel | 1749ns | 1788ns | 1239ns | 1788ns | 2141ns | -54.08% |
| warm-container-lanes-deferred | 2282ns | 2260ns | 2035ns | 2229ns | 2689ns | -40.07% |
| warm-container-minimum | 3717ns | 4109ns | 1508ns | 3615ns | 6235ns | -2.38% |
| warm-container-native | 2058ns | 2036ns | 1860ns | 2033ns | 2332ns | -45.96% |
| warm-container-plusone | 3862ns | 4262ns | 1508ns | 3781ns | 6460ns | +1.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3666ns | 1551ns | 6218ns | base | 11.173 |
| warm-container-kernel | 1543ns | 1096ns | 1876ns | -57.90% | 26.539 |
| warm-container-lanes-deferred | 2009ns | 1809ns | 2349ns | -45.19% | 20.386 |
| warm-container-minimum | 3578ns | 1456ns | 5975ns | -2.41% | 11.449 |
| warm-container-native | 1816ns | 1647ns | 2028ns | -50.46% | 22.552 |
| warm-container-plusone | 3715ns | 1456ns | 6181ns | +1.33% | 11.027 |

## Performance model

- Peak throughput: **37.361 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.338 | 25.0% |
| warm-container-kernel | 25.739 | 68.9% |
| warm-container-lanes-deferred | 20.397 | 54.6% |
| warm-container-minimum | 10.321 | 27.6% |
| warm-container-native | 22.647 | 60.6% |
| warm-container-plusone | 9.950 | 26.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3808ns | 3808ns | base |
| warm-container-kernel | 1749ns | 1749ns | -54.08% |
| warm-container-lanes-deferred | 2282ns | 2282ns | -40.07% |
| warm-container-minimum | 3717ns | 3717ns | -2.38% |
| warm-container-native | 2058ns | 2058ns | -45.96% |
| warm-container-plusone | 3862ns | 3862ns | +1.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4386ns | base | --- | [1792, 4972] | --- | --- | --- | --- |
| warm-container-kernel | 1591ns | -2797.4ns (-63.8%) | [-3215, -700]ns | [1511, 1677] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 2008ns | no significant difference | [-2818, +17]ns | [1831, 2046] | no | 0.6358 | 0.6358 | 0 |
| warm-container-minimum | 3969ns | no significant difference | [-336, +84]ns | [1787, 4956] | no | 0.2564 | 0.1539 | 0 |
| warm-container-native | 1809ns | -2578.5ns (-58.8%) | [-2951, -136]ns | [1806, 1810] | YES | 0.0161 | 0.0064 | 0 |
| warm-container-plusone | 4116ns | no significant difference | [-15, +308]ns | [1782, 5300] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4688ns | -67.3% | -55.8% | -9.3% | -62.1% | -63.3% |
| 1 | 2 | 3074ns | -63.3% | -34.9% | -13.0% | -41.2% | +71.3% |
| 1 | 3 | 3291ns | -44.8% | -37.8% | -11.1% | -42.3% | -17.6% |
| 1 | 4 | 1799ns | -8.5% | +0.6% | +151.0% | -3.5% | +119.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.078 | ok |
| warm-container-kernel | -0.323 | moderate- |
| warm-container-lanes-deferred | -0.193 | ok |
| warm-container-minimum | -0.237 | moderate- |
| warm-container-native | -0.436 | moderate- |
| warm-container-plusone | -0.680 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 52.0ns | 3665.9ns | 1.4% |  |
| warm-container-kernel | 73.8ns | 1543.4ns | 4.8% |  |
| warm-container-lanes-deferred | 95.2ns | 2009.2ns | 4.7% |  |
| warm-container-minimum | 48.6ns | 3577.6ns | 1.4% |  |
| warm-container-native | 87.1ns | 1816.3ns | 4.8% |  |
| warm-container-plusone | 50.6ns | 3714.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1550.9-6218.0 ns)
   1550.9 |
   1784.3 |########################################
   2017.6 |###
   2251.0 |
   2484.3 |
   2717.7 |
   2951.1 |
   3184.4 |
   3417.8 |
   3651.1 |
   3884.5 |
   4117.8 |###
   4351.2 |######
   4584.5 |##########
   4817.9 |##########
   5051.2 |
   5284.6 |##########
   5517.9 |######
   5751.3 |######
   5984.7 |###
  (6 below, 4 above range)

warm-container-kernel (n=40, range 1096.3-1876.4 ns)
   1096.3 |#########################
   1135.3 |
   1174.3 |#####
   1213.3 |
   1252.3 |
   1291.3 |
   1330.4 |
   1369.4 |#####
   1408.4 |
   1447.4 |#####
   1486.4 |#########################
   1525.4 |##########
   1564.4 |##########
   1603.4 |
   1642.4 |#########################
   1681.4 |#####
   1720.4 |#####
   1759.4 |
   1798.4 |########################################
   1837.4 |##########
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1808.5-2349.2 ns)
   1808.5 |########################################
   1835.6 |##########
   1862.6 |###
   1889.7 |
   1916.7 |
   1943.7 |
   1970.8 |###
   1997.8 |#########################
   2024.8 |
   2051.9 |###
   2078.9 |###
   2105.9 |##########
   2133.0 |
   2160.0 |
   2187.0 |
   2214.1 |###
   2241.1 |##########
   2268.1 |###
   2295.2 |
   2322.2 |#######
  (3 below, 2 above range)

warm-container-minimum (n=40, range 1455.9-5974.8 ns)
   1455.9 |########################################
   1681.9 |#################################
   1907.8 |
   2133.8 |######
   2359.7 |
   2585.7 |
   2811.6 |
   3037.5 |
   3263.5 |
   3489.4 |######
   3715.4 |######
   3941.3 |######
   4167.3 |######
   4393.2 |
   4619.2 |#############
   4845.1 |##########################
   5071.0 |
   5297.0 |#################################
   5522.9 |
   5748.9 |##########################
  (6 below, 3 above range)

warm-container-native (n=40, range 1647.5-2028.4 ns)
   1647.5 |##
   1666.5 |##
   1685.6 |##
   1704.6 |
   1723.7 |
   1742.7 |##
   1761.8 |
   1780.8 |##
   1799.9 |########################################
   1818.9 |
   1838.0 |
   1857.0 |#######
   1876.1 |##
   1895.1 |##
   1914.2 |
   1933.2 |##
   1952.2 |
   1971.3 |
   1990.3 |####
   2009.4 |
  (7 below, 3 above range)

warm-container-plusone (n=40, range 1456.3-6181.4 ns)
   1456.3 |###############
   1692.6 |########################################
   1928.8 |#####
   2165.1 |
   2401.4 |
   2637.6 |
   2873.9 |#####
   3110.1 |
   3346.4 |
   3582.6 |#####
   3818.9 |
   4055.1 |
   4291.4 |
   4527.6 |#####
   4763.9 |##########
   5000.2 |##########
   5236.4 |####################
   5472.7 |#####
   5708.9 |#########################
   5945.2 |#####
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=52.7% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-minimum**: CV=52.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-plusone**: CV=53.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.2x (possible bimodal distribution)
