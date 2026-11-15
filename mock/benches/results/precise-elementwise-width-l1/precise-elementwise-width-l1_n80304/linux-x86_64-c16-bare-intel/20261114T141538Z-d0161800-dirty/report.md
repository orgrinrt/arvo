# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (1.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 592 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 68% (significant)

warm-container-native is -1.26 us (68%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.1x slower than the field

warm-container-headroom (1.85 us) is 3.1x the fastest (592 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (592 ns) and warm-container-native (593 ns) differ by 0.04%, inside the noise, even though the wider field spreads 212.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (181% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 181% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest warm-container-minimum (592 ns) to slowest warm-container-headroom (1.85 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 1.8x its best-20%

warm-container-native's best 20% of batches run at 584 ns but its worst 20% at 1.06 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 592.5 ns median (-68.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.13x (fastest 592.5 ns, slowest 1853.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 2158ns | 2082ns | 1978ns | 2119ns | 2455ns | base |
| warm-container-kernel | 962ns | 912ns | 817ns | 929ns | 1205ns | -55.43% |
| warm-container-minimum | 870ns | 824ns | 783ns | 859ns | 991ns | -59.67% |
| warm-container-native | 991ns | 820ns | 805ns | 883ns | 1501ns | -54.08% |
| warm-container-plusone | 2215ns | 2084ns | 1858ns | 2114ns | 2876ns | +2.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 1910ns | 1762ns | 2146ns | base | 21.441 |
| warm-container-kernel | 683ns | 592ns | 854ns | -64.25% | 59.969 |
| warm-container-minimum | 614ns | 565ns | 672ns | -67.88% | 66.755 |
| warm-container-native | 705ns | 584ns | 1061ns | -63.08% | 58.068 |
| warm-container-plusone | 1959ns | 1655ns | 2516ns | +2.54% | 20.910 |

## Performance model

- Peak throughput: **72.542 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 22.105 | 30.5% |
| warm-container-kernel | 62.126 | 85.6% |
| warm-container-minimum | 69.131 | 95.3% |
| warm-container-native | 69.102 | 95.3% |
| warm-container-plusone | 22.109 | 30.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 2158ns | 2158ns | base |
| warm-container-kernel | 962ns | 962ns | -55.43% |
| warm-container-minimum | 870ns | 870ns | -59.67% |
| warm-container-native | 991ns | 991ns | -54.08% |
| warm-container-plusone | 2215ns | 2215ns | +2.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1853ns | base | --- | [1852, 1898] | --- | --- | --- | --- |
| warm-container-kernel | 659ns | -1258.8ns (-67.9%) | [-1261, -1194]ns | [594, 717] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 592ns | -1260.3ns (-68.0%) | [-1302, -1227]ns | [592, 655] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 593ns | -1260.5ns (-68.0%) | [-1307, -1260]ns | [592, 640] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 1853ns | no significant difference | [-51, +2]ns | [1852, 1942] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 1849ns | -54.2% | -64.6% | -46.6% | +4.2% |
| 1 | 2 | 1885ns | -65.0% | -65.2% | -68.6% | -1.7% |
| 1 | 3 | 1853ns | -68.0% | -68.2% | -68.0% | +2.8% |
| 1 | 4 | 1913ns | -69.1% | -69.0% | -69.0% | -12.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.483 | moderate- |
| warm-container-kernel | 0.119 | ok |
| warm-container-minimum | 0.235 | moderate+ |
| warm-container-native | -0.084 | ok |
| warm-container-plusone | -0.233 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 90.6ns | 1910.4ns | 4.7% |  |
| warm-container-kernel | 109.1ns | 683.0ns | 16.0% | HIGH |
| warm-container-minimum | 91.5ns | 613.6ns | 14.9% | HIGH |
| warm-container-native | 105.4ns | 705.4ns | 14.9% | HIGH |
| warm-container-plusone | 92.5ns | 1958.9ns | 4.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1762.3-2145.8 ns)
   1762.3 |
   1781.4 |
   1800.6 |
   1819.8 |
   1839.0 |########################################
   1858.1 |
   1877.3 |##
   1896.5 |##
   1915.7 |##
   1934.8 |##
   1954.0 |####
   1973.2 |
   1992.4 |
   2011.5 |
   2030.7 |
   2049.9 |########
   2069.1 |##
   2088.2 |
   2107.4 |##
   2126.6 |##
  (5 below, 2 above range)

warm-container-kernel (n=40, range 591.7-854.0 ns)
    591.7 |########################################
    604.8 |
    617.9 |
    631.0 |
    644.1 |####
    657.3 |############################
    670.4 |
    683.5 |
    696.6 |
    709.7 |########
    722.9 |
    736.0 |####################
    749.1 |
    762.2 |####
    775.3 |
    788.4 |
    801.6 |
    814.7 |####
    827.8 |
    840.9 |################
  (7 below, 2 above range)

warm-container-minimum (n=40, range 564.6-672.2 ns)
    564.6 |
    570.0 |
    575.4 |
    580.8 |
    586.1 |####################################
    591.5 |########################################
    596.9 |
    602.3 |
    607.7 |
    613.0 |
    618.4 |
    623.8 |
    629.2 |
    634.5 |
    639.9 |
    645.3 |
    650.7 |#####################
    656.0 |#########################
    661.4 |
    666.8 |###
  (4 below, 1 above range)

warm-container-native (n=40, range 584.2-1060.7 ns)
    584.2 |########################################
    608.0 |
    631.9 |###
    655.7 |#####
    679.5 |
    703.3 |
    727.2 |
    751.0 |
    774.8 |
    798.6 |
    822.5 |
    846.3 |###
    870.1 |
    893.9 |
    917.8 |
    941.6 |
    965.4 |######
    989.3 |
   1013.1 |
   1036.9 |#
  (2 below, 3 above range)

warm-container-plusone (n=40, range 1654.8-2515.9 ns)
   1654.8 |#####################
   1697.9 |
   1741.0 |######
   1784.0 |###
   1827.1 |########################################
   1870.1 |
   1913.2 |#########
   1956.2 |###
   1999.3 |###
   2042.3 |###############
   2085.4 |###
   2128.4 |
   2171.5 |
   2214.5 |
   2257.6 |###
   2300.6 |
   2343.7 |
   2386.7 |
   2429.8 |
   2472.8 |
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-kernel**: bridge=14.9% of algo (FFI overhead may distort results)
- **warm-container-minimum**: bridge=15.0% of algo (FFI overhead may distort results)
- **warm-container-native**: CV=27.1% (high variance, measurements may be unstable)
- **warm-container-native**: bridge=15.0% of algo (FFI overhead may distort results)
- **warm-container-plusone**: CV=20.9% (high variance, measurements may be unstable)
