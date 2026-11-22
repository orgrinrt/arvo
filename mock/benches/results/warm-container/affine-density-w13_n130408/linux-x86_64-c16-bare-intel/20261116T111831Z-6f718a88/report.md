# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (2.35 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 33% (significant)

warm-container-kernel is -770 ns (33%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-container-kernel, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-kernel (1.58 us) and warm-container-lanes-deferred (1.59 us) differ by 0.29%, inside the noise, even though the wider field spreads 48.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.53)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-native, warm-container-minimum, warm-container-plusone} vs {warm-container-headroom} (31% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-native, warm-container-minimum, warm-container-plusone} and a slow tier {warm-container-headroom} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-kernel vs stability leader warm-container-lanes-deferred (+0% speed for 1.5x steadier)

warm-container-kernel is fastest (1.58 us, CV 8.7%); warm-container-lanes-deferred gives up 0.3% median for 1.5x lower variance (CV 5.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-minimum is inconsistent: worst-20% is 4.6x its best-20%

warm-container-minimum's best 20% of batches run at 1.50 us but its worst 20% at 6.88 us (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1583.8 ns median (-32.7% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.49x (fastest 1583.8 ns, slowest 2353.7 ns)

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
| warm-container-headroom | 3814ns | 2416ns | 1976ns | 2895ns | 8406ns | base |
| warm-container-kernel | 1863ns | 1814ns | 1659ns | 1838ns | 2143ns | -51.14% |
| warm-container-lanes-deferred | 1876ns | 1816ns | 1757ns | 1845ns | 2088ns | -50.80% |
| warm-container-minimum | 3393ns | 1848ns | 1552ns | 2739ns | 7196ns | -11.03% |
| warm-container-native | 2085ns | 1842ns | 1584ns | 1895ns | 3155ns | -45.33% |
| warm-container-plusone | 3617ns | 1853ns | 1550ns | 3197ns | 6948ns | -5.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3701ns | 1924ns | 8124ns | base | 19.919 |
| warm-container-kernel | 1616ns | 1447ns | 1830ns | -56.33% | 45.617 |
| warm-container-lanes-deferred | 1629ns | 1538ns | 1782ns | -55.98% | 45.252 |
| warm-container-minimum | 3263ns | 1499ns | 6882ns | -11.85% | 22.596 |
| warm-container-native | 1825ns | 1385ns | 2828ns | -50.69% | 40.396 |
| warm-container-plusone | 3480ns | 1497ns | 6643ns | -5.98% | 21.186 |

## Performance model

- Peak throughput: **53.230 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 31.324 | 58.8% |
| warm-container-kernel | 46.550 | 87.4% |
| warm-container-lanes-deferred | 46.415 | 87.2% |
| warm-container-minimum | 41.318 | 77.6% |
| warm-container-native | 46.406 | 87.2% |
| warm-container-plusone | 41.191 | 77.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3814ns | 3814ns | base |
| warm-container-kernel | 1863ns | 1863ns | -51.14% |
| warm-container-lanes-deferred | 1876ns | 1876ns | -50.80% |
| warm-container-minimum | 3393ns | 3393ns | -11.03% |
| warm-container-native | 2085ns | 2085ns | -45.33% |
| warm-container-plusone | 3617ns | 3617ns | -5.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2354ns | base | --- | [2352, 2368] | --- | --- | --- | --- |
| warm-container-kernel | 1584ns | -770.1ns (-32.7%) | [-853, -599]ns | [1580, 1586] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1588ns | -765.7ns (-32.5%) | [-817, -612]ns | [1588, 1611] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1784ns | -576.7ns (-24.5%) | [-892, -143]ns | [1777, 2712] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-native | 1589ns | -763.1ns (-32.4%) | [-918, -587]ns | [1584, 1764] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1790ns | -227.0ns (-9.6%) | [-589, -146]ns | [1779, 5019] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2362ns | -28.2% | -32.7% | -24.5% | -16.0% | +8.1% |
| 1 | 2 | 3548ns | -55.5% | -55.2% | -49.7% | -58.6% | -19.9% |
| 1 | 3 | 1930ns | -18.0% | -17.7% | +40.2% | -15.3% | +19.8% |
| 1 | 4 | 2352ns | -32.7% | -28.7% | -37.9% | -36.3% | -38.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.470 | moderate- |
| warm-container-kernel | -0.113 | ok |
| warm-container-lanes-deferred | -0.086 | ok |
| warm-container-minimum | -0.529 | HIGH- (thermal bounce) |
| warm-container-native | -0.336 | moderate- |
| warm-container-plusone | 0.129 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 39.2ns | 3701.4ns | 1.1% |  |
| warm-container-kernel | 88.6ns | 1616.2ns | 5.5% | HIGH |
| warm-container-lanes-deferred | 89.0ns | 1629.3ns | 5.5% | HIGH |
| warm-container-minimum | 44.7ns | 3262.8ns | 1.4% |  |
| warm-container-native | 90.6ns | 1825.1ns | 5.0% |  |
| warm-container-plusone | 48.7ns | 3480.0ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1924.1-8123.6 ns)
   1924.1 |#####################
   2234.1 |########################################
   2544.0 |
   2854.0 |
   3164.0 |
   3474.0 |##
   3783.9 |##
   4093.9 |
   4403.9 |
   4713.9 |
   5023.8 |##
   5333.8 |
   5643.8 |##
   5953.8 |##
   6263.7 |##
   6573.7 |##
   6883.7 |
   7193.7 |##
   7503.6 |##
   7813.6 |
  (4 below, 4 above range)

warm-container-kernel (n=40, range 1447.2-1830.5 ns)
   1447.2 |
   1466.3 |
   1485.5 |
   1504.7 |
   1523.8 |
   1543.0 |
   1562.2 |########################################
   1581.3 |################################
   1600.5 |
   1619.7 |##########
   1638.8 |
   1658.0 |
   1677.1 |
   1696.3 |
   1715.5 |
   1734.6 |##########
   1753.8 |##########
   1773.0 |
   1792.1 |
   1811.3 |#######
  (7 below, 2 above range)

warm-container-lanes-deferred (n=40, range 1537.8-1781.8 ns)
   1537.8 |
   1550.0 |
   1562.2 |##
   1574.4 |###########
   1586.6 |########################################
   1598.8 |
   1611.0 |
   1623.2 |##
   1635.4 |####
   1647.6 |
   1659.8 |
   1672.0 |##
   1684.2 |
   1696.4 |
   1708.6 |####
   1720.8 |
   1733.0 |
   1745.2 |##
   1757.4 |###########
   1769.6 |
  (3 below, 2 above range)

warm-container-minimum (n=40, range 1498.5-6882.4 ns)
   1498.5 |
   1767.7 |########################################
   2036.9 |
   2306.1 |
   2575.3 |
   2844.5 |
   3113.7 |
   3382.9 |##
   3652.1 |
   3921.3 |
   4190.5 |##
   4459.7 |
   4728.9 |
   4998.1 |
   5267.3 |##
   5536.4 |
   5805.6 |####
   6074.8 |##
   6344.0 |########
   6613.2 |
  (7 below, 4 above range)

warm-container-native (n=40, range 1385.1-2828.0 ns)
   1385.1 |#########################
   1457.2 |###
   1529.4 |########################################
   1601.5 |###
   1673.7 |#######
   1745.8 |#####################
   1817.9 |#######
   1890.1 |
   1962.2 |#######
   2034.4 |#######
   2106.5 |
   2178.7 |
   2250.8 |###
   2323.0 |###
   2395.1 |
   2467.2 |
   2539.4 |
   2611.5 |
   2683.7 |
   2755.8 |
  (3 below, 1 above range)

warm-container-plusone (n=40, range 1496.6-6642.9 ns)
   1496.6 |
   1753.9 |########################################
   2011.2 |
   2268.5 |
   2525.9 |
   2783.2 |##
   3040.5 |
   3297.8 |##
   3555.1 |
   3812.4 |##
   4069.7 |
   4327.0 |##
   4584.4 |
   4841.7 |
   5099.0 |
   5356.3 |
   5613.6 |#############
   5870.9 |##
   6128.2 |##
   6385.5 |#############
  (7 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=67.5% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=68.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-native**: CV=56.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=62.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.4x (possible bimodal distribution)
