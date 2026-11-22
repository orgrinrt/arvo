# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (1.79 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 592 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 66% (significant)

warm-container-minimum is -1.17 us (66%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.0x slower than the field

warm-container-headroom (1.79 us) is 3.0x the fastest (592 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-minimum) are a dead heat (<1%)

warm-container-native (592 ns) and warm-container-minimum (592 ns) differ by 0.03%, inside the noise, even though the wider field spreads 201.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (164% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 164% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.0x the fastest

Fastest warm-container-native (592 ns) to slowest warm-container-headroom (1.79 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-native vs stability leader warm-container-kernel (+0% speed for 1.3x steadier)

warm-container-native is fastest (592 ns, CV 11.2%); warm-container-kernel gives up 0.2% median for 1.3x lower variance (CV 8.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-minimum is inconsistent: worst-20% is 2.3x its best-20%

warm-container-minimum's best 20% of batches run at 525 ns but its worst 20% at 1.23 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 592.3 ns median (-66.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 3.01x (fastest 592.3 ns, slowest 1785.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112840787 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 1971ns | 2036ns | 1657ns | 1957ns | 2329ns | base |
| warm-container-kernel | 832ns | 821ns | 733ns | 826ns | 949ns | -57.79% |
| warm-container-minimum | 1007ns | 820ns | 726ns | 868ns | 1703ns | -48.93% |
| warm-container-native | 858ns | 836ns | 723ns | 854ns | 1009ns | -56.45% |
| warm-container-plusone | 1852ns | 1805ns | 1599ns | 1832ns | 2167ns | -6.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1741ns | 1477ns | 2043ns | base | 23.531 |
| warm-container-kernel | 594ns | 530ns | 667ns | -65.87% | 68.954 |
| warm-container-minimum | 721ns | 525ns | 1228ns | -58.57% | 56.800 |
| warm-container-native | 606ns | 518ns | 701ns | -65.21% | 67.637 |
| warm-container-plusone | 1640ns | 1424ns | 1900ns | -5.77% | 24.973 |

## Performance model

- Peak throughput: **79.129 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 22.945 | 29.0% |
| warm-container-kernel | 69.038 | 87.2% |
| warm-container-minimum | 69.137 | 87.4% |
| warm-container-native | 69.154 | 87.4% |
| warm-container-plusone | 26.193 | 33.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 1971ns | 1971ns | base |
| warm-container-kernel | 832ns | 832ns | -57.79% |
| warm-container-minimum | 1007ns | 1007ns | -48.93% |
| warm-container-native | 858ns | 858ns | -56.45% |
| warm-container-plusone | 1852ns | 1852ns | -6.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1785ns | base | --- | [1564, 1853] | --- | --- | --- | --- |
| warm-container-kernel | 593ns | -1168.2ns (-65.4%) | [-1308, -951]ns | [593, 594] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 592ns | -1171.1ns (-65.6%) | [-1260, -1007]ns | [539, 626] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 592ns | -1147.8ns (-64.3%) | [-1195, -1051]ns | [592, 614] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1564ns | -112.8ns (-6.3%) | [-204, -28]ns | [1543, 1715] | YES | 0.0095 | 0.0095 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 1569ns | -62.2% | -65.7% | -65.8% | -1.7% |
| 1 | 2 | 1541ns | -58.8% | -23.1% | -61.5% | +9.2% |
| 1 | 3 | 1978ns | -70.0% | -72.8% | -70.1% | -6.4% |
| 1 | 4 | 1853ns | -70.9% | -68.0% | -63.2% | -22.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.099 | ok |
| warm-container-kernel | 0.025 | ok |
| warm-container-minimum | -0.483 | moderate- |
| warm-container-native | -0.013 | ok |
| warm-container-plusone | -0.365 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 83.6ns | 1740.7ns | 4.8% |  |
| warm-container-kernel | 88.0ns | 594.0ns | 14.8% | HIGH |
| warm-container-minimum | 106.9ns | 721.1ns | 14.8% | HIGH |
| warm-container-native | 93.5ns | 605.6ns | 15.4% | HIGH |
| warm-container-plusone | 78.1ns | 1640.2ns | 4.8% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1476.6-2042.7 ns)
   1476.6 |
   1504.9 |
   1533.2 |################################
   1561.5 |###
   1589.8 |###
   1618.1 |
   1646.4 |
   1674.7 |##########
   1703.0 |
   1731.3 |###
   1759.6 |
   1787.9 |
   1816.2 |###
   1844.5 |########################################
   1872.8 |#######
   1901.1 |
   1929.4 |
   1957.7 |
   1986.1 |
   2014.4 |
  (5 below, 6 above range)

warm-container-kernel (n=40, range 530.1-666.5 ns)
    530.1 |
    536.9 |############
    543.7 |
    550.6 |
    557.4 |
    564.2 |
    571.0 |
    577.8 |
    584.7 |
    591.5 |########################################
    598.3 |
    605.1 |###
    612.0 |
    618.8 |
    625.6 |
    632.4 |
    639.3 |
    646.1 |
    652.9 |###
    659.7 |###
  (2 below, 2 above range)

warm-container-minimum (n=40, range 524.8-1228.3 ns)
    524.8 |########################################
    560.0 |##############################
    595.1 |
    630.3 |############
    665.5 |
    700.7 |
    735.8 |
    771.0 |
    806.2 |
    841.4 |
    876.5 |
    911.7 |
    946.9 |
    982.0 |#########
   1017.2 |
   1052.4 |###
   1087.6 |
   1122.7 |
   1157.9 |############
   1193.1 |
  (3 below, 2 above range)

warm-container-native (n=40, range 517.6-701.2 ns)
    517.6 |
    526.8 |
    536.0 |###############
    545.2 |
    554.3 |
    563.5 |
    572.7 |
    581.9 |
    591.1 |########################################
    600.2 |
    609.4 |
    618.6 |
    627.8 |##
    636.9 |
    646.1 |
    655.3 |####################
    664.5 |
    673.6 |
    682.8 |
    692.0 |
  (4 below, 5 above range)

warm-container-plusone (n=40, range 1424.5-1899.8 ns)
   1424.5 |#############
   1448.2 |######
   1472.0 |######
   1495.8 |
   1519.5 |#################################
   1543.3 |##########################
   1567.1 |#############
   1590.8 |
   1614.6 |
   1638.4 |######
   1662.1 |####################
   1685.9 |
   1709.7 |######
   1733.4 |######
   1757.2 |
   1781.0 |
   1804.7 |
   1828.5 |##########################
   1852.3 |########################################
   1876.0 |
  (7 below, 2 above range)

```

## Diagnostics

- **warm-container-kernel**: bridge=15.0% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=39.7% (high variance, measurements may be unstable)
- **warm-container-minimum**: bridge=15.0% of algo (FFI overhead may distort results)
- **warm-container-native**: bridge=15.1% of algo (FFI overhead may distort results)
