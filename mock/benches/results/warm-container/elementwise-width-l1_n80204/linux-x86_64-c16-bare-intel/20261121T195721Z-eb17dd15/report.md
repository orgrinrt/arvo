# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (2.11 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 933 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 52% (significant)

warm-container-minimum is -1.10 us (52%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.3x slower than the field

warm-container-headroom (2.11 us) is 2.3x the fastest (933 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum is fastest but the noisiest (CV 42.1%)

warm-container-minimum wins on median (933 ns) yet has the highest variance (CV 42.1%), while warm-container-headroom is the steadiest (CV 6.5%, 2.11 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.60)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-kernel, warm-container-native} vs {warm-container-plusone, warm-container-headroom} (99% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom} with a 99% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 2.2x its best-20%

warm-container-minimum's best 20% of batches run at 847 ns but its worst 20% at 1.85 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 932.5 ns median (-55.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.27x (fastest 932.5 ns, slowest 2114.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112697097 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 2288ns | 2336ns | 2090ns | 2271ns | 2537ns | base |
| warm-container-kernel | 1253ns | 1201ns | 1022ns | 1210ns | 1613ns | -45.23% |
| warm-container-minimum | 1418ns | 1159ns | 1051ns | 1238ns | 2325ns | -38.01% |
| warm-container-native | 1324ns | 1313ns | 1088ns | 1317ns | 1582ns | -42.12% |
| warm-container-plusone | 2281ns | 2339ns | 1789ns | 2274ns | 2791ns | -0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 2057ns | 1889ns | 2246ns | base | 19.913 |
| warm-container-kernel | 1002ns | 823ns | 1281ns | -51.28% | 40.869 |
| warm-container-minimum | 1132ns | 847ns | 1851ns | -44.97% | 36.183 |
| warm-container-native | 1061ns | 880ns | 1250ns | -48.42% | 38.603 |
| warm-container-plusone | 2050ns | 1617ns | 2486ns | -0.32% | 19.978 |

## Performance model

- Peak throughput: **49.744 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 19.370 | 38.9% |
| warm-container-kernel | 42.859 | 86.2% |
| warm-container-minimum | 43.923 | 88.3% |
| warm-container-native | 38.556 | 77.5% |
| warm-container-plusone | 19.375 | 38.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 2288ns | 2288ns | base |
| warm-container-kernel | 1253ns | 1253ns | -45.23% |
| warm-container-minimum | 1418ns | 1418ns | -38.01% |
| warm-container-native | 1324ns | 1324ns | -42.12% |
| warm-container-plusone | 2281ns | 2281ns | -0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2115ns | base | --- | [1977, 2115] | --- | --- | --- | --- |
| warm-container-kernel | 956ns | -1050.3ns (-49.7%) | [-1108, -966]ns | [864, 1057] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 933ns | -1096.2ns (-51.8%) | [-1182, -1058]ns | [906, 956] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1062ns | -1028.8ns (-48.6%) | [-1062, -924]ns | [986, 1117] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2114ns | no significant difference | [-93, +123]ns | [1935, 2132] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2110ns | -50.6% | -59.8% | -54.3% | -3.8% |
| 1 | 2 | 2115ns | -59.8% | -58.8% | -58.4% | +11.1% |
| 1 | 3 | 1992ns | -56.7% | -6.2% | -40.8% | -18.3% |
| 1 | 4 | 2115ns | -42.3% | -55.9% | -43.4% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.449 | moderate- |
| warm-container-kernel | -0.195 | ok |
| warm-container-minimum | -0.361 | moderate- |
| warm-container-native | 0.160 | ok |
| warm-container-plusone | -0.602 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 84.9ns | 2057.0ns | 4.1% |  |
| warm-container-kernel | 92.4ns | 1002.2ns | 9.2% | HIGH |
| warm-container-minimum | 104.4ns | 1132.0ns | 9.2% | HIGH |
| warm-container-native | 96.0ns | 1061.1ns | 9.0% | HIGH |
| warm-container-plusone | 83.9ns | 2050.3ns | 4.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1889.1-2245.5 ns)
   1889.1 |
   1906.9 |##########################
   1924.7 |
   1942.5 |
   1960.4 |#####
   1978.2 |#####
   1996.0 |#####
   2013.8 |
   2031.7 |
   2049.5 |
   2067.3 |
   2085.1 |
   2102.9 |########################################
   2120.8 |
   2138.6 |
   2156.4 |########
   2174.2 |
   2192.1 |
   2209.9 |
   2227.7 |
  (2 below, 4 above range)

warm-container-kernel (n=40, range 823.4-1281.0 ns)
    823.4 |
    846.3 |########################################
    869.2 |##############
    892.0 |
    914.9 |###
    937.8 |###
    960.7 |#######
    983.6 |###
   1006.4 |
   1029.3 |##########
   1052.2 |###
   1075.1 |#######
   1097.9 |
   1120.8 |###
   1143.7 |
   1166.6 |##################
   1189.5 |
   1212.3 |
   1235.2 |
   1258.1 |###
  (3 below, 4 above range)

warm-container-minimum (n=40, range 847.0-1850.8 ns)
    847.0 |#################################
    897.1 |########################################
    947.3 |###
    997.5 |###
   1047.7 |###
   1097.9 |###
   1148.1 |
   1198.3 |
   1248.5 |
   1298.7 |
   1348.9 |
   1399.1 |
   1449.3 |
   1499.5 |
   1549.7 |######
   1599.8 |###
   1650.0 |
   1700.2 |
   1750.4 |###
   1800.6 |
  (4 below, 6 above range)

warm-container-native (n=40, range 879.7-1250.0 ns)
    879.7 |#####
    898.3 |
    916.8 |######################
    935.3 |#####
    953.8 |#####
    972.3 |#####
    990.8 |#####
   1009.3 |
   1027.9 |###########
   1046.4 |######################
   1064.9 |#################
   1083.4 |#####
   1101.9 |#####
   1120.4 |###########
   1139.0 |
   1157.5 |#####
   1176.0 |
   1194.5 |########################################
   1213.0 |
   1231.5 |#####
  (6 below, 3 above range)

warm-container-plusone (n=40, range 1617.2-2485.9 ns)
   1617.2 |##########
   1660.6 |###
   1704.0 |
   1747.5 |#############
   1790.9 |
   1834.3 |
   1877.8 |
   1921.2 |#############
   1964.6 |
   2008.1 |
   2051.5 |###
   2094.9 |########################################
   2138.4 |#############
   2181.8 |
   2225.2 |
   2268.7 |
   2312.1 |##########
   2355.5 |###
   2399.0 |###
   2442.4 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-minimum**: CV=34.7% (high variance, measurements may be unstable)
