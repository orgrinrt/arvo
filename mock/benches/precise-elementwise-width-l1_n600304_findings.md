# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (10.14 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-plusone at 1.65 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 84% (significant)

warm-container-native is -8.49 us (84%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 6.2x slower than the field

warm-container-headroom (10.14 us) is 6.2x the fastest (1.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.53)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-plusone, warm-container-native, warm-container-minimum, warm-container-kernel} vs {warm-container-headroom} (341% apart)

The field splits into a fast tier {warm-container-plusone, warm-container-native, warm-container-minimum, warm-container-kernel} and a slow tier {warm-container-headroom} with a 341% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.2x the fastest

Fastest warm-container-plusone (1.65 us) to slowest warm-container-headroom (10.14 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-plusone** at 1648.5 ns median (-83.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 6.15x (fastest 1648.5 ns, slowest 10139.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 10263ns | 10236ns | 10117ns | 10245ns | 10465ns | base |
| warm-container-kernel | 2393ns | 2365ns | 2357ns | 2377ns | 2478ns | -76.68% |
| warm-container-minimum | 1739ns | 1735ns | 1708ns | 1732ns | 1788ns | -83.06% |
| warm-container-native | 1765ns | 1733ns | 1708ns | 1741ns | 1892ns | -82.81% |
| warm-container-plusone | 1720ns | 1712ns | 1707ns | 1713ns | 1752ns | -83.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 10166ns | 10020ns | 10361ns | base | 4.029 |
| warm-container-kernel | 2325ns | 2295ns | 2404ns | -77.13% | 17.620 |
| warm-container-minimum | 1676ns | 1646ns | 1723ns | -83.52% | 24.445 |
| warm-container-native | 1699ns | 1646ns | 1824ns | -83.29% | 24.106 |
| warm-container-plusone | 1657ns | 1645ns | 1687ns | -83.70% | 24.726 |

## Performance model

- Peak throughput: **24.896 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.040 | 16.2% |
| warm-container-kernel | 17.818 | 71.6% |
| warm-container-minimum | 24.487 | 98.4% |
| warm-container-native | 24.561 | 98.7% |
| warm-container-plusone | 24.846 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 10263ns | 10263ns | base |
| warm-container-kernel | 2393ns | 2393ns | -76.68% |
| warm-container-minimum | 1739ns | 1739ns | -83.06% |
| warm-container-native | 1765ns | 1765ns | -82.81% |
| warm-container-plusone | 1720ns | 1720ns | -83.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 10139ns | base | --- | [10098, 10197] | --- | --- | --- | --- |
| warm-container-kernel | 2299ns | -7823.4ns (-77.2%) | [-7885, -7768]ns | [2298, 2312] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 1673ns | -8470.4ns (-83.5%) | [-8545, -8412]ns | [1659, 1674] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1668ns | -8447.3ns (-83.3%) | [-8525, -8412]ns | [1651, 1692] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1649ns | -8492.0ns (-83.8%) | [-8551, -8437]ns | [1648, 1652] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 10289ns | -77.7% | -84.0% | -83.7% | -84.0% |
| 2 | 10120ns | -77.3% | -83.7% | -83.5% | -83.7% |
| 3 | 10210ns | -77.5% | -83.9% | -83.9% | -83.9% |
| 4 | 10278ns | -77.7% | -84.0% | -83.9% | -83.8% |
| 5 | 10184ns | -77.4% | -83.8% | -83.7% | -83.8% |
| 6 | 10185ns | -77.5% | -83.8% | -83.8% | -83.8% |
| 7 | 10257ns | -77.6% | -83.9% | -83.2% | -84.0% |
| 8 | 10217ns | -77.5% | -83.9% | -83.4% | -83.9% |
| 9 | 10375ns | -77.9% | -84.1% | -83.8% | -84.1% |
| 10 | 10548ns | -76.6% | -84.4% | -84.4% | -84.4% |
| 11 | 10247ns | -76.1% | -83.7% | -83.5% | -83.8% |
| 12 | 10264ns | -76.9% | -83.7% | -84.0% | -83.5% |
| 13 | 10090ns | -77.1% | -83.2% | -81.8% | -83.6% |
| 14 | 10114ns | -76.3% | -83.5% | -83.7% | -83.7% |
| 15 | 10016ns | -76.7% | -82.1% | -83.6% | -83.5% |
| 16 | 10042ns | -76.3% | -83.3% | -83.6% | -83.5% |
| 17 | 10278ns | -76.3% | -83.4% | -84.0% | -83.9% |
| 18 | 10086ns | -76.6% | -83.4% | -83.6% | -83.7% |
| 19 | 10044ns | -77.1% | -83.3% | -83.2% | -83.6% |
| 20 | 10138ns | -77.3% | -83.5% | -83.4% | -83.7% |
| 21 | 10111ns | -77.3% | -83.2% | -78.8% | -82.9% |
| 22 | 10022ns | -77.1% | -83.3% | -82.4% | -83.0% |
| 23 | 10140ns | -77.4% | -83.5% | -82.9% | -83.8% |
| 24 | 10181ns | -77.4% | -83.6% | -83.0% | -83.8% |
| 25 | 10225ns | -77.5% | -83.6% | -82.4% | -83.9% |
| 26 | 10353ns | -77.8% | -83.8% | -83.3% | -84.1% |
| 27 | 10395ns | -77.9% | -83.8% | -83.4% | -84.2% |
| 28 | 10106ns | -77.3% | -83.3% | -82.8% | -83.7% |
| 29 | 10028ns | -77.1% | -83.3% | -81.7% | -83.5% |
| 30 | 10008ns | -77.1% | -83.1% | -83.1% | -83.4% |
| 31 | 10035ns | -76.7% | -83.6% | -83.6% | -83.0% |
| 32 | 10022ns | -76.7% | -83.6% | -83.6% | -83.4% |
| 33 | 10012ns | -76.7% | -83.3% | -83.4% | -83.4% |
| 34 | 10018ns | -77.1% | -82.9% | -83.6% | -83.6% |
| 35 | 10374ns | -77.9% | -83.5% | -84.1% | -84.1% |
| 36 | 10155ns | -77.4% | -83.1% | -83.0% | -83.8% |
| 37 | 10071ns | -77.1% | -82.8% | -83.6% | -83.6% |
| 38 | 10074ns | -77.2% | -83.0% | -83.6% | -83.6% |
| 39 | 10133ns | -76.5% | -83.7% | -83.7% | -83.8% |
| 40 | 10185ns | -76.8% | -83.8% | -83.7% | -83.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.463 | moderate+ |
| warm-container-kernel | 0.530 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.397 | moderate+ |
| warm-container-native | 0.185 | ok |
| warm-container-plusone | 0.330 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.7ns | 10165.8ns | 0.0% |  |
| warm-container-kernel | 2.9ns | 2324.6ns | 0.1% |  |
| warm-container-minimum | 2.6ns | 1675.6ns | 0.2% |  |
| warm-container-native | 2.7ns | 1699.2ns | 0.2% |  |
| warm-container-plusone | 2.6ns | 1656.5ns | 0.2% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 10020.2-10361.3 ns)
  10020.2 |########################################
  10037.3 |####################
  10054.3 |##########
  10071.4 |####################
  10088.4 |##########
  10105.5 |########################################
  10122.5 |####################
  10139.6 |####################
  10156.6 |
  10173.7 |########################################
  10190.8 |
  10207.8 |##############################
  10224.9 |
  10241.9 |####################
  10259.0 |##########
  10276.0 |##############################
  10293.1 |
  10310.1 |
  10327.2 |
  10344.2 |##########
  (4 below, 4 above range)

warm-container-kernel (n=40, range 2295.3-2403.7 ns)
   2295.3 |########################################
   2300.8 |
   2306.2 |#
   2311.6 |#
   2317.0 |
   2322.4 |
   2327.9 |#
   2333.3 |###
   2338.7 |#
   2344.1 |
   2349.5 |
   2355.0 |
   2360.4 |###
   2365.8 |#
   2371.2 |#
   2376.6 |
   2382.1 |#
   2387.5 |
   2392.9 |#
   2398.3 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 1646.0-1723.1 ns)
   1646.0 |########################################
   1649.9 |##########
   1653.7 |
   1657.6 |
   1661.4 |
   1665.3 |#####
   1669.1 |##############################
   1673.0 |##############################
   1676.9 |#####
   1680.7 |#####
   1684.6 |#####
   1688.4 |#####
   1692.3 |#####
   1696.1 |
   1700.0 |#####
   1703.8 |#####
   1707.7 |##########
   1711.6 |
   1715.4 |##########
   1719.3 |
  (4 below, 2 above range)

warm-container-native (n=40, range 1645.8-1823.6 ns)
   1645.8 |########################################
   1654.7 |##########
   1663.6 |######
   1672.5 |###
   1681.4 |#############
   1690.3 |######
   1699.2 |
   1708.1 |
   1717.0 |######
   1725.8 |######
   1734.7 |##########
   1743.6 |
   1752.5 |###
   1761.4 |
   1770.3 |
   1779.2 |
   1788.1 |
   1797.0 |###
   1805.9 |
   1814.7 |
  (4 below, 3 above range)

warm-container-plusone (n=40, range 1645.3-1686.9 ns)
   1645.3 |#####################
   1647.3 |########################################
   1649.4 |##########
   1651.5 |##########
   1653.6 |##########
   1655.7 |
   1657.7 |###
   1659.8 |###
   1661.9 |#######
   1664.0 |
   1666.1 |###
   1668.1 |
   1670.2 |
   1672.3 |
   1674.4 |
   1676.5 |
   1678.5 |
   1680.6 |
   1682.7 |
   1684.8 |
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.53 (measurement drift or warm-up artifact)
