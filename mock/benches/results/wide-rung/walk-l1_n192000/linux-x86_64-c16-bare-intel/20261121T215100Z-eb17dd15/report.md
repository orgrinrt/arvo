# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (3.08 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 2.39 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-wordround-alias dominates: 12% faster than the next best (wide-rung-ragged)

wide-rung-wordround-alias (2.39 us) leads wide-rung-ragged (2.67 us) by 12%, a clear separation rather than a photo finish. CV 26.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-wordround-alias beats baseline by 22% (significant)

wide-rung-wordround-alias is -669 ns (22%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-align16 is inconsistent: worst-20% is 3.9x its best-20%

wide-rung-align16's best 20% of batches run at 1.03 us but its worst 20% at 3.98 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 2390.7 ns median (-22.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.29x (fastest 2390.7 ns, slowest 3079.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112830321 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2869ns | 3292ns | 1104ns | 2978ns | 4306ns | base |
| wide-rung-ragged | 2817ns | 2893ns | 1982ns | 2897ns | 3411ns | -1.81% |
| wide-rung-ragged-overread | 2957ns | 3176ns | 1382ns | 3224ns | 3730ns | +3.07% |
| wide-rung-wordround | 2795ns | 3209ns | 997ns | 3059ns | 3800ns | -2.56% |
| wide-rung-wordround-alias | 2579ns | 2637ns | 1539ns | 2647ns | 3413ns | -10.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2671ns | 1031ns | 3982ns | base | 0.767 |
| wide-rung-ragged | 2587ns | 1828ns | 3102ns | -3.16% | 0.792 |
| wide-rung-ragged-overread | 2714ns | 1274ns | 3394ns | +1.60% | 0.755 |
| wide-rung-wordround | 2566ns | 919ns | 3466ns | -3.94% | 0.798 |
| wide-rung-wordround-alias | 2368ns | 1420ns | 3117ns | -11.34% | 0.865 |

## Performance model

- Peak throughput: **2.228 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.665 | 29.9% |
| wide-rung-ragged | 0.767 | 34.4% |
| wide-rung-ragged-overread | 0.698 | 31.3% |
| wide-rung-wordround | 0.691 | 31.0% |
| wide-rung-wordround-alias | 0.857 | 38.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2869ns | 2869ns | base |
| wide-rung-ragged | 2817ns | 2817ns | -1.81% |
| wide-rung-ragged-overread | 2957ns | 2957ns | +3.07% |
| wide-rung-wordround | 2795ns | 2795ns | -2.56% |
| wide-rung-wordround-alias | 2579ns | 2579ns | -10.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3079ns | base | --- | [2186, 3506] | --- | --- | --- | --- |
| wide-rung-ragged | 2670ns | -202.3ns (-6.6%) | [-653, -94]ns | [2514, 2756] | YES | 0.0332 | 0.0166 | 0 |
| wide-rung-ragged-overread | 2935ns | no significant difference | [-524, +182]ns | [2879, 3144] | no | 0.2682 | 0.2682 | 0 |
| wide-rung-wordround | 2963ns | -205.7ns (-6.7%) | [-388, -98]ns | [2784, 3149] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-wordround-alias | 2391ns | -668.5ns (-21.7%) | [-838, -258]ns | [2298, 2665] | YES | 0.0089 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1300ns | +92.4% | +125.7% | +126.5% | +69.0% |
| 1 | 2 | 3522ns | -26.6% | -16.5% | -14.4% | -32.9% |
| 1 | 3 | 3505ns | -17.5% | -18.9% | -12.2% | -23.2% |
| 1 | 4 | 2940ns | -4.8% | +6.9% | +0.3% | -25.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.152 | ok |
| wide-rung-ragged | 0.205 | moderate+ |
| wide-rung-ragged-overread | -0.380 | moderate- |
| wide-rung-wordround | -0.284 | moderate- |
| wide-rung-wordround-alias | -0.329 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 54.7ns | 2671.1ns | 2.0% |  |
| wide-rung-ragged | 61.7ns | 2586.6ns | 2.4% |  |
| wide-rung-ragged-overread | 66.7ns | 2713.8ns | 2.5% |  |
| wide-rung-wordround | 62.1ns | 2565.9ns | 2.4% |  |
| wide-rung-wordround-alias | 56.5ns | 2368.3ns | 2.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1031.1-3982.3 ns)
   1031.1 |########################################
   1178.7 |
   1326.2 |
   1473.8 |
   1621.3 |###########
   1768.9 |
   1916.5 |
   2064.0 |
   2211.6 |
   2359.2 |
   2506.7 |
   2654.3 |#####
   2801.8 |#################
   2949.4 |###########
   3097.0 |#################
   3244.5 |#####
   3392.1 |############################
   3539.6 |###########
   3687.2 |#################
   3834.8 |#################
  (5 below, 3 above range)

wide-rung-ragged (n=40, range 1828.3-3101.8 ns)
   1828.3 |####
   1891.9 |
   1955.6 |####
   2019.3 |
   2083.0 |
   2146.6 |
   2210.3 |
   2274.0 |########
   2337.7 |
   2401.4 |####
   2465.0 |###############################
   2528.7 |####
   2592.4 |
   2656.1 |########################################
   2719.7 |####
   2783.4 |####
   2847.1 |##########################
   2910.8 |########
   2974.4 |
   3038.1 |
  (3 below, 5 above range)

wide-rung-ragged-overread (n=40, range 1274.3-3393.7 ns)
   1274.3 |
   1380.3 |
   1486.3 |
   1592.3 |
   1698.2 |
   1804.2 |#####
   1910.2 |
   2016.1 |
   2122.1 |
   2228.1 |
   2334.0 |
   2440.0 |
   2546.0 |
   2651.9 |#########################
   2757.9 |#####
   2863.9 |########################################
   2969.8 |#####
   3075.8 |########################################
   3181.8 |####################
   3287.8 |#####
  (6 below, 5 above range)

wide-rung-wordround (n=40, range 919.3-3466.1 ns)
    919.3 |########################################
   1046.6 |
   1174.0 |
   1301.3 |
   1428.7 |#####
   1556.0 |
   1683.3 |
   1810.7 |
   1938.0 |
   2065.4 |
   2192.7 |#####
   2320.0 |
   2447.4 |#####
   2574.7 |
   2702.1 |#################
   2829.4 |######################
   2956.7 |##################################
   3084.1 |##################################
   3211.4 |#####
   3338.7 |############################
  (2 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1420.4-3117.5 ns)
   1420.4 |
   1505.2 |
   1590.1 |
   1674.9 |
   1759.8 |
   1844.6 |
   1929.5 |
   2014.4 |
   2099.2 |##########################
   2184.1 |##########################
   2268.9 |########################################
   2353.8 |#############
   2438.6 |####################
   2523.5 |######
   2608.4 |#################################
   2693.2 |####################
   2778.1 |
   2862.9 |#############
   2947.8 |######
   3032.6 |
  (5 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=43.8% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=29.4% (high variance, measurements may be unstable)
- **wide-rung-wordround**: CV=37.2% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=26.7% (high variance, measurements may be unstable)
