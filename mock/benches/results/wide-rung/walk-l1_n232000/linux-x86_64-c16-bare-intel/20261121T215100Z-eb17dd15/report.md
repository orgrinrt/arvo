# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread dominates: 18% faster than the next best (wide-rung-wordround-alias)

wide-rung-ragged-overread (1.57 us) leads wide-rung-wordround-alias (1.86 us) by 18%, a clear separation rather than a photo finish. CV 118.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged-overread is fastest but the noisiest (CV 118.3%)

wide-rung-ragged-overread wins on median (1.57 us) yet has the highest variance (CV 118.3%), while wide-rung-ragged is the steadiest (CV 89.0%, 2.21 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (641 ns) is smaller than the fastest variant's own run-to-run std-dev (1.86 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wide-rung-ragged is inconsistent: worst-20% is 4.4x its best-20%

wide-rung-ragged's best 20% of batches run at 1.59 us but its worst 20% at 6.91 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 1572.7 ns median (-26.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.41x (fastest 1572.7 ns, slowest 2213.8 ns)

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
| wide-rung-align16 | 3627ns | 2244ns | 1716ns | 3256ns | 6650ns | base |
| wide-rung-ragged | 3319ns | 2296ns | 1654ns | 2567ns | 7239ns | -8.49% |
| wide-rung-ragged-overread | 2849ns | 1639ns | 1515ns | 2090ns | 6460ns | -21.44% |
| wide-rung-wordround | 3191ns | 1948ns | 1600ns | 2783ns | 6005ns | -12.02% |
| wide-rung-wordround-alias | 3462ns | 1942ns | 1776ns | 2936ns | 6725ns | -4.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3462ns | 1644ns | 6310ns | base | 0.592 |
| wide-rung-ragged | 3185ns | 1588ns | 6914ns | -8.01% | 0.643 |
| wide-rung-ragged-overread | 2718ns | 1449ns | 6130ns | -21.50% | 0.754 |
| wide-rung-wordround | 3047ns | 1533ns | 5703ns | -12.00% | 0.672 |
| wide-rung-wordround-alias | 3304ns | 1698ns | 6397ns | -4.58% | 0.620 |

## Performance model

- Peak throughput: **1.414 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.952 | 67.3% |
| wide-rung-ragged | 0.925 | 65.4% |
| wide-rung-ragged-overread | 1.302 | 92.1% |
| wide-rung-wordround | 1.097 | 77.6% |
| wide-rung-wordround-alias | 1.100 | 77.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3627ns | 3627ns | base |
| wide-rung-ragged | 3319ns | 3319ns | -8.49% |
| wide-rung-ragged-overread | 2849ns | 2849ns | -21.44% |
| wide-rung-wordround | 3191ns | 3191ns | -12.02% |
| wide-rung-wordround-alias | 3462ns | 3462ns | -4.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2151ns | base | --- | [1855, 4779] | --- | --- | --- | --- |
| wide-rung-ragged | 2214ns | no significant difference | [-272, +360]ns | [2213, 2228] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-ragged-overread | 1573ns | -292.4ns (-13.6%) | [-372, -281]ns | [1562, 1913] | YES | 0.0007 | 0.0002 | 0 |
| wide-rung-wordround | 1867ns | no significant difference | [-336, +1]ns | [1854, 4323] | no | 0.3576 | 0.2682 | 0 |
| wide-rung-wordround-alias | 1862ns | -118.8ns (-5.5%) | [-340, -9]ns | [1848, 4163] | YES | 0.0332 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1862ns | -14.8% | -16.1% | -0.4% | -0.4% |
| 1 | 2 | 4786ns | -53.6% | -60.0% | -61.1% | -11.1% |
| 1 | 3 | 2070ns | +7.3% | -30.7% | +115.8% | -10.6% |
| 1 | 4 | 1861ns | +19.3% | -15.7% | -15.4% | +12.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.400 | moderate- |
| wide-rung-ragged | -0.082 | ok |
| wide-rung-ragged-overread | -0.488 | moderate- |
| wide-rung-wordround | -0.468 | moderate- |
| wide-rung-wordround-alias | -0.495 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 45.2ns | 3462.2ns | 1.3% |  |
| wide-rung-ragged | 35.7ns | 3185.1ns | 1.1% |  |
| wide-rung-ragged-overread | 35.2ns | 2717.8ns | 1.3% |  |
| wide-rung-wordround | 40.5ns | 3046.7ns | 1.3% |  |
| wide-rung-wordround-alias | 44.0ns | 3303.5ns | 1.3% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1644.2-6309.8 ns)
   1644.2 |########################################
   1877.5 |
   2110.8 |
   2344.1 |##
   2577.3 |##
   2810.6 |
   3043.9 |
   3277.2 |
   3510.5 |
   3743.7 |
   3977.0 |
   4210.3 |##
   4443.6 |##
   4676.8 |################
   4910.1 |##
   5143.4 |##########
   5376.7 |
   5610.0 |
   5843.2 |
   6076.5 |
  (5 below, 5 above range)

wide-rung-ragged (n=40, range 1588.2-6913.6 ns)
   1588.2 |####
   1854.5 |
   2120.7 |########################################
   2387.0 |##
   2653.3 |
   2919.6 |
   3185.8 |
   3452.1 |
   3718.4 |
   3984.6 |####
   4250.9 |##
   4517.2 |
   4783.5 |
   5049.7 |
   5316.0 |
   5582.3 |
   5848.6 |
   6114.8 |####
   6381.1 |
   6647.4 |##
  (6 below, 5 above range)

wide-rung-ragged-overread (n=40, range 1448.6-6129.7 ns)
   1448.6 |########################################
   1682.6 |#################
   1916.7 |##
   2150.8 |##
   2384.8 |
   2618.9 |##
   2852.9 |
   3087.0 |
   3321.0 |
   3555.1 |
   3789.2 |
   4023.2 |
   4257.3 |##
   4491.3 |
   4725.4 |
   4959.4 |##
   5193.5 |#####
   5427.6 |
   5661.6 |#####
   5895.7 |
  (7 below, 4 above range)

wide-rung-wordround (n=40, range 1532.9-5703.1 ns)
   1532.9 |##
   1741.4 |########################################
   1949.9 |
   2158.4 |
   2367.0 |##
   2575.5 |
   2784.0 |
   2992.5 |
   3201.0 |
   3409.5 |
   3618.0 |
   3826.5 |
   4035.0 |##
   4243.6 |##
   4452.1 |#######
   4660.6 |##########
   4869.1 |##
   5077.6 |##
   5286.1 |
   5494.6 |##
  (7 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1698.3-6396.9 ns)
   1698.3 |########################################
   1933.2 |
   2168.2 |
   2403.1 |##
   2638.0 |
   2873.0 |
   3107.9 |
   3342.8 |
   3577.8 |##
   3812.7 |##
   4047.6 |####
   4282.6 |####
   4517.5 |##
   4752.4 |######
   4987.4 |
   5222.3 |
   5457.2 |
   5692.2 |
   5927.1 |##
   6162.0 |####
  (2 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=55.4% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wide-rung-ragged**: CV=61.8% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=68.5% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **wide-rung-wordround**: CV=56.6% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=57.1% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.8x (possible bimodal distribution)
