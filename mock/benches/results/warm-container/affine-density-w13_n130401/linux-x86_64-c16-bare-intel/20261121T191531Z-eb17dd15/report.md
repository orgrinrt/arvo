# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 45% (significant)

warm-container-lanes-deferred is -821 ns (45%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 4.4x slower than the field

warm-container-minimum (4.55 us) is 4.4x the fastest (1.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-lanes-deferred, warm-container-kernel) are a dead heat (<1%)

warm-container-lanes-deferred (1.04 us) and warm-container-kernel (1.04 us) differ by 0.47%, inside the noise, even though the wider field spreads 339.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 4.4x the fastest

Fastest warm-container-lanes-deferred (1.04 us) to slowest warm-container-minimum (4.55 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-lanes-deferred vs stability leader warm-container-native (+6% speed for 1.4x steadier)

warm-container-lanes-deferred is fastest (1.04 us, CV 9.8%); warm-container-native gives up 5.7% median for 1.4x lower variance (CV 7.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-headroom is inconsistent: worst-20% is 4.4x its best-20%

warm-container-headroom's best 20% of batches run at 1.50 us but its worst 20% at 6.63 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1035.8 ns median (-43.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.39x (fastest 1035.8 ns, slowest 4550.0 ns)

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
| warm-container-headroom | 3244ns | 1898ns | 1553ns | 2581ns | 6922ns | base |
| warm-container-kernel | 1266ns | 1299ns | 1033ns | 1257ns | 1524ns | -60.98% |
| warm-container-lanes-deferred | 1242ns | 1252ns | 1032ns | 1258ns | 1402ns | -61.71% |
| warm-container-minimum | 3839ns | 4706ns | 1564ns | 3761ns | 6349ns | +18.37% |
| warm-container-native | 1368ns | 1319ns | 1248ns | 1349ns | 1547ns | -57.81% |
| warm-container-plusone | 3786ns | 4197ns | 1627ns | 3576ns | 6573ns | +16.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3122ns | 1501ns | 6629ns | base | 5.247 |
| warm-container-kernel | 1039ns | 857ns | 1242ns | -66.71% | 15.764 |
| warm-container-lanes-deferred | 1021ns | 856ns | 1133ns | -67.29% | 16.043 |
| warm-container-minimum | 3701ns | 1512ns | 6093ns | +18.54% | 4.427 |
| warm-container-native | 1126ns | 1036ns | 1241ns | -63.95% | 14.555 |
| warm-container-plusone | 3647ns | 1573ns | 6306ns | +16.82% | 4.492 |

## Performance model

- Peak throughput: **19.129 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.927 | 46.7% |
| warm-container-kernel | 15.743 | 82.3% |
| warm-container-lanes-deferred | 15.817 | 82.7% |
| warm-container-minimum | 3.601 | 18.8% |
| warm-container-native | 14.969 | 78.3% |
| warm-container-plusone | 4.039 | 21.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3244ns | 3244ns | base |
| warm-container-kernel | 1266ns | 1266ns | -60.98% |
| warm-container-lanes-deferred | 1242ns | 1242ns | -61.71% |
| warm-container-minimum | 3839ns | 3839ns | +18.37% |
| warm-container-native | 1368ns | 1368ns | -57.81% |
| warm-container-plusone | 3786ns | 3786ns | +16.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1835ns | base | --- | [1502, 3836] | --- | --- | --- | --- |
| warm-container-kernel | 1041ns | -674.6ns (-36.8%) | [-2695, -644]ns | [949, 1092] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1036ns | -820.6ns (-44.7%) | [-2794, -658]ns | [992, 1090] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4550ns | no significant difference | [-100, +778]ns | [1827, 5043] | no | 0.7948 | 0.6358 | 0 |
| warm-container-native | 1094ns | -741.2ns (-40.4%) | [-2741, -482]ns | [1092, 1136] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4056ns | no significant difference | [-12, +808]ns | [1823, 4694] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 1841ns | -34.2% | -42.2% | +147.2% | -40.5% | +34.4% |
| 1 | 2 | 4328ns | -77.2% | -76.3% | -47.4% | -74.4% | -57.8% |
| 1 | 3 | 1502ns | -44.1% | -43.5% | +132.4% | -24.7% | +191.1% |
| 1 | 4 | 1501ns | -25.5% | -27.3% | +233.5% | -28.8% | +201.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.340 | moderate- |
| warm-container-kernel | -0.183 | ok |
| warm-container-lanes-deferred | -0.424 | moderate- |
| warm-container-minimum | -0.223 | moderate- |
| warm-container-native | -0.400 | moderate- |
| warm-container-plusone | 0.172 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 1/4, lost 3/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 39.9ns | 3122.5ns | 1.3% |  |
| warm-container-kernel | 83.6ns | 1039.3ns | 8.0% | HIGH |
| warm-container-lanes-deferred | 81.0ns | 1021.3ns | 7.9% | HIGH |
| warm-container-minimum | 47.6ns | 3701.2ns | 1.3% |  |
| warm-container-native | 88.5ns | 1125.6ns | 7.9% | HIGH |
| warm-container-plusone | 46.9ns | 3647.5ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1501.0-6629.2 ns)
   1501.0 |########################################
   1757.4 |################
   2013.9 |
   2270.3 |
   2526.7 |
   2783.1 |##
   3039.5 |
   3295.9 |##
   3552.3 |
   3808.7 |
   4065.1 |##
   4321.5 |#####
   4577.9 |########
   4834.3 |
   5090.7 |##
   5347.1 |
   5603.6 |##
   5860.0 |
   6116.4 |
   6372.8 |#####
  (3 below, 4 above range)

warm-container-kernel (n=40, range 856.5-1241.9 ns)
    856.5 |
    875.8 |
    895.1 |########################################
    914.3 |
    933.6 |
    952.9 |
    972.1 |###############
    991.4 |###############
   1010.7 |
   1029.9 |
   1049.2 |
   1068.5 |
   1087.8 |########################################
   1107.0 |
   1126.3 |#####
   1145.6 |##########
   1164.8 |
   1184.1 |#####
   1203.4 |##############################
   1222.6 |
  (6 below, 2 above range)

warm-container-lanes-deferred (n=40, range 856.5-1133.2 ns)
    856.5 |
    870.3 |###
    884.2 |
    898.0 |#######
    911.8 |
    925.7 |
    939.5 |
    953.3 |
    967.2 |
    981.0 |#############################
    994.8 |###
   1008.7 |###
   1022.5 |###
   1036.3 |###
   1050.2 |
   1064.0 |
   1077.8 |########################################
   1091.7 |##############
   1105.5 |
   1119.3 |
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1512.0-6093.3 ns)
   1512.0 |#####
   1741.1 |########################################
   1970.2 |#####
   2199.2 |#####
   2428.3 |
   2657.3 |#####
   2886.4 |
   3115.5 |
   3344.5 |
   3573.6 |
   3802.7 |
   4031.7 |
   4260.8 |#####
   4489.9 |##########
   4718.9 |#####
   4948.0 |#########################
   5177.0 |##########
   5406.1 |###############
   5635.2 |#####
   5864.2 |###############
  (7 below, 3 above range)

warm-container-native (n=40, range 1036.1-1241.4 ns)
   1036.1 |##
   1046.3 |
   1056.6 |
   1066.9 |
   1077.1 |
   1087.4 |########################################
   1097.7 |##
   1107.9 |
   1118.2 |##
   1128.5 |
   1138.7 |
   1149.0 |##
   1159.2 |####
   1169.5 |
   1179.8 |##
   1190.0 |
   1200.3 |##
   1210.6 |##############
   1220.8 |
   1231.1 |
  (4 below, 2 above range)

warm-container-plusone (n=40, range 1572.8-6306.1 ns)
   1572.8 |
   1809.5 |########################################
   2046.1 |
   2282.8 |
   2519.5 |
   2756.1 |###
   2992.8 |###
   3229.4 |
   3466.1 |
   3702.8 |###
   3939.4 |
   4176.1 |#######
   4412.8 |##########
   4649.4 |##############
   4886.1 |#######
   5122.7 |###
   5359.4 |###
   5596.1 |###
   5832.7 |###
   6069.4 |
  (6 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=67.0% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=51.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-plusone**: CV=51.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.0x (possible bimodal distribution)
