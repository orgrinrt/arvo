# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 48% (significant)

warm-container-lanes-deferred is -1.15 us (48%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.0x slower than the field

warm-container-plusone (4.26 us) is 3.0x the fastest (1.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Speed leader warm-container-kernel vs stability leader warm-container-lanes-deferred (+10% speed for 1.3x steadier)

warm-container-kernel is fastest (1.44 us, CV 12.0%); warm-container-lanes-deferred gives up 9.7% median for 1.3x lower variance (CV 9.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 3.9x its best-20%

warm-container-headroom's best 20% of batches run at 2.05 us but its worst 20% at 8.05 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1443.9 ns median (-39.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.95x (fastest 1443.9 ns, slowest 4263.3 ns)

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
| Counter frequency | 2112850797 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4077ns | 2441ns | 2103ns | 3327ns | 8304ns | base |
| warm-container-kernel | 1735ns | 1652ns | 1520ns | 1700ns | 2054ns | -57.45% |
| warm-container-lanes-deferred | 1714ns | 1810ns | 1420ns | 1749ns | 1902ns | -57.97% |
| warm-container-minimum | 3775ns | 4189ns | 1513ns | 3820ns | 5903ns | -7.41% |
| warm-container-native | 1801ns | 1792ns | 1418ns | 1747ns | 2348ns | -55.83% |
| warm-container-plusone | 3814ns | 4413ns | 1633ns | 3674ns | 6415ns | -6.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3961ns | 2049ns | 8047ns | base | 35.160 |
| warm-container-kernel | 1509ns | 1326ns | 1762ns | -61.91% | 92.313 |
| warm-container-lanes-deferred | 1489ns | 1238ns | 1628ns | -62.40% | 93.508 |
| warm-container-minimum | 3638ns | 1462ns | 5665ns | -8.16% | 38.286 |
| warm-container-native | 1565ns | 1239ns | 2028ns | -60.48% | 88.975 |
| warm-container-plusone | 3674ns | 1578ns | 6172ns | -7.23% | 37.901 |

## Performance model

- Peak throughput: **112.465 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 58.568 | 52.1% |
| warm-container-kernel | 96.450 | 85.8% |
| warm-container-lanes-deferred | 87.958 | 78.2% |
| warm-container-minimum | 34.419 | 30.6% |
| warm-container-native | 88.918 | 79.1% |
| warm-container-plusone | 32.666 | 29.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4077ns | 4077ns | base |
| warm-container-kernel | 1735ns | 1735ns | -57.45% |
| warm-container-lanes-deferred | 1714ns | 1714ns | -57.97% |
| warm-container-minimum | 3775ns | 3775ns | -7.41% |
| warm-container-native | 1801ns | 1801ns | -55.83% |
| warm-container-plusone | 3814ns | 3814ns | -6.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2378ns | base | --- | [2371, 2647] | --- | --- | --- | --- |
| warm-container-kernel | 1444ns | -974.2ns (-41.0%) | [-1238, -928]ns | [1439, 1577] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1583ns | -1150.9ns (-48.4%) | [-1233, -788]ns | [1442, 1586] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4046ns | no significant difference | [-911, +94]ns | [2398, 4627] | no | 0.0807 | 0.0807 | 0 |
| warm-container-native | 1566ns | -891.4ns (-37.5%) | [-1051, -803]ns | [1425, 1584] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4263ns | -592.8ns (-24.9%) | [-911, -480]ns | [1795, 4574] | YES | 0.0207 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2397ns | -29.5% | -49.1% | +75.5% | -34.6% | +82.8% |
| 1 | 2 | 2384ns | -39.6% | -33.5% | +69.4% | -17.2% | -25.0% |
| 1 | 3 | 2372ns | -39.2% | -33.1% | +44.5% | -39.6% | +105.7% |
| 1 | 4 | 1945ns | -26.0% | -18.5% | +131.7% | -36.2% | +119.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.055 | ok |
| warm-container-kernel | -0.084 | ok |
| warm-container-lanes-deferred | -0.084 | ok |
| warm-container-minimum | -0.454 | moderate- |
| warm-container-native | -0.025 | ok |
| warm-container-plusone | -0.489 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 0/4, lost 4/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 40.3ns | 3960.8ns | 1.0% |  |
| warm-container-kernel | 81.3ns | 1508.6ns | 5.4% | HIGH |
| warm-container-lanes-deferred | 78.9ns | 1489.3ns | 5.3% | HIGH |
| warm-container-minimum | 48.2ns | 3637.5ns | 1.3% |  |
| warm-container-native | 83.5ns | 1565.2ns | 5.3% | HIGH |
| warm-container-plusone | 50.2ns | 3674.4ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2048.9-8046.9 ns)
   2048.9 |
   2348.8 |########################################
   2648.7 |##
   2948.6 |
   3248.5 |
   3548.4 |
   3848.3 |
   4148.2 |##
   4448.1 |
   4748.0 |
   5047.9 |
   5347.8 |
   5647.7 |
   5947.6 |
   6247.5 |##
   6547.4 |
   6847.3 |####
   7147.2 |####
   7447.1 |##
   7747.0 |######
  (6 below, 3 above range)

warm-container-kernel (n=40, range 1325.6-1761.8 ns)
   1325.6 |
   1347.4 |############
   1369.3 |
   1391.1 |
   1412.9 |####
   1434.7 |########################################
   1456.5 |####
   1478.3 |####
   1500.1 |########
   1521.9 |
   1543.7 |
   1565.5 |############################
   1587.4 |########
   1609.2 |####
   1631.0 |
   1652.8 |
   1674.6 |
   1696.4 |
   1718.2 |
   1740.0 |########
  (7 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1238.3-1627.7 ns)
   1238.3 |
   1257.8 |##
   1277.2 |
   1296.7 |
   1316.2 |######
   1335.6 |
   1355.1 |
   1374.6 |
   1394.0 |
   1413.5 |
   1433.0 |############
   1452.4 |
   1471.9 |
   1491.4 |
   1510.8 |
   1530.3 |
   1549.8 |
   1569.3 |########################################
   1588.7 |##
   1608.2 |
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1461.6-5665.0 ns)
   1461.6 |################
   1671.8 |################################
   1881.9 |
   2092.1 |########
   2302.3 |
   2512.4 |########
   2722.6 |
   2932.8 |
   3143.0 |########
   3353.1 |
   3563.3 |
   3773.5 |
   3983.6 |########################################
   4193.8 |################
   4404.0 |################
   4614.1 |########################
   4824.3 |########################
   5034.5 |################
   5244.6 |########################
   5454.8 |########
  (7 below, 3 above range)

warm-container-native (n=40, range 1239.2-2028.0 ns)
   1239.2 |#####
   1278.6 |####################
   1318.1 |##########
   1357.5 |
   1397.0 |###############
   1436.4 |###############
   1475.8 |
   1515.3 |#####
   1554.7 |########################################
   1594.1 |##########
   1633.6 |
   1673.0 |
   1712.5 |#####
   1751.9 |##########
   1791.3 |#####
   1830.8 |
   1870.2 |#####
   1909.7 |
   1949.1 |###############
   1988.5 |
  (5 below, 3 above range)

warm-container-plusone (n=40, range 1578.0-6171.5 ns)
   1578.0 |########################################
   1807.6 |####
   2037.3 |
   2267.0 |
   2496.7 |
   2726.3 |####
   2956.0 |
   3185.7 |####
   3415.4 |
   3645.1 |
   3874.7 |####
   4104.4 |############
   4334.1 |############
   4563.8 |########
   4793.5 |################
   5023.1 |####
   5252.8 |####
   5482.5 |
   5712.2 |####
   5941.8 |####
  (5 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=62.8% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-minimum**: CV=45.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-plusone**: CV=48.9% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.9x (possible bimodal distribution)
