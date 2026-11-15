# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 70% (significant)

warm-container-native is -1.17 us (70%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.3x slower than the field

warm-container-plusone (1.66 us) is 3.3x the fastest (497 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.88)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (230% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 230% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest warm-container-native (497 ns) to slowest warm-container-plusone (1.66 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone's edge over baseline is significant but tiny (-7 ns, 0.40%)

warm-container-plusone differs from baseline warm-container-headroom by -7 ns (0.40%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 497.1 ns median (-70.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.34x (fastest 497.1 ns, slowest 1662.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 1754ns | 1730ns | 1705ns | 1727ns | 1883ns | base |
| warm-container-kernel | 584ns | 565ns | 557ns | 573ns | 643ns | -66.70% |
| warm-container-minimum | 574ns | 568ns | 555ns | 568ns | 609ns | -67.29% |
| warm-container-native | 561ns | 558ns | 554ns | 560ns | 572ns | -68.01% |
| warm-container-plusone | 1812ns | 1734ns | 1707ns | 1758ns | 2078ns | +3.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1688ns | 1645ns | 1811ns | base | 24.261 |
| warm-container-kernel | 517ns | 494ns | 564ns | -69.35% | 79.165 |
| warm-container-minimum | 510ns | 494ns | 545ns | -69.79% | 80.303 |
| warm-container-native | 500ns | 494ns | 510ns | -70.41% | 81.988 |
| warm-container-plusone | 1743ns | 1644ns | 2003ns | +3.22% | 23.505 |

## Performance model

- Peak throughput: **82.967 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 24.678 | 29.7% |
| warm-container-kernel | 81.448 | 98.2% |
| warm-container-minimum | 81.351 | 98.1% |
| warm-container-native | 82.398 | 99.3% |
| warm-container-plusone | 24.641 | 29.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 1754ns | 1754ns | base |
| warm-container-kernel | 584ns | 584ns | -66.70% |
| warm-container-minimum | 574ns | 574ns | -67.29% |
| warm-container-native | 561ns | 561ns | -68.01% |
| warm-container-plusone | 1812ns | 1812ns | +3.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1660ns | base | --- | [1647, 1675] | --- | --- | --- | --- |
| warm-container-kernel | 503ns | -1149.0ns (-69.2%) | [-1153, -1142]ns | [498, 507] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 504ns | -1152.9ns (-69.5%) | [-1155, -1150]ns | [497, 506] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 497ns | -1163.9ns (-70.1%) | [-1169, -1151]ns | [496, 501] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1662ns | no significant difference | [-2, +14]ns | [1647, 1682] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1713ns | -71.0% | -71.2% | -71.0% | -3.9% |
| 2 | 1646ns | -70.1% | -70.1% | -69.9% | -0.1% |
| 3 | 1643ns | -70.0% | -69.9% | -70.0% | +0.7% |
| 4 | 1647ns | -70.0% | -69.8% | -69.8% | +4.1% |
| 5 | 1658ns | -70.3% | -70.0% | -70.1% | +0.5% |
| 6 | 1649ns | -69.9% | -69.9% | -70.1% | -0.2% |
| 7 | 1645ns | -69.8% | -69.9% | -69.9% | +0.0% |
| 8 | 1647ns | -69.9% | -69.9% | -70.0% | +0.1% |
| 9 | 1646ns | -70.0% | -70.1% | -69.8% | +0.1% |
| 10 | 1645ns | -69.7% | -69.8% | -69.9% | +0.1% |
| 11 | 1646ns | -69.3% | -69.4% | -69.4% | +0.8% |
| 12 | 1647ns | -69.4% | -69.4% | -69.5% | +1.4% |
| 13 | 1645ns | -68.2% | -69.4% | -69.9% | +2.0% |
| 14 | 1647ns | -69.5% | -69.3% | -69.8% | -0.1% |
| 15 | 1658ns | -69.7% | -69.5% | -70.2% | +0.9% |
| 16 | 1842ns | -72.6% | -72.5% | -73.3% | -8.4% |
| 17 | 1869ns | -73.1% | -73.0% | -73.4% | -9.7% |
| 18 | 1933ns | -73.9% | -73.8% | -74.2% | -13.3% |
| 19 | 1865ns | -73.1% | -72.8% | -73.0% | -10.4% |
| 20 | 1864ns | -72.4% | -69.0% | -73.0% | -9.0% |
| 21 | 1689ns | -66.7% | -69.2% | -68.9% | +18.6% |
| 22 | 1680ns | -66.4% | -68.8% | -69.5% | +19.0% |
| 23 | 1678ns | -66.6% | -69.0% | -69.8% | +19.6% |
| 24 | 1674ns | -66.5% | -69.0% | -69.7% | +19.5% |
| 25 | 1672ns | -66.4% | -68.8% | -69.9% | +19.8% |
| 26 | 1673ns | -66.0% | -68.8% | -69.7% | +19.5% |
| 27 | 1678ns | -66.2% | -68.8% | -69.7% | +19.5% |
| 28 | 1675ns | -66.4% | -68.9% | -69.9% | +19.3% |
| 29 | 1675ns | -66.4% | -68.8% | -69.9% | +19.7% |
| 30 | 1673ns | -66.3% | -68.8% | -69.8% | +15.1% |
| 31 | 1646ns | -70.0% | -69.9% | -69.9% | -0.1% |
| 32 | 1646ns | -69.7% | -70.1% | -69.9% | -0.1% |
| 33 | 1648ns | -66.0% | -69.9% | -69.8% | -0.2% |
| 34 | 1645ns | -69.0% | -69.9% | -69.9% | +0.2% |
| 35 | 1679ns | -70.5% | -70.5% | -70.5% | -1.9% |
| 36 | 1645ns | -69.7% | -69.9% | -69.8% | +0.2% |
| 37 | 1715ns | -71.0% | -70.9% | -71.2% | -4.0% |
| 38 | 1678ns | -70.4% | -61.8% | -70.4% | -1.8% |
| 39 | 1647ns | -70.0% | -69.9% | -69.7% | -0.3% |
| 40 | 1661ns | -70.0% | -69.7% | -70.3% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.790 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.763 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.072 | ok |
| warm-container-native | 0.648 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.879 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 14/40, lost 20/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.0ns | 1688.3ns | 0.2% |  |
| warm-container-kernel | 2.6ns | 517.4ns | 0.5% |  |
| warm-container-minimum | 2.7ns | 510.1ns | 0.5% |  |
| warm-container-native | 2.7ns | 499.6ns | 0.5% |  |
| warm-container-plusone | 2.7ns | 1742.6ns | 0.2% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1645.0-1811.2 ns)
   1645.0 |########################################
   1653.3 |#######
   1661.6 |
   1669.9 |####################
   1678.2 |#######
   1686.5 |##
   1694.9 |
   1703.2 |
   1711.5 |#####
   1719.8 |
   1728.1 |
   1736.4 |
   1744.7 |
   1753.0 |
   1761.3 |
   1769.6 |
   1778.0 |
   1786.3 |
   1794.6 |
   1802.9 |
  (2 below, 5 above range)

warm-container-kernel (n=40, range 493.8-564.0 ns)
    493.8 |########################################
    497.3 |####################
    500.8 |########################
    504.3 |########
    507.8 |####
    511.3 |
    514.8 |####
    518.3 |
    521.8 |####
    525.4 |
    528.9 |
    532.4 |
    535.9 |
    539.4 |
    542.9 |
    546.4 |
    549.9 |
    553.4 |
    557.0 |####
    560.5 |########################
  (3 below, 4 above range)

warm-container-minimum (n=40, range 493.8-544.5 ns)
    493.8 |########################################
    496.3 |######################
    498.9 |
    501.4 |#############
    503.9 |##########################
    506.5 |####
    509.0 |
    511.6 |
    514.1 |
    516.6 |
    519.2 |#################
    521.7 |##########################
    524.2 |
    526.8 |
    529.3 |
    531.8 |
    534.4 |
    536.9 |
    539.5 |
    542.0 |
  (4 below, 2 above range)

warm-container-native (n=40, range 493.7-509.6 ns)
    493.7 |##########################
    494.5 |##########################
    495.3 |########################################
    496.1 |####################
    496.9 |##########################
    497.7 |######
    498.5 |#############
    499.3 |
    500.1 |
    500.8 |
    501.6 |
    502.4 |####################
    503.2 |######
    504.0 |#############
    504.8 |#############
    505.6 |
    506.4 |#############
    507.2 |######
    508.0 |######
    508.8 |
  (2 below, 2 above range)

warm-container-plusone (n=40, range 1644.2-2003.1 ns)
   1644.2 |########################################
   1662.2 |##############
   1680.1 |#######
   1698.1 |##
   1716.0 |
   1733.9 |
   1751.9 |
   1769.8 |
   1787.8 |
   1805.7 |
   1823.7 |
   1841.6 |
   1859.5 |
   1877.5 |
   1895.4 |
   1913.4 |##
   1931.3 |
   1949.2 |
   1967.2 |
   1985.1 |#########
  (3 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.88 (measurement drift or warm-up artifact)
