# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 51% (significant)

warm-container-lanes-deferred is -1.50 us (51%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.2x slower than the field

warm-container-plusone (4.97 us) is 3.2x the fastest (1.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.54)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest warm-container-native (1.58 us) to slowest warm-container-plusone (4.97 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-native vs stability leader warm-container-lanes-deferred (+1% speed for 1.2x steadier)

warm-container-native is fastest (1.58 us, CV 10.0%); warm-container-lanes-deferred gives up 1.0% median for 1.2x lower variance (CV 8.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-minimum is inconsistent: worst-20% is 4.1x its best-20%

warm-container-minimum's best 20% of batches run at 1.78 us but its worst 20% at 7.38 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 1575.2 ns median (-46.5% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 3.15x (fastest 1575.2 ns, slowest 4968.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4588ns | 3086ns | 2434ns | 4100ns | 8203ns | base |
| warm-container-kernel | 1978ns | 1970ns | 1784ns | 1939ns | 2289ns | -56.89% |
| warm-container-lanes-deferred | 1860ns | 1818ns | 1694ns | 1831ns | 2113ns | -59.45% |
| warm-container-minimum | 3871ns | 1857ns | 1843ns | 3264ns | 7721ns | -15.61% |
| warm-container-native | 1790ns | 1806ns | 1565ns | 1773ns | 2068ns | -60.97% |
| warm-container-plusone | 4407ns | 5144ns | 1848ns | 4380ns | 7047ns | -3.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4432ns | 2371ns | 7910ns | base | 31.426 |
| warm-container-kernel | 1715ns | 1557ns | 1969ns | -61.30% | 81.213 |
| warm-container-lanes-deferred | 1619ns | 1480ns | 1834ns | -63.47% | 86.020 |
| warm-container-minimum | 3724ns | 1780ns | 7383ns | -15.97% | 37.398 |
| warm-container-native | 1555ns | 1363ns | 1777ns | -64.90% | 89.536 |
| warm-container-plusone | 4245ns | 1785ns | 6763ns | -4.22% | 32.810 |

## Performance model

- Peak throughput: **102.150 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 47.316 | 46.3% |
| warm-container-kernel | 81.260 | 79.5% |
| warm-container-lanes-deferred | 87.527 | 85.7% |
| warm-container-minimum | 77.636 | 76.0% |
| warm-container-native | 88.408 | 86.5% |
| warm-container-plusone | 28.031 | 27.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4588ns | 4588ns | base |
| warm-container-kernel | 1978ns | 1978ns | -56.89% |
| warm-container-lanes-deferred | 1860ns | 1860ns | -59.45% |
| warm-container-minimum | 3871ns | 3871ns | -15.61% |
| warm-container-native | 1790ns | 1790ns | -60.97% |
| warm-container-plusone | 4407ns | 4407ns | -3.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2943ns | base | --- | [2379, 5974] | --- | --- | --- | --- |
| warm-container-kernel | 1714ns | -1286.3ns (-43.7%) | [-4081, -820]ns | [1585, 1735] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1591ns | -1495.5ns (-50.8%) | [-4264, -820]ns | [1562, 1607] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1794ns | -604.1ns (-20.5%) | [-1157, -588]ns | [1784, 5260] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-native | 1575ns | -1348.2ns (-45.8%) | [-4456, -1052]ns | [1471, 1589] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4968ns | -587.0ns (-19.9%) | [-1146, -335]ns | [1969, 5429] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2891ns | -43.2% | -45.0% | +36.5% | -45.3% | +79.1% |
| 1 | 2 | 2374ns | -33.4% | -33.0% | +22.8% | -39.4% | +100.4% |
| 1 | 3 | 2695ns | -35.4% | -34.4% | -33.8% | -45.7% | +94.0% |
| 1 | 4 | 2949ns | -39.4% | -48.6% | -39.5% | -45.8% | +12.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.265 | moderate- |
| warm-container-kernel | 0.168 | ok |
| warm-container-lanes-deferred | -0.538 | HIGH- (thermal bounce) |
| warm-container-minimum | 0.258 | moderate+ |
| warm-container-native | -0.241 | moderate- |
| warm-container-plusone | -0.264 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 50.4ns | 4431.5ns | 1.1% |  |
| warm-container-kernel | 94.0ns | 1714.8ns | 5.5% | HIGH |
| warm-container-lanes-deferred | 88.7ns | 1619.0ns | 5.5% | HIGH |
| warm-container-minimum | 50.4ns | 3723.8ns | 1.4% |  |
| warm-container-native | 84.1ns | 1555.4ns | 5.4% | HIGH |
| warm-container-plusone | 57.7ns | 4244.6ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2371.0-7910.5 ns)
   2371.0 |########################################
   2648.0 |
   2925.0 |#######################
   3201.9 |###
   3478.9 |
   3755.9 |
   4032.9 |
   4309.8 |
   4586.8 |
   4863.8 |###
   5140.8 |###
   5417.7 |
   5694.7 |
   5971.7 |
   6248.6 |
   6525.6 |###
   6802.6 |##########
   7079.6 |#############
   7356.5 |
   7633.5 |##########
  (4 below, 3 above range)

warm-container-kernel (n=40, range 1557.2-1968.9 ns)
   1557.2 |########################################
   1577.7 |#########################
   1598.3 |
   1618.9 |##########
   1639.5 |
   1660.1 |
   1680.7 |
   1701.3 |####################
   1721.9 |#########################
   1742.5 |##########
   1763.0 |
   1783.6 |#####
   1804.2 |#####
   1824.8 |
   1845.4 |
   1866.0 |#####
   1886.6 |#####
   1907.2 |
   1927.8 |
   1948.3 |#####
  (3 below, 6 above range)

warm-container-lanes-deferred (n=40, range 1480.0-1834.1 ns)
   1480.0 |
   1497.7 |
   1515.4 |
   1533.1 |###
   1550.8 |########################################
   1568.5 |
   1586.2 |################################
   1603.9 |#######
   1621.6 |###
   1639.3 |###
   1657.0 |###
   1674.7 |
   1692.4 |#######
   1710.1 |
   1727.8 |
   1745.5 |
   1763.2 |##########
   1780.9 |
   1798.7 |
   1816.4 |###
  (5 below, 3 above range)

warm-container-minimum (n=40, range 1780.1-7383.3 ns)
   1780.1 |########################################
   2060.2 |
   2340.4 |
   2620.6 |##
   2900.7 |
   3180.9 |
   3461.1 |
   3741.2 |
   4021.4 |##
   4301.5 |##
   4581.7 |
   4861.9 |
   5142.0 |#######
   5422.2 |##
   5702.4 |####
   5982.5 |##
   6262.7 |#########
   6542.8 |##
   6823.0 |##
   7103.2 |
  (5 below, 2 above range)

warm-container-native (n=40, range 1363.3-1777.4 ns)
   1363.3 |
   1384.0 |
   1404.7 |
   1425.4 |############################
   1446.1 |########
   1466.8 |####
   1487.6 |
   1508.3 |
   1529.0 |
   1549.7 |############
   1570.4 |########################################
   1591.1 |########
   1611.8 |########
   1632.5 |########
   1653.2 |####
   1673.9 |
   1694.6 |
   1715.3 |########
   1736.0 |
   1756.7 |
  (5 below, 3 above range)

warm-container-plusone (n=40, range 1784.7-6762.8 ns)
   1784.7 |########################################
   2033.6 |####
   2282.5 |
   2531.4 |
   2780.3 |
   3029.2 |
   3278.1 |
   3527.0 |
   3775.9 |
   4024.8 |####
   4273.7 |
   4522.6 |########
   4771.5 |################
   5020.5 |####
   5269.4 |################
   5518.3 |########
   5767.2 |############
   6016.1 |
   6265.0 |####################
   6513.9 |####
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=53.1% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-minimum**: CV=65.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-plusone**: CV=47.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
