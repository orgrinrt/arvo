# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 55% (significant)

warm-container-kernel is -1.31 us (55%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 3.2x slower than the field

warm-container-minimum (4.50 us) is 3.2x the fastest (1.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows alternating (throttle bounce) (autocorr -0.64)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest warm-container-lanes-deferred (1.42 us) to slowest warm-container-minimum (4.50 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone is inconsistent: worst-20% is 4.1x its best-20%

warm-container-plusone's best 20% of batches run at 1.46 us but its worst 20% at 6.01 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1422.0 ns median (-39.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 3.16x (fastest 1422.0 ns, slowest 4499.0 ns)

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
| warm-container-headroom | 4016ns | 2424ns | 1976ns | 3488ns | 7639ns | base |
| warm-container-kernel | 1663ns | 1645ns | 1235ns | 1639ns | 2164ns | -58.59% |
| warm-container-lanes-deferred | 1532ns | 1627ns | 874ns | 1598ns | 1990ns | -61.86% |
| warm-container-minimum | 3669ns | 4657ns | 1507ns | 3645ns | 5903ns | -8.64% |
| warm-container-native | 1951ns | 2013ns | 1336ns | 1990ns | 2448ns | -51.42% |
| warm-container-plusone | 3615ns | 3670ns | 1515ns | 3433ns | 6260ns | -9.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3890ns | 1925ns | 7348ns | base | 18.954 |
| warm-container-kernel | 1445ns | 1077ns | 1876ns | -62.85% | 51.016 |
| warm-container-lanes-deferred | 1331ns | 764ns | 1717ns | -65.79% | 55.407 |
| warm-container-minimum | 3533ns | 1455ns | 5649ns | -9.17% | 20.869 |
| warm-container-native | 1696ns | 1168ns | 2096ns | -56.40% | 43.469 |
| warm-container-plusone | 3481ns | 1463ns | 6008ns | -10.50% | 21.178 |

## Performance model

- Peak throughput: **96.550 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 31.231 | 32.3% |
| warm-container-kernel | 51.266 | 53.1% |
| warm-container-lanes-deferred | 51.848 | 53.7% |
| warm-container-minimum | 16.388 | 17.0% |
| warm-container-native | 41.877 | 43.4% |
| warm-container-plusone | 20.800 | 21.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4016ns | 4016ns | base |
| warm-container-kernel | 1663ns | 1663ns | -58.59% |
| warm-container-lanes-deferred | 1532ns | 1532ns | -61.86% |
| warm-container-minimum | 3669ns | 3669ns | -8.64% |
| warm-container-native | 1951ns | 1951ns | -51.42% |
| warm-container-plusone | 3615ns | 3615ns | -9.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2361ns | base | --- | [2020, 5608] | --- | --- | --- | --- |
| warm-container-kernel | 1438ns | -1305.2ns (-55.3%) | [-3932, -617]ns | [1335, 1509] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1422ns | -1170.5ns (-49.6%) | [-4386, -1034]ns | [1320, 1559] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4499ns | -478.2ns (-20.3%) | [-899, -465]ns | [1466, 4663] | YES | 0.0022 | 0.0022 | 0 |
| warm-container-native | 1761ns | -794.2ns (-33.6%) | [-3569, -601]ns | [1650, 1872] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3545ns | -470.9ns (-19.9%) | [-583, -376]ns | [1778, 4598] | YES | 0.0008 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2361ns | -53.4% | -43.3% | +76.5% | -23.0% | +5.4% |
| 1 | 2 | 2020ns | -29.9% | -18.0% | -27.6% | -12.8% | +2.2% |
| 1 | 3 | 4706ns | -69.4% | -83.4% | -1.3% | -57.9% | -60.4% |
| 1 | 4 | 2329ns | -23.9% | -34.0% | +90.8% | -48.9% | +80.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.453 | moderate- |
| warm-container-kernel | 0.036 | ok |
| warm-container-lanes-deferred | -0.643 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.372 | moderate- |
| warm-container-native | -0.321 | moderate- |
| warm-container-plusone | -0.194 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 41.9ns | 3889.8ns | 1.1% |  |
| warm-container-kernel | 77.0ns | 1445.2ns | 5.3% | HIGH |
| warm-container-lanes-deferred | 72.5ns | 1330.7ns | 5.4% | HIGH |
| warm-container-minimum | 47.1ns | 3532.9ns | 1.3% |  |
| warm-container-native | 89.7ns | 1696.1ns | 5.3% | HIGH |
| warm-container-plusone | 46.6ns | 3481.4ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1924.9-7348.3 ns)
   1924.9 |########################################
   2196.1 |#########################
   2467.3 |###
   2738.4 |
   3009.6 |
   3280.8 |
   3551.9 |
   3823.1 |#######
   4094.3 |
   4365.4 |
   4636.6 |
   4907.8 |
   5178.9 |
   5450.1 |##########
   5721.3 |#######
   5992.4 |##########
   6263.6 |###
   6534.8 |###
   6806.0 |###
   7077.1 |
  (4 below, 4 above range)

warm-container-kernel (n=40, range 1077.3-1876.2 ns)
   1077.3 |##########################
   1117.2 |######
   1157.2 |
   1197.1 |######
   1237.1 |
   1277.0 |#############
   1317.0 |####################
   1356.9 |#############
   1396.9 |####################
   1436.8 |########################################
   1476.8 |
   1516.7 |######
   1556.7 |####################
   1596.6 |######
   1636.6 |######
   1676.5 |
   1716.5 |####################
   1756.4 |
   1796.4 |######
   1836.3 |
  (4 below, 4 above range)

warm-container-lanes-deferred (n=40, range 763.6-1716.9 ns)
    763.6 |#########################
    811.3 |##########
    859.0 |
    906.6 |
    954.3 |
   1001.9 |
   1049.6 |
   1097.3 |
   1144.9 |
   1192.6 |
   1240.2 |
   1287.9 |#########################
   1335.6 |#####
   1383.2 |########################################
   1430.9 |
   1478.6 |#####
   1526.2 |########################################
   1573.9 |#####
   1621.6 |
   1669.2 |##########
  (3 below, 4 above range)

warm-container-minimum (n=40, range 1455.0-5649.1 ns)
   1455.0 |########################################
   1664.8 |####
   1874.5 |
   2084.2 |
   2293.9 |
   2503.6 |
   2713.3 |
   2923.0 |####
   3132.7 |
   3342.4 |
   3552.1 |
   3761.8 |####
   3971.5 |
   4181.2 |####
   4390.9 |####################
   4600.6 |################
   4810.3 |################
   5020.0 |####
   5229.7 |############
   5439.4 |####
  (5 below, 3 above range)

warm-container-native (n=40, range 1168.3-2096.4 ns)
   1168.3 |
   1214.7 |######################
   1261.1 |
   1307.5 |
   1353.9 |
   1400.4 |
   1446.8 |
   1493.2 |
   1539.6 |#############
   1586.0 |####
   1632.4 |
   1678.8 |####
   1725.2 |########################################
   1771.6 |####
   1818.0 |####
   1864.4 |####
   1910.8 |####
   1957.2 |###############################
   2003.6 |####
   2050.0 |####
  (5 below, 3 above range)

warm-container-plusone (n=40, range 1463.1-6007.8 ns)
   1463.1 |########################################
   1690.3 |##########################
   1917.6 |
   2144.8 |####
   2372.0 |
   2599.3 |####
   2826.5 |
   3053.7 |####
   3281.0 |
   3508.2 |
   3735.4 |####
   3962.7 |########
   4189.9 |########
   4417.1 |########
   4644.4 |
   4871.6 |####
   5098.8 |
   5326.1 |#################
   5553.3 |########
   5780.5 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=57.8% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=26.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=49.9% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-native**: CV=20.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=53.2% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.1x (possible bimodal distribution)
